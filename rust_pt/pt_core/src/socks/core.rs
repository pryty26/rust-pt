// File: rust_pt\pt_core\src\socks\core.rs
// Directory: rust_pt\pt_core\src\socks
// Filename: core.rs
//======================================================================

//! RFC 1928 (https://www.rfc-editor.org/info/rfc1928/)
use crate::variables::NO_ACCEPTABLE_METHODS;
use derive_deftly::Deftly;
use pt_config::{
    derive_deftly_template_Builder, derive_deftly_template_FromDiscriminant,
    derive_deftly_template_IntoU8,
};
use pt_err::SocksError;
use std::io::ErrorKind;
use std::net::{IpAddr, Ipv4Addr};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

/// Different version of Socks protocol
/// # Note
/// Currently, we only support `Socks5`
#[derive(Clone, Copy, PartialEq, Deftly)]
#[derive_deftly(FromDiscriminant, IntoU8)]
#[allow(clippy::exhaustive_enums)]
pub enum SocksVer {
    /// Socks5
    V5 = 0x05,
}

/// An socks connection and it's settings
/// Please only call `builder()`, user must not set anything
///
///
/// # Examples
///
/// ```rust,no_run
/// use pt_core::socks::core::SocksConn;
/// use anyhow::Result;
/// use tokio::net::TcpListener;
/// #[tokio::main]
/// async fn main() -> Result<()> {
///     let listener = TcpListener::bind("127.0.0.1:9050").await?;
///     let (mut client, _) = listener.accept().await?;
///     let mut conn = SocksConn::builder();
///     conn.handle_connect(&mut client).await?;
///     Ok(())
/// }
///
/// ```
///
/// ```rust,no_run
/// use pt_core::socks::core::SocksConn;
/// use tokio::net::TcpStream;
/// use anyhow::Result;
/// use tokio::io::AsyncWriteExt;
/// #[tokio::main]
/// async fn main() -> Result<()> {
///     
///     let mut stream = TcpStream::connect("127.0.0.1:9050").await?;
///     let mut conn = SocksConn::builder();
///     let (bnd_addr, bnd_port) = conn.connect(
///         &mut stream,
///         "example.com".to_string(),
///         443,
///     ).await?;
///     // And you will get raw stream which has already done the handshake :)
///     stream.write_all(b"GET / HTTP/1.0\r\n\r\n").await?;
///     Ok(())
/// }
///
/// ```
#[derive(Deftly)]
#[non_exhaustive]
#[derive_deftly(Builder)]
pub struct SocksConn {
    /// The version of the connection
    #[deftly(default = "None")]
    pub version: Option<SocksVer>,
    /// selected auth type
    #[deftly(default = "None")]
    pub auth_type: Option<AuthMethods>,
    /// The ip address that user called to either bind or connect
    #[deftly(default = "None")]
    pub ip: Option<IpAddr>,
    /// The command that user asked us to do
    #[deftly(default = "None")]
    pub command: Option<SocksCommand>,
    /// Address type of the target (IPv4 / IPv6 / domain)
    #[deftly(default = "None")]
    pub addr_type: Option<AddrType>,
    /// Target domain name, set when `addr_type` is [`AddrType::Domain`]
    #[deftly(default = "None")]
    pub domain: Option<String>,
    /// Target port, big-endian on the wire
    #[deftly(default = "None")]
    pub port: Option<u16>,
}

/// Address type
// `0x02` is intentionally absent: RFC 1928 §5 only defines 0x01/0x03/0x04.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deftly)]
#[allow(clippy::exhaustive_enums)]
#[derive_deftly(FromDiscriminant, IntoU8)]
pub enum AddrType {
    /// IPv4 address
    IpV4 = 0x01,
    /// Domain name
    Domain = 0x03,
    /// IPv6 address
    IpV6 = 0x04,
}

/// The supported authentication methods
/// Please check if that matches [`SUPPORTED_AUTH_METHODS`]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deftly)]
#[allow(clippy::exhaustive_enums)]
#[derive_deftly(FromDiscriminant, IntoU8)]
pub enum AuthMethods {
    /// no authentication
    NoAuth = 0x00,
}

/// Supported authentication methods
/// Please check if that matches [`AuthMethods`]
pub const SUPPORTED_AUTH_METHODS: [u8; 1] = [0x00];

/// The supported command
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deftly)]
#[allow(clippy::exhaustive_enums)]
#[derive_deftly(FromDiscriminant, IntoU8)]
pub enum SocksCommand {
    /// Connect to the given address
    Connect = 0x01,
}

/// State for the negotiation
#[non_exhaustive]
pub struct AuthNegotiation {
    /// Version
    pub ver: SocksVer,
    /// How many auth methods is supported
    pub nmethods: u8,
    /// Supported auth methods
    pub methods: Vec<AuthMethods>,
}

/// Server's reply during the authentication negotiation
/// `[ver: u8][method: AuthMethods]`
#[allow(clippy::exhaustive_structs)]
pub struct AuthReply {
    /// version that is selected
    pub ver: u8,
    /// Method that is selected
    pub method: AuthMethods,
}

/// Reply field (`REP`) in a SOCKS5 reply (RFC 1928 §6)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deftly)]
#[allow(clippy::exhaustive_enums)]
#[derive_deftly(FromDiscriminant, IntoU8)]
pub enum ReplyCode {
    /// succeeded
    Succeeded = 0x00,
    /// general SOCKS server failure
    GeneralFailure = 0x01,
    /// connection not allowed by ruleset
    NotAllowed = 0x02,
    /// network unreachable
    NetworkUnreachable = 0x03,
    /// host unreachable
    HostUnreachable = 0x04,
    /// connection refused
    ConnectionRefused = 0x05,
    /// TTL expired
    TtlExpired = 0x06,
    /// command not supported
    CommandNotSupported = 0x07,
    /// address type not supported
    AddrTypeNotSupported = 0x08,
}

impl ReplyCode {
    /// Map a closest SOCKS5 reply code (RFC 1928 §6) to an [`std::io::Error`].
    ///
    /// Returns `None` for [`Self::Succeeded`], since a successful reply is
    /// not an error.
    #[must_use]
    pub fn into_io_error(&self) -> Option<std::io::Error> {
        let kind = match self {
            Self::Succeeded => return None,
            Self::GeneralFailure => ErrorKind::Other,
            Self::NotAllowed => ErrorKind::PermissionDenied,
            Self::NetworkUnreachable => ErrorKind::NetworkUnreachable,
            Self::HostUnreachable => ErrorKind::HostUnreachable,
            Self::ConnectionRefused => ErrorKind::ConnectionRefused,
            Self::TtlExpired => ErrorKind::TimedOut,
            Self::CommandNotSupported | Self::AddrTypeNotSupported => ErrorKind::Unsupported,
        };
        Some(std::io::Error::from(kind))
    }
    /// Map a connect failure to the closest SOCKS5 reply code (RFC 1928 §6).
    ///
    /// The RFC does not prescribe an exact mapping; this is a best-effort guess.
    #[must_use]
    pub fn from_io_error(err: &std::io::Error) -> Self {
        match err.kind() {
            ErrorKind::ConnectionRefused => Self::ConnectionRefused,
            ErrorKind::NetworkUnreachable => Self::NetworkUnreachable,
            ErrorKind::HostUnreachable => Self::HostUnreachable,
            ErrorKind::TimedOut => Self::TtlExpired,
            _ => Self::GeneralFailure,
        }
    }
    /// Pick a reply code for a request-parsing error, or `None` if the
    /// error must not be reported to the client (RFC 1928 §6).
    #[must_use]
    pub fn for_error(err: &SocksError) -> Option<Self> {
        match err {
            SocksError::CommandNotSupported(_) => Some(Self::CommandNotSupported),
            SocksError::AddrTypeNotSupported(_) => Some(Self::AddrTypeNotSupported),
            SocksError::BadReserved(_) => Some(Self::GeneralFailure),
            _ => None,
        }
    }
    /// Map a SOCKS5 reply code (RFC 1928 §6) back to a [`SocksError`].
    ///
    /// This is the (lossy) inverse of [`Self::for_error`].
    #[must_use]
    pub fn into_socks_error(self) -> Option<SocksError> {
        match self {
            Self::Succeeded => None,
            Self::GeneralFailure => Some(SocksError::Other(anyhow::anyhow!(
                "general SOCKS server failure"
            ))),
            Self::NotAllowed => Some(SocksError::Other(anyhow::anyhow!(
                "connection not allowed by ruleset"
            ))),
            Self::NetworkUnreachable => {
                Some(SocksError::Other(anyhow::anyhow!("network unreachable")))
            },
            Self::HostUnreachable => Some(SocksError::Other(anyhow::anyhow!("host unreachable"))),
            Self::ConnectionRefused => {
                Some(SocksError::Other(anyhow::anyhow!("connection refused")))
            },
            Self::TtlExpired => Some(SocksError::Other(anyhow::anyhow!("TTL expired"))),
            Self::CommandNotSupported => Some(SocksError::CommandNotSupported(None)),
            Self::AddrTypeNotSupported => Some(SocksError::AddrTypeNotSupported(None)),
        }
    }
}

// Server implementation
impl SocksConn {
    /// Return `0x05, 0xff` for rejecting auth methods
    async fn reject_auth_methods(stream: &mut TcpStream) -> Result<(), SocksError> {
        stream.write_all(&NO_ACCEPTABLE_METHODS).await?;
        Ok(())
    }

    /// Return auth type negotiation success
    async fn auth_negotiation_success(
        stream: &mut TcpStream,
        auth_type: u8,
    ) -> Result<(), SocksError> {
        stream.write_all(&[0x05, auth_type]).await?;
        Ok(())
    }

    /// Handle the authentication negotiation and set [`SocksConn::auth_type`].
    ///
    /// # Errors
    ///
    /// - [`SocksError::UnsupportedVersion`] if the client's SOCKS version is not 5.
    /// - [`SocksError::InvalidFormMsg`] if the method list doesn't match `NMETHODS`.
    /// - [`SocksError::NoSupportedAuthMethod`] if none of the client's methods are supported.
    pub async fn server_auth_negotiation(
        &mut self,
        stream: &mut TcpStream,
    ) -> Result<(), SocksError> {
        let mut auth_header_buf = [0_u8; 2];
        stream.read_exact(&mut auth_header_buf).await?;

        let ver = SocksVer::from_discriminant(auth_header_buf[0] as usize)
            .map_err(|_| SocksError::UnsupportedVersion)?;
        self.version = Some(ver);

        let len = auth_header_buf[1];
        let mut auth_buf = vec![0_u8; len as usize];
        stream.read_exact(&mut auth_buf).await.map_err(|e| {
            SocksError::InvalidFormMsg(format!(
                "the message doesn't follow the indicated length: {e}"
            ))
        })?;

        for auth_type in auth_buf {
            if let Ok(auth_method) = AuthMethods::from_discriminant(auth_type as usize) {
                let byte = u8::from(auth_method);
                self.auth_type = Some(auth_method);
                Self::auth_negotiation_success(stream, byte).await?;
                return Ok(());
            }
        }

        Self::reject_auth_methods(stream).await?;
        Err(SocksError::NoSupportedAuthMethod)
    }

    /// Validate the version, if the version matches with the previous indicated version
    ///
    /// # Errors
    ///
    /// - [`SocksError::SocksVerUnfound`] if no version has been set yet.
    fn validate_version(&self, version: SocksVer) -> Result<bool, SocksError> {
        if let Some(ver) = self.version {
            return Ok(ver == version);
        }
        Err(SocksError::SocksVerUnfound)
    }
    /// Parse the SOCKS5 request (CMD) message from the client.
    ///
    /// Reads `VER, CMD, RSV, ATYP`, then the address (and port) according
    /// to `ATYP`, and stores the result into [`Self::command`],
    /// [`Self::addr_type`], [`Self::ip`] / [`Self::domain`], and [`Self::port`].
    ///
    /// # Errors
    ///
    /// Errors that can be reported to the client with a SOCKS5 reply:
    ///
    /// - [`SocksError::CommandNotSupported`] — CMD is unknown; reply `0x07`.
    /// - [`SocksError::AddrTypeNotSupported`] — ATYP is unknown; reply `0x08`.
    /// - [`SocksError::BadReserved`] — RSV is not `0x00`; reply `0x01`.
    ///
    /// Errors that must not produce a reply (drop the connection):
    ///
    /// - [`SocksError::InvalidFormMsg`] if the message is truncated or a domain
    ///   name is not valid UTF-8.
    /// - [`SocksError::UnsupportedVersion`] if the client's SOCKS version is not 5.
    /// - [`SocksError::SocksVerUnfound`] if no version has been negotiated yet
    ///   (see [`Self::auth_negotiation`]).
    /// - [`SocksError::SocksVerNoMatching`] if the version in this message does
    ///   not match the previously negotiated one.
    /// - I/O error variant if reading from the stream fails.
    pub async fn parse_cmd(&mut self, stream: &mut TcpStream) -> Result<(), SocksError> {
        let mut header_buf = [0_u8; 4];
        stream
            .read_exact(&mut header_buf)
            .await
            .map_err(|e| SocksError::InvalidFormMsg(format!("Command message too short: {e}")))?;

        // VER — no reply on failure (client version is wrong anyway).
        let ver = SocksVer::from_discriminant(header_buf[0] as usize)
            .map_err(|_| SocksError::UnsupportedVersion)?;
        if !self.validate_version(ver)? {
            return Err(SocksError::SocksVerNoMatching);
        }

        // CMD — reply 0x07 if unsupported.
        let command = SocksCommand::from_discriminant(header_buf[1] as usize)
            .map_err(|_| SocksError::CommandNotSupported(Some(header_buf[1])))?;

        // RSV — reply 0x01 if not 0x00.
        if header_buf[2] != 0x00 {
            return Err(SocksError::BadReserved(Some(header_buf[2])));
        }

        // ATYP — reply 0x08 if unsupported.
        let addr_type = AddrType::from_discriminant(header_buf[3] as usize)
            .map_err(|_| SocksError::AddrTypeNotSupported(Some(header_buf[3])))?;

        match addr_type {
            AddrType::IpV4 => {
                let mut buf = [0_u8; 4];
                stream.read_exact(&mut buf).await?;
                self.ip = Some(IpAddr::from(buf));
            },
            AddrType::IpV6 => {
                let mut buf = [0_u8; 16];
                stream.read_exact(&mut buf).await?;
                self.ip = Some(IpAddr::from(buf));
            },
            AddrType::Domain => {
                let mut len_buf = [0_u8; 1];
                stream.read_exact(&mut len_buf).await?;
                let len = len_buf[0] as usize;
                if len == 0 {
                    return Err(SocksError::InvalidFormMsg("empty domain name".into()));
                }
                let mut name_buf = vec![0_u8; len];
                stream.read_exact(&mut name_buf).await?;
                let domain = String::from_utf8(name_buf).map_err(|e| {
                    SocksError::InvalidFormMsg(format!("domain is not valid UTF-8: {e}"))
                })?;
                self.domain = Some(domain);
            },
        }

        let mut port_buf = [0_u8; 2];
        stream.read_exact(&mut port_buf).await?;

        self.command = Some(command);
        self.addr_type = Some(addr_type);
        self.port = Some(u16::from_be_bytes(port_buf));

        Ok(())
    }
    /// Perform the full SOCKS5 handshake in one call.
    ///
    /// Runs [`Self::server_auth_negotiation`] followed by [`Self::parse_cmd`], so the
    /// connection is ready to relay after this returns.
    ///
    /// # Errors
    ///
    /// - Whatever [`Self::server_auth_negotiation`] can return.
    /// - Whatever [`Self::parse_cmd`] can return.
    pub async fn server_handshake(&mut self, stream: &mut TcpStream) -> Result<(), SocksError> {
        self.server_auth_negotiation(stream).await?;
        self.parse_cmd(stream).await?;
        Ok(())
    }

    /// Send a SOCKS5 reply to the client (RFC 1928 §6).
    ///
    /// `bnd_addr` / `bnd_port` are the server-bound address and port. On
    /// failure replies, callers typically pass
    /// `IpAddr::V4(Ipv4Addr::UNSPECIFIED)` and `0`.
    ///
    /// # Errors
    ///
    /// - I/O error variant if writing to `stream` fails.
    pub async fn reply(
        &self,
        stream: &mut TcpStream,
        rep: ReplyCode,
        bnd_addr: IpAddr,
        bnd_port: u16,
    ) -> Result<(), SocksError> {
        let (atyp, addr_bytes): (AddrType, Vec<u8>) = match bnd_addr {
            IpAddr::V4(v4) => (AddrType::IpV4, v4.octets().to_vec()),
            IpAddr::V6(v6) => (AddrType::IpV6, v6.octets().to_vec()),
        };

        let mut buf = Vec::with_capacity(4 + addr_bytes.len() + 2);
        buf.push(u8::from(SocksVer::V5));
        buf.push(u8::from(rep));
        buf.push(0x00);
        buf.push(u8::from(atyp));
        buf.extend_from_slice(&addr_bytes);
        buf.extend_from_slice(&bnd_port.to_be_bytes());

        stream.write_all(&buf).await?;
        Ok(())
    }

    /// Handle a CONNECT request end to end (RFC 1928 §4, §6).
    ///
    /// We will run [`Self::server_handshake`] in this function
    ///
    /// After that, we will open a bidirectional data forwarding channel
    ///
    /// # Errors
    ///
    /// - [`SocksError::InvalidFormMsg`] if the target address is missing.
    /// - I/O error variant if dialing upstream or writing the reply fails.
    /// - Whatever [`relay`] returns.
    pub async fn handle_connect(&mut self, client: &mut TcpStream) -> Result<(), SocksError> {
        if let Err(e) = self.server_handshake(client).await {
            if let Some(rep) = ReplyCode::for_error(&e) {
                self.reply(client, rep, IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0)
                    .await?;
            }
            return Err(e);
        }
        let port = self
            .port
            .ok_or_else(|| SocksError::PoisonedSetting("missing DST.PORT".into()))?;

        let target: (String, u16) = match self.addr_type {
            Some(AddrType::IpV4 | AddrType::IpV6) => {
                let ip = self
                    .ip
                    .ok_or_else(|| SocksError::PoisonedSetting("missing DST.ADDR".into()))?;
                (ip.to_string(), port)
            },
            Some(AddrType::Domain) => {
                let d = self
                    .domain
                    .as_ref()
                    .ok_or_else(|| SocksError::PoisonedSetting("missing DST.ADDR".into()))?;
                (d.clone(), port)
            },
            None => return Err(SocksError::PoisonedSetting("missing ATYP".into())),
        };
        // Note:
        // `connect<A: ToSocketAddrs>(addr: A)`
        // `addr` is an address of the remote host.
        // Anything which implements the
        // [`ToSocketAddrs`] trait can be supplied as the address.
        match TcpStream::connect((target.0.as_str(), target.1)).await {
            Ok(mut upstream) => {
                let local = upstream.local_addr()?;
                self.reply(client, ReplyCode::Succeeded, local.ip(), local.port())
                    .await?;
                tokio::io::copy_bidirectional(client, &mut upstream).await?;
            },
            Err(e) => {
                let rep = ReplyCode::from_io_error(&e);
                self.reply(client, rep, IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0)
                    .await?;
                return Err(e.into());
            },
        }

        Ok(())
    }
}

// Client implementation
impl SocksConn {
    /// client authentication negotiation
    ///
    /// # Errors
    ///
    /// - `InvalidFormMsg` — server replied with a version other than `0x05`.
    /// - `UnsupportedVersion` — server selected a method not in `SUPPORTED_AUTH_METHODS`.
    /// - I/O error variant — read/write failed
    /// # Panics
    /// if the [`SUPPORTED_AUTH_METHODS`] len is bigger than 255,
    /// it is a huge problem, which means the original code has a crusial issue.
    /// Therefore, we will panic here
    pub async fn client_auth_negotiation(
        &mut self,
        stream: &mut TcpStream,
    ) -> Result<(), SocksError> {
        let mut buf = Vec::with_capacity(2 + SUPPORTED_AUTH_METHODS.len());
        buf.push(u8::from(SocksVer::V5));
        self.version = Some(SocksVer::V5);

        buf.push(
            SUPPORTED_AUTH_METHODS
                .len()
                .try_into()
                .expect("The SUPPORTED_AUTH_METHODS.len() is bigger than 255"),
        );
        buf.extend_from_slice(&SUPPORTED_AUTH_METHODS);
        stream.write_all(&buf).await?;

        // Receive the reply
        let mut reply_buf = [0_u8; 2];
        stream.read_exact(&mut reply_buf).await?;
        if reply_buf[0] != 0x05 {
            return Err(SocksError::InvalidFormMsg(
                "The version should be 0x05".to_string(),
            ));
        }
        if !SUPPORTED_AUTH_METHODS.contains(&reply_buf[1]) {
            return Err(SocksError::NoSupportedAuthMethod);
        }
        self.auth_type = Some(AuthMethods::from_discriminant(reply_buf[1].into())?);
        Ok(())
    }
    /// Sends a SOCKS5 CONNECT request and reads the reply
    ///
    /// `addr` is sent as IPv4/IPv6 if it parses as an IP, otherwise as a domain.
    /// On success, returns the reply's `BND.ADDR` / `BND.PORT`.
    ///
    /// # Errors
    ///
    /// - `InvalidFormMsg` — bad version, unknown ATYP, or non-UTF-8 domain
    /// - `BadReserved` — reply RSV != 0
    /// - `SocksError` from `into_socks_error` — non-zero REP
    /// - I/O error variant — read/write failed
    pub async fn send_cmd(
        &mut self,
        stream: &mut TcpStream,
        addr: String,
        port: u16,
    ) -> Result<(String, u16), SocksError> {
        let mut msg = match addr.parse::<IpAddr>() {
            Ok(IpAddr::V4(v4)) => {
                self.addr_type = Some(AddrType::IpV4);
                let mut req = Vec::with_capacity(1 + 1 + 1 + 1 + 4 + 2);
                req.extend_from_slice(&[0x05, 0x01, 0x00, 0x01]);
                req.extend_from_slice(&v4.octets());
                req
            },
            Ok(IpAddr::V6(v6)) => {
                self.addr_type = Some(AddrType::IpV6);
                let mut req = Vec::with_capacity(1 + 1 + 1 + 1 + 16 + 2);
                req.extend_from_slice(&[0x05, 0x01, 0x00, 0x04]);
                req.extend_from_slice(&v6.octets());
                req
            },
            Err(_) => {
                self.addr_type = Some(AddrType::Domain);
                let mut req = Vec::with_capacity(1 + 1 + 1 + 1 + 1 + addr.len() + 2);
                req.extend_from_slice(&[0x05, 0x01, 0x00, 0x03]);
                req.push(
                    u8::try_from(addr.len())
                        .map_err(|_| SocksError::InvalidFormMsg("address too long".to_string()))?,
                );
                req.extend_from_slice(addr.as_bytes());
                req
            },
        };
        msg.extend_from_slice(&port.to_be_bytes());
        stream.write_all(&msg).await?;
        let mut reply_buf = [0_u8; 4];
        stream.read_exact(&mut reply_buf).await?;
        if reply_buf[0] != 0x05 {
            return Err(SocksError::InvalidFormMsg(
                "The version should be 0x05".to_string(),
            ));
        }
        if let Some(e) =
            ReplyCode::into_socks_error(ReplyCode::from_discriminant(reply_buf[1].into())?)
        {
            return Err(e);
        }

        if reply_buf[2] != 0x00 {
            return Err(SocksError::BadReserved(Some(reply_buf[2])));
        }

        let addr = match AddrType::from_discriminant(reply_buf[3].into())? {
            AddrType::Domain => {
                let mut len = [0_u8; 1];
                stream.read_exact(&mut len).await?;
                let mut addr_buf = vec![0_u8; len[0].into()];
                stream.read_exact(&mut addr_buf).await?;
                String::from_utf8(addr_buf).map_err(|e| {
                    SocksError::InvalidFormMsg(format!("domain is not valid UTF-8: {e}"))
                })?
            },
            AddrType::IpV4 => {
                let mut buf = [0_u8; 4];
                stream.read_exact(&mut buf).await?;
                IpAddr::from(buf).to_string()
            },
            AddrType::IpV6 => {
                let mut buf = [0_u8; 16];
                stream.read_exact(&mut buf).await?;
                IpAddr::from(buf).to_string()
            },
        };
        let mut port_buf = [0_u8; 2];
        stream.read_exact(&mut port_buf).await?;
        let port = u16::from_be_bytes(port_buf);
        Ok((addr, port))
    }
    /// Performs the full SOCKS5 client handshake
    ///
    /// Runs authentication negotiation followed by `CONNECT`. On success,
    /// returns the server's bound address and port from the reply.
    ///
    /// # Errors
    ///
    /// - Whatever [`Self::client_auth_negotiation`] returns.
    /// - Whatever [`Self::send_cmd`] returns.
    pub async fn connect(
        &mut self,
        stream: &mut TcpStream,
        addr: String,
        port: u16,
    ) -> Result<(String, u16), SocksError> {
        self.client_auth_negotiation(stream).await?;
        self.send_cmd(stream, addr, port).await
    }
}
