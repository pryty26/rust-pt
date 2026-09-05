// File: rust_pt\pt_core\src\orport\extorport.rs
// Directory: rust_pt\pt_core\src\orport
// Filename: extorport.rs
//======================================================================

use crate::variables::StaticHeader;
use anyhow::{Result, anyhow};
use derive_deftly::Deftly;
use getrandom::fill;
use hmac::{Hmac, KeyInit, Mac};
use pt_config::{
    derive_deftly_template_Builder, derive_deftly_template_FromDiscriminant,
    derive_deftly_template_FromString,
};
use pt_err::ExtOrPortError;
use pt_tracing::rec_panic;
use sha2::Sha256;
use std::net::SocketAddr;
use std::path::PathBuf;
use tokio::fs;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
///! See: (https://spec.torproject.org/ext-orport-spec.html)
use tokio::net::tcp::{ReadHalf, WriteHalf};
/// CookieString
pub type CookieString = [u8; 32];
/// Buffer for AuthTypes negotiation (null-terminated u8 list).
/// Valid auth types: 1..=255, sentinel terminator: 0.
/// Maximum size: 255 values + 1 terminator = 256 bytes.
pub type AuthNegBuf = [u8; 256];

/// Build a HMAC message
pub fn mac_message(key: &[u8], message: &[u8]) -> Result<[u8; 32]> {
    let mut mac = Hmac::<Sha256>::new_from_slice(key)?;
    mac.update(message);
    Ok(mac.finalize().into_bytes().into())
}
/// Different state for Pt ExtOrPort
#[derive(Copy, Clone, Deftly, PartialEq, Eq)]
#[derive_deftly(FromDiscriminant, FromString)]
pub enum ExtOrPortState {
    /// Negetiating the auth type
    AuthTypesNegotiation = 0,
    /// SafeCookie authenticating
    SafeCookieAuthentication = 1,
    // TODO: Add else
}
/// Different state for Safe Cookie Authentication
#[derive(Copy, Clone, Deftly, PartialEq, Eq)]
#[derive_deftly(FromDiscriminant, FromString)]
pub enum SafeCookieState {
    /// Send Client Nonce
    SendClientNonce = 0,
    /// Recerve server nonce
    RecvServerNonce = 1,
    /// Send Client Hash
    SendClientHash = 2,
    /// Receive the final server Response after sending client hash
    RecvFinalResp = 3,
}
///
#[derive(Clone, Deftly, PartialEq, Eq)]
pub struct ExtOrPortConnection {}

/// Config for ExtOrPort
#[derive(Clone, Deftly, PartialEq, Eq)]
#[derive_deftly(Builder)]
pub struct ExtOrPort {
    /// Addr of ExtOrPort
    #[deftly(default = "\"127.0.0.1:8080\".parse::<SocketAddr>().unwrap()")]
    addr: SocketAddr,
    /// We define one authentication type: SAFE_COOKIE.
    /// Its AuthType value is 1.
    /// It is based on the client proving to the bridge that it can access a given “cookie” file on disk.
    /// The purpose of authentication is to defend against cross-protocol attacks.
    ///
    /// If the Extended ORPort is enabled, Tor should regenerate the cookie file on startup
    /// and store it in $DataDirectory/extended_orport_auth_cookie.
    ///
    /// The location of the cookie can be overridden by using the configuration file parameter
    /// ExtORPortCookieAuthFile, which is defined as:
    /// ```text
    /// ExtORPortCookieAuthFile <path>
    /// ```
    /// where <path> is a filesystem path.
    #[deftly(default = "\"./rust\"")]
    auth_cookie_file: PathBuf,
    /// Different state for Pt ExtOrPort
    #[deftly(default = "ExtOrPortState::AuthTypesNegotiation")]
    state: ExtOrPortState,
}

/// Diffrent auth types
/// ```text
/// When a client (that is to say, a server-side pluggable transport) connects to an Extended ORPort,
///  the server sends:
///
///     AuthTypes                                   [variable]
///     EndAuthTypes                                [1 octet]
///
///   Where,
///
///   + AuthTypes are the authentication schemes that the server supports
///     for this session. They are multiple concatenated 1-octet values that
///     take values from 1 to 255.
///   + EndAuthTypes is the special value 0.
///
/// The client reads the list of supported authentication schemes,
/// chooses one,
/// and sends it back:
///
/// AuthType [1 octet]
///
/// Where,
///
///   + AuthType is the authentication scheme that the client wants to use
///     for this session. A valid authentication type takes values from 1 to
///     255. A value of 0 means that the client did not like the
///     authentication types offered by the server.
///
/// If the client sent an AuthType of value 0, or an AuthType that the server does not support,
/// the server MUST close the connection.
/// ```
#[repr(u8)]
#[derive(Deftly, PartialEq, Eq, Copy, Clone)]
#[derive_deftly(FromDiscriminant, FromString)]
pub enum AuthTypes {
    /// EndAuthType
    /// It is not a AuthType
    EndAuthType = 0,
    /// We define one authentication type: SAFE_COOKIE.
    /// Its AuthType value is 1.
    SafeCookie = 1,
}
impl ExtOrPort {
    /// ```text
    /// Cookie-file format
    ///
    /// The format of the cookie-file is:
    ///
    ///      StaticHeader                                [32 octets]
    ///      Cookie                                      [32 octets]
    ///
    ///   Where,
    ///   + StaticHeader is the following string:
    ///     "! Extended ORPort Auth Cookie !\x0a"
    ///   + Cookie is the shared-secret. During the SAFE_COOKIE protocol, the
    ///     cookie is called CookieString.
    /// Extended ORPort clients MUST make sure that the StaticHeader is present in the cookie file,
    /// before proceeding with the authentication protocol.
    /// ```
    pub async fn auth_cookie(&self) -> Result<CookieString> {
        let contents = fs::read(self.auth_cookie_file.clone()).await?;
        let static_cookie: [u8; 32] = contents[0..32].try_into()?;
        // If static_cookie is not equal to "! Extended ORPort Auth Cookie !\x0a"
        // we must raise an Error
        if static_cookie != StaticHeader {
            return Err(ExtOrPortError::InvalidStaticHeader.into());
        }
        let cookie_string: CookieString = contents[32..64].try_into()?;
        Ok(cookie_string)
    }
    /// Build ClientHash
    /// Where:
    /// ```text
    ///
    /// + ClientHash is computed as:
    ///     HMAC-SHA256(CookieString,
    ///      "ExtORPort authentication client-to-server hash" | ClientNonce | ServerNonce)
    /// ```
    pub fn client_hash(
        cookie: &[u8],
        client_nonce: &[u8],
        server_nonce: &[u8],
    ) -> Result<[u8; 32]> {
        let label = b"ExtORPort authentication client-to-server hash";

        let mut message = [0u8; 110];
        message[0..46].copy_from_slice(label);
        message[46..78].copy_from_slice(client_nonce);
        message[78..110].copy_from_slice(server_nonce);
        Ok(mac_message(cookie, &message)?)
    }
    /// Build ServerHash
    /// ```test
    ///
    /// + ServerHash is computed as:
    ///     HMAC-SHA256(CookieString,
    ///       "ExtORPort authentication server-to-client hash" | ClientNonce | ServerNonce)
    /// ```
    pub fn server_hash(
        cookie: &[u8],
        client_nonce: &[u8],
        server_nonce: &[u8],
    ) -> Result<[u8; 32]> {
        let label = b"ExtORPort authentication server-to-client hash";
        let mut message = [0u8; 110];
        message[0..46].copy_from_slice(label);
        message[46..78].copy_from_slice(client_nonce);
        message[78..110].copy_from_slice(server_nonce);
        Ok(mac_message(cookie, &message)?)
    }
    /// Check if the sender closes the connection, if so,
    /// then we should panic
    fn check_msg(msg: usize) {
        if msg == 0 {
            // The sender closed connection and we should panic,
            // because if we can not establish a connection,
            // it means we can not send data to the tor server
            // therefore, we must panic to tell the user that we have not estalbished connection
            rec_panic!("The sender closed connection in ExtOrPort");
        }
    }
    /// safe cookie authentication
    async fn safe_cookie_authentication(
        &mut self,
        writer: &mut WriteHalf<'_>,
        reader: &mut ReadHalf<'_>,
    ) -> Result<(), ExtOrPortError> {
        let cookie_string: CookieString = self.auth_cookie().await?;
        let mut server_nonce_buf: [u8; 32] = [0; 32];
        let client_nonce = {
            let mut buf: [u8; 32] = [0; 32];
            fill(&mut buf).map_err(|e| anyhow!(e))?;
            buf
        };
        writer
            .write_all(&client_nonce)
            .await
            .map_err(|e| anyhow!(e))?;
        let mut auth_state = SafeCookieState::RecvServerNonce;
        'l: loop {
            match auth_state {
                SafeCookieState::SendClientNonce => {
                    unreachable!("SendClientNonce should be changed before that!")
                },
                SafeCookieState::RecvServerNonce => {
                    //
                    // Then, the server replies with:
                    //
                    //      ServerHash                                  [32 octets]
                    //      ServerNonce                                 [32 octets]
                    //
                    //   Where,
                    //   + ServerHash is computed as:
                    //       HMAC-SHA256(CookieString,
                    //         "ExtORPort authentication server-to-client hash" | ClientNonce | ServerNonce)
                    //   + ServerNonce is 32 random octets.
                    let mut buf = [0; 64];
                    Self::check_msg(reader.read_exact(&mut buf).await.map_err(|e| anyhow!(e))?);
                    let server_nonce: [u8; 32] = buf[32..64].try_into().map_err(|_| {
                        ExtOrPortError::InvalidServerMsg("Invalid ServerNonce".to_string())
                    })?;
                    let expected_serv_hash =
                        Self::server_hash(&cookie_string, &client_nonce, &server_nonce)?;
                    let server_hash: [u8; 32] = buf[0..32].try_into().map_err(|_| {
                        ExtOrPortError::InvalidServerMsg("Invalid ServerHash".to_string())
                    })?;
                    match server_hash {
                        _ if server_hash == expected_serv_hash => {
                            server_nonce_buf = server_nonce;
                            auth_state = SafeCookieState::SendClientHash;
                            continue 'l;
                        },
                        _ => {
                            return Err(ExtOrPortError::InvalidServerHash);
                        },
                    }
                },
                SafeCookieState::SendClientHash => {
                    // ClientHash                                  [32 octets]
                    //
                    // Where,
                    // + ClientHash is computed as:
                    //     HMAC-SHA256(CookieString,
                    //       "ExtORPort authentication client-to-server hash" | ClientNonce | ServerNonce)
                    writer
                        .write_all(&Self::client_hash(
                            &cookie_string,
                            &client_nonce,
                            &server_nonce_buf,
                        )?)
                        .await?;
                    auth_state = SafeCookieState::RecvFinalResp;
                },
                SafeCookieState::RecvFinalResp => {
                    let mut buf = [0; 1];
                    Self::check_msg(reader.read_exact(&mut buf).await?);
                    match buf {
                        // Status   [1 octet]
                        //
                        // Where,
                        // + Status is 1 if the authentication was successful.
                        // If the   authentication failed, Status is 0.
                        [1] => {
                            // anthentication was successful
                            // break and the function will return Ok(())
                            break;
                        },
                        _ => {
                            // Regarding the spec, server should terminate the connection
                            return Err(ExtOrPortError::InvalidClientHash);
                        },
                    }
                },
            }
        }
        Ok(())
    }
    /// Auth types negotiation
    /// len should be the output of reader.read(&mut buf).await?;
    fn auth_types_neg(&self, buf: AuthNegBuf, len: usize) -> Result<AuthTypes, ExtOrPortError> {
        // supported auth types
        let mut sup_buf = Vec::<AuthTypes>::new();
        let mut end_found = false;
        for i in 0..len {
            let auth_type: Option<AuthTypes> = AuthTypes::from_discriminant(buf[i].into()).ok();
            match auth_type {
                Some(auth_type) if auth_type == AuthTypes::EndAuthType => {
                    end_found = true;
                    break;
                },
                Some(auth_type) => {
                    // We have already made sure that auth_type is not AuthTypes::EndAuthType
                    sup_buf.push(auth_type);
                },
                None => {
                    // Nothing to do here, we do not support that AuthTypes
                    // so just continue
                    continue;
                },
            }
        }
        match end_found {
            false => {
                return Err(ExtOrPortError::EndAuthTypeUnfound);
            },
            true => {},
        }
        if sup_buf.is_empty() {
            return Err(ExtOrPortError::UnsupportedAuthTypes);
        }
        // Let's just return the first supported auth type
        Ok(sup_buf[0])
    }
    /// Establish a ExtOrPort connection
    pub async fn connect(&mut self) -> Result<TcpStream, ExtOrPortError> {
        let mut connection = TcpStream::connect(self.addr).await?;
        let (mut reader, mut writer) = connection.split();
        // When a client (that is to say, a server-side pluggable transport) connects to an Extended ORPort, the server sends:
        // AuthTypes                                   [variable]
        // EndAuthTypes                                [1 octet]
        'l: loop {
            let mut buf: AuthNegBuf = [0; 256];
            let msg = reader.read(&mut buf).await?;
            if msg == 0 {
                match self.state {
                    _ => {
                        // The sender closed connection and we should panic,
                        // because if we can not establish a connection,
                        // it means we can not send data to the tor server
                        // therefore, we must panic to tell the user that we have not estalbished connection
                        rec_panic!("The sender closed connection in ExtOrPort");
                    },
                }
            }
            match self.state {
                ExtOrPortState::AuthTypesNegotiation => {
                    match self.auth_types_neg(buf, msg) {
                        Ok(auth_type) => {
                            self.state = ExtOrPortState::SafeCookieAuthentication;
                            writer.write_all(&[auth_type as u8]).await?;
                            continue 'l;
                        },
                        Err(ExtOrPortError::EndAuthTypeUnfound) => {
                            // Maybe user repeats some auth types
                            // So we should try to collect again
                            continue 'l;
                        },
                        Err(ExtOrPortError::UnsupportedAuthTypes) => {
                            writer.write_all(&[0]).await?;
                            // Server will terminate the connection.
                            // If we cannot establish connection we cannot forward traffics
                            // Let's just panic
                            rec_panic!("UnsupportedAuthTypes");
                        },
                        Err(e) => return Err(e.into()),
                    }
                },
                ExtOrPortState::SafeCookieAuthentication => {
                    self.safe_cookie_authentication(&mut writer, &mut reader)
                        .await?;
                    break 'l;
                },
            }
        }
        Ok(connection)
    }
}
