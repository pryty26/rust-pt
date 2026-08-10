#![allow(non_snake_case)]
use crate::MODE;
use anyhow::Result;
use pt_err::ConfigError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::convert::TryFrom;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::path::PathBuf;
use url::Url;
/// PTs SHOULD ignore PT names that it does not recognize.
/// So using String is acceptable, since we won't validate it.
type PtTransportName = String;
/// RawCommonKey is used to validate that the config is valid
/// #[allow(unused)] is fine in this situation,
/// since this is only to validate the config
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub(crate) struct RawCommonKey {
    TOR_PT_MANAGED_TRANSPORT_VER: Vec<String>,
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
        Ok(CommonKey {
            TOR_PT_MANAGED_TRANSPORT_VER: raw.TOR_PT_MANAGED_TRANSPORT_VER,
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
    /// Example:
    /// "TOR_PT_MANAGED_TRANSPORT_VER": ["1.0", "2.0", "3.0"]
    TOR_PT_MANAGED_TRANSPORT_VER: Vec<String>,

    /// Directory where the Pluggable Transport may persist state.
    /// Example:
    /// "TOR_PT_STATE_LOCATION": "/var/lib/tor/state",
    TOR_PT_STATE_LOCATION: PathBuf,

    /// Whether the PT should gracefully exit when stdin is closed.
    /// Example:
    /// "TOR_PT_STATE_LOCATION": 1
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
pub struct ClientKey {
    TOR_PT_CLIENT_TRANSPORTS: Vec<PtTransportName>,
    TOR_PT_PROXY: Option<Url>,
}

/// Specifies per-PT protocol configuration directives,
///  as a semicolon-separated list of <key>:<value> pairs,
///  where <key> is a PT name and <value> is a k=v string value
/// with options that are to be passed to the transport.
/// Example:
/// TOR_PT_SERVER_TRANSPORT_OPTIONS=scramblesuit:key=banana;automata:rule=110;automata:depth=3
#[derive(Debug, Serialize, Deserialize)]
pub struct TransportOptions {
    name: PtTransportName,
    settings: HashMap<String, String>,
}

/// Settings which is needed at server side
#[derive(Debug, Serialize, Deserialize)]
pub struct ServerKey {
    /// Specifies the PT protocols the server proxy should initialize, as a comma separated list of PT names.
    /// PTs SHOULD ignore PT names that it does not recognize.
    /// Example:
    /// TOR_PT_SERVER_TRANSPORTS=obfs3,scramblesuit
    TOR_PT_SERVER_TRANSPORTS: Vec<PtTransportName>,
    /// Specifies per-PT protocol configuration directives,
    ///  as a semicolon-separated list of <key>:<value> pairs,
    ///  where <key> is a PT name and <value> is a k=v string value
    /// with options that are to be passed to the transport.
    /// Example:
    /// TOR_PT_SERVER_TRANSPORT_OPTIONS=scramblesuit:key=banana;automata:rule=110;automata:depth=3
    TOR_PT_SERVER_TRANSPORT_OPTIONS: Option<TransportOptions>,
    /// A comma separated list of <key>-<value> pairs,
    /// where <key> is a PT name and <value> is the <address>:<port>
    /// on which it should listen for incoming client connections.
    ///Example:
    /// TOR_PT_SERVER_BINDADDR=obfs3-198.51.100.1:1984,scramblesuit-127.0.0.1:4891
    TOR_PT_SERVER_BINDADDR: Vec<IpAddr>,
    /// Specifies the destination that
    /// the PT reverse proxy should forward traffic to after transforming it as appropriate,
    ///  as an <address>:<port>.

    /// Connections to the destination specified via “TOR_PT_ORPORT” MUST only contain application payload.
    ///  If the parent process requires the actual source IP address of client connections (or other metadata),
    ///  it should set “TOR_PT_EXTENDED_SERVER_PORT” instead.
    TOR_PT_ORPORT: IpAddr,
    TOR_PT_EXTENDED_SERVER_PORT: IpAddr,
    TOR_PT_AUTH_COOKIE_FILE: PathBuf,
}

/*let config: TorConfig = envy::from_env()?;
*/
