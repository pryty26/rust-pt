#![allow(non_snake_case)]
use crate::MODE;
use anyhow::Result;
use pt_err::ConfigError;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::net::{Ipv4Addr, Ipv6Addr};
use std::path::PathBuf;
/// RawCommonKey is used to validate that the config is valid
/// #[allow(unused)] is fine in this situation,
/// since this is only to validate the config
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub(crate) struct RawCommonKey {
    TOR_PT_MANAGED_TRANSPORT_VER: String,
    TOR_PT_STATE_LOCATION: PathBuf,
    TOR_PT_EXIT_ON_STDIN_CLOSE: i8,
    TOR_PT_OUTBOUND_BIND_ADDRESS_V4: Option<Ipv4Addr>,
    TOR_PT_OUTBOUND_BIND_ADDRESS_V6: Option<Ipv6Addr>,
}

/// Splits `s` by commas and collects the results into a `Vec<String>`.
pub fn separate_with(s: String, comma: &str) -> Vec<String> {
    s.split(comma)
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(String::from)
        .collect()
}
impl TryFrom<RawCommonKey> for CommonKey {
    type Error = ConfigError;
    /// this is for validating whether the config is valid
    fn try_from(raw: RawCommonKey) -> Result<Self, Self::Error> {
        match raw.TOR_PT_EXIT_ON_STDIN_CLOSE {
            0 | 1 => {}
            _ => {
                return Err(ConfigError::InvalidConfigErr {
                    side: (*MODE).into(),
                    message: "TOR_PT_EXIT_ON_STDIN_CLOSE should be 1 or 0".to_string(),
                });
            }
        }
        let TOR_PT_OUTBOUND_BIND_ADDRESS_V4 = match raw.TOR_PT_OUTBOUND_BIND_ADDRESS_V4 {
            Some(x) if !x.is_loopback() => x,
            _ => "0.0.0.0"
                .parse::<Ipv4Addr>()
                .map_err(|e| ConfigError::InvalidConfigErr {
                    side: (*MODE).into(),
                    message: format!("{}", e),
                })?,
        };
        let TOR_PT_OUTBOUND_BIND_ADDRESS_V6 = match raw.TOR_PT_OUTBOUND_BIND_ADDRESS_V6 {
            Some(x) if !x.is_loopback() => x,
            _ => "::1"
                .parse::<Ipv6Addr>()
                .map_err(|e| ConfigError::InvalidConfigErr {
                    side: (*MODE).into(),
                    message: format!("{}", e),
                })?,
        };
        let versions = separate_with(raw.TOR_PT_MANAGED_TRANSPORT_VER, ",");
        Ok(CommonKey {
            TOR_PT_MANAGED_TRANSPORT_VER: versions,
            TOR_PT_STATE_LOCATION: raw.TOR_PT_STATE_LOCATION,
            TOR_PT_EXIT_ON_STDIN_CLOSE: raw.TOR_PT_EXIT_ON_STDIN_CLOSE,
            TOR_PT_OUTBOUND_BIND_ADDRESS_V4: TOR_PT_OUTBOUND_BIND_ADDRESS_V4,
            TOR_PT_OUTBOUND_BIND_ADDRESS_V6: TOR_PT_OUTBOUND_BIND_ADDRESS_V6,
        })
    }
}

/// Common settings required by both client-side and server-side
/// Pluggable Transport proxies.
#[derive(Debug, Serialize, Deserialize)]
#[serde(try_from = "RawCommonKey")]
pub struct CommonKey {
    /// Comma-separated list of Pluggable Transport specification versions
    /// supported by the parent process.
    TOR_PT_MANAGED_TRANSPORT_VER: Vec<String>,

    /// Directory where the Pluggable Transport may persist state.
    TOR_PT_STATE_LOCATION: PathBuf,

    /// Whether the PT should gracefully exit when stdin is closed.
    TOR_PT_EXIT_ON_STDIN_CLOSE: i8,

    /// Optional IPv4 source address for outbound connections.
    ///
    /// If unset, the system default source address is used.
    TOR_PT_OUTBOUND_BIND_ADDRESS_V4: Ipv4Addr,

    /// Optional IPv6 source address for outbound connections.
    ///
    /// If unset, the system default source address is used.
    TOR_PT_OUTBOUND_BIND_ADDRESS_V6: Ipv6Addr,
}

/// Settings which is needed at client side
#[derive(Debug, Serialize, Deserialize)]
// TODO: #[serde(try_from = "?")]
pub struct ClientKey {
    TOR_PT_CLIENT_TRANSPORTS: Vec<String>,
}

/*
use std::ffi::OsString;
use std::fmt;

pub(crate) const TRANSPORT_NAME: &str = "proteus";
pub(crate) const SUPPORTED_PT_VERSION: &str = "1";
pub(crate) const SUPPORTED_SOCKS_VERSION: &str = "socks5";

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub(crate) enum CommonKey {
    TOR_PT_MANAGED_TRANSPORT_VER,
    TOR_PT_STATE_LOCATION,
    TOR_PT_EXIT_ON_STDIN_CLOSE,
    TOR_PT_OUTBOUND_BIND_ADDRESS_V4,
    TOR_PT_OUTBOUND_BIND_ADDRESS_V6,
}

impl CommonKey {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            CommonKey::TOR_PT_MANAGED_TRANSPORT_VER => "TOR_PT_MANAGED_TRANSPORT_VER",
            CommonKey::TOR_PT_STATE_LOCATION => "TOR_PT_STATE_LOCATION",
            CommonKey::TOR_PT_EXIT_ON_STDIN_CLOSE => "TOR_PT_EXIT_ON_STDIN_CLOSE",
            CommonKey::TOR_PT_OUTBOUND_BIND_ADDRESS_V4 => "TOR_PT_OUTBOUND_BIND_ADDRESS_V4",
            CommonKey::TOR_PT_OUTBOUND_BIND_ADDRESS_V6 => "TOR_PT_OUTBOUND_BIND_ADDRESS_V6",
        }
    }
}

impl fmt::Display for CommonKey {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "{:?}", self)
    }
}

impl From<CommonKey> for &str {
    fn from(value: CommonKey) -> Self {
        value.as_str()
    }
}

impl From<CommonKey> for String {
    fn from(value: CommonKey) -> Self {
        value.to_string()
    }
}

impl From<CommonKey> for OsString {
    fn from(value: CommonKey) -> Self {
        OsString::from(String::from(value))
    }
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub(crate) enum ClientKey {
    TOR_PT_CLIENT_TRANSPORTS,
    TOR_PT_PROXY,
}

impl ClientKey {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            ClientKey::TOR_PT_CLIENT_TRANSPORTS => "TOR_PT_CLIENT_TRANSPORTS",
            ClientKey::TOR_PT_PROXY => "TOR_PT_PROXY",
        }
    }
}

impl fmt::Display for ClientKey {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "{:?}", self)
    }
}

impl From<ClientKey> for &str {
    fn from(value: ClientKey) -> Self {
        value.as_str()
    }
}

impl From<ClientKey> for String {
    fn from(value: ClientKey) -> Self {
        value.to_string()
    }
}

impl From<ClientKey> for OsString {
    fn from(value: ClientKey) -> Self {
        OsString::from(String::from(value))
    }
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub(crate) enum ServerKey {
    TOR_PT_SERVER_TRANSPORTS,
    TOR_PT_SERVER_TRANSPORT_OPTIONS,
    TOR_PT_SERVER_BINDADDR,
    TOR_PT_ORPORT,
    TOR_PT_EXTENDED_SERVER_PORT,
    TOR_PT_AUTH_COOKIE_FILE,
}

impl ServerKey {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            ServerKey::TOR_PT_SERVER_TRANSPORTS => "TOR_PT_SERVER_TRANSPORTS",
            ServerKey::TOR_PT_SERVER_TRANSPORT_OPTIONS => "TOR_PT_SERVER_TRANSPORT_OPTIONS",
            ServerKey::TOR_PT_SERVER_BINDADDR => "TOR_PT_SERVER_BINDADDR",
            ServerKey::TOR_PT_ORPORT => "TOR_PT_ORPORT",
            ServerKey::TOR_PT_EXTENDED_SERVER_PORT => "TOR_PT_EXTENDED_SERVER_PORT",
            ServerKey::TOR_PT_AUTH_COOKIE_FILE => "TOR_PT_AUTH_COOKIE_FILE",
        }
    }
}

impl fmt::Display for ServerKey {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "{:?}", self)
    }
}

impl From<ServerKey> for &str {
    fn from(value: ServerKey) -> Self {
        value.as_str()
    }
}

impl From<ServerKey> for String {
    fn from(value: ServerKey) -> Self {
        value.to_string()
    }
}

impl From<ServerKey> for OsString {
    fn from(value: ServerKey) -> Self {
        OsString::from(String::from(value))
    }
}

*/
