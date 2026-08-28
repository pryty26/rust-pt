use crate::variables::StaticHeader;
use anyhow::{Result};
use derive_deftly::Deftly;
use hmac::{Hmac, KeyInit, Mac};
use pt_config::derive_deftly_template_FromDiscriminant;
use pt_err::ExtOrPortError;
use sha2::Sha256;
use std::net::SocketAddr;
use std::path::PathBuf;
use tokio::fs;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
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
pub enum ExtOrPortState {
    /// Negetiating the auth type
    AuthTypesNegotiation,
    // TODO: Add else
}
/// Config for ExtOrPort
pub struct ExtOrPort {
    /// Addr of ExtOrPort
    pub addr: SocketAddr,
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
    pub auth_cookie_file: PathBuf,
    /// Different state for Pt ExtOrPort
    pub state: ExtOrPortState,
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
#[derive_deftly(FromDiscriminant)]
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
    /// Auth types negotiation
    pub fn auth_types_neg(&self, buf: AuthNegBuf) -> Result<AuthTypes, ExtOrPortError> {
        // supported auth types
        let mut sup_buf = Vec::<AuthTypes>::new();
        let mut end_found = false;
        for i in 0..buf.len() {
            let auth_type: Option<AuthTypes> = AuthTypes::from_discriminant(buf[i].into()).ok();
            match auth_type {
                Some(auth_type) if auth_type != AuthTypes::EndAuthType => {
                    sup_buf.push(auth_type);
                },
                Some(auth_type) if auth_type == AuthTypes::EndAuthType => {
                    end_found = true;
                    break;
                },
                Some(_) => {
                    unreachable!(
                        "unreachable pattern how can a pattern be both != and == EndAuthType"
                    )
                },
                None => {
                    // Nothing to do here, we do not support that AuthTypes
                    // so just continue
                    continue
                },
            }
        }
        match end_found {
            false => {
                return Err(ExtOrPortError::EndAuthTypeUnfound);
            },
            true => {}
        }
        if sup_buf.is_empty() {
            return Err(ExtOrPortError::UnsupportedAuthTypes);
        }
        // Let's just return the first supported auth type
        Ok(sup_buf[0])
    }
    /// Establish a ExtOrPort connection
    pub async fn connect(&self) -> Result<()> {
        let cookie_string: CookieString = self.auth_cookie().await?;
        let mut connection = TcpStream::connect(self.addr).await?;
            let (mut reader, mut writer) = connection.split();
        // When a client (that is to say, a server-side pluggable transport) connects to an Extended ORPort, the server sends:
        // AuthTypes                                   [variable]
        // EndAuthTypes                                [1 octet]
        loop {
            let mut buf: AuthNegBuf = [0; 256];
            let msg = reader.read(&mut buf).await?;
            if msg == 0 {
                todo!("we should return diffrent message depends on the state")
            }
            match self.state {
                ExtOrPortState::AuthTypesNegotiation => {
                    let auth_type = self.auth_types_neg(buf)?;
                    writer.write_all(&[auth_type as u8]).await?;
                },
                // TODO: else
            }
        }
        Ok(())
    }
}
