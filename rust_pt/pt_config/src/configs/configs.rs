#![allow(non_snake_case)]
use crate::MODE;
use anyhow::Result;
use derive_deftly::Deftly;
use pt_err::ConfigError;
use pt_err::derive_deftly_template_DefineVariantError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::convert::TryFrom;
use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr};
use std::path::PathBuf;
use std::str::FromStr;
use url::Url;
/// PTs SHOULD ignore PT names that it does not recognize.
/// So using String is acceptable, since we won't validate it.
type PtTransportName = String;
use crate::variable::{CURRENT_TRANSPORT_VER, SCHEMES};
///! Currently the most of the Errors are ConfigError::InvalidConfigErr{...}
///! TODO: Maybe Adding some diffrent Error would be better?
///! But I think developing more feature is more important now.

/// RawCommonKey is used to validate that the config is valid
/// #[allow(unused)] is fine in this situation,
/// since this is only to validate the config
#[derive(Debug, Deserialize)]
pub(crate) struct RawCommonKey {
    #[serde(rename = "tor_pt_managed_transport_ver")]
    TOR_PT_MANAGED_TRANSPORT_VER: String,

    #[serde(rename = "tor_pt_state_location")]
    TOR_PT_STATE_LOCATION: PathBuf,

    #[serde(rename = "tor_pt_exit_on_stdin_close")]
    TOR_PT_EXIT_ON_STDIN_CLOSE: i8,

    #[serde(rename = "tor_pt_outbound_bind_address_v4")]
    TOR_PT_OUTBOUND_BIND_ADDRESS_V4: Option<Ipv4Addr>,

    #[serde(rename = "tor_pt_outbound_bind_address_v6")]
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

/// Common settings required by both client-side and server-side
/// Pluggable Transport proxies.
#[derive(Debug, Serialize, Deserialize, Deftly)]
#[derive_deftly(DefineVariantError)]
#[serde(try_from = "RawCommonKey")]
pub struct CommonKey {
    /// Comma-separated list of Pluggable Transport specification versions
    /// supported by the parent process.
    /// Example:
    /// TOR_PT_MANAGED_TRANSPORT_VER=1,1a,2b,radish_is_a_valid_ver
    pub TOR_PT_MANAGED_TRANSPORT_VER: Vec<String>,

    /// Directory where the Pluggable Transport may persist state.
    /// Example:
    /// TOR_PT_STATE_LOCATION=/var/lib/tor/pt_state/
    pub TOR_PT_STATE_LOCATION: PathBuf,

    /// Whether the PT should gracefully exit when stdin is closed.
    /// Example:
    /// TOR_PT_EXIT_ON_STDIN_CLOSE=1
    pub TOR_PT_EXIT_ON_STDIN_CLOSE: i8,

    /// Optional IPv4 source address for outbound connections.
    ///
    /// If unset, the system default source address is used.
    /// Example:
    /// TOR_PT_OUTBOUND_BIND_ADDRESS_V4=203.0.113.4
    pub TOR_PT_OUTBOUND_BIND_ADDRESS_V4: Ipv4Addr,

    /// Optional IPv6 source address for outbound connections.
    ///
    /// If unset, the system default source address is used.
    /// Example:
    /// TOR_PT_OUTBOUND_BIND_ADDRESS_V6=[2001:db8::4]
    pub TOR_PT_OUTBOUND_BIND_ADDRESS_V6: Ipv6Addr,
}
impl TryFrom<RawCommonKey> for CommonKey {
    type Error = ConfigError;
    /// this is for validating whether the config is valid
    fn try_from(raw: RawCommonKey) -> Result<Self, Self::Error> {
        match raw.TOR_PT_EXIT_ON_STDIN_CLOSE {
            0 | 1 => {},
            _ => {
                return Err(ConfigError::InvalidConfigErr {
                    side: (*MODE).into(),
                    message: "TOR_PT_EXIT_ON_STDIN_CLOSE should be 1 or 0".to_string(),
                });
            },
        }
        // If this value is unset or empty,
        // the PT proxy MUST use the default source address for outgoing connections.
        // This setting MUST be ignored for connections to loopback addresses (127.0.0.0/8).
        let TOR_PT_OUTBOUND_BIND_ADDRESS_V4 = match raw.TOR_PT_OUTBOUND_BIND_ADDRESS_V4 {
            Some(x) if !x.is_loopback() => x,
            _ => "0.0.0.0"
                .parse::<Ipv4Addr>()
                .map_err(|e| ConfigError::InvalidConfigErr {
                    side: (*MODE).into(),
                    message: format!("Invalid TOR_PT_OUTBOUND_BIND_ADDRESS_V4: {}", e),
                })?,
        };
        let TOR_PT_OUTBOUND_BIND_ADDRESS_V6 = match raw.TOR_PT_OUTBOUND_BIND_ADDRESS_V6 {
            Some(x) if !x.is_loopback() => x,
            _ => "::"
                .parse::<Ipv6Addr>()
                .map_err(|e| ConfigError::InvalidConfigErr {
                    side: (*MODE).into(),
                    message: format!("Invalid TOR_PT_OUTBOUND_BIND_ADDRESS_V6: {}", e),
                })?,
        };
        let version = separate_with(raw.TOR_PT_MANAGED_TRANSPORT_VER, ",");
        if !version.iter().any(|v| v == CURRENT_TRANSPORT_VER) {
            return Err(ConfigError::UnsupportedVer {
                message: format!("Supported version: {}", CURRENT_TRANSPORT_VER),
            });
        }
        Ok(CommonKey {
            TOR_PT_MANAGED_TRANSPORT_VER: version,
            TOR_PT_STATE_LOCATION: raw.TOR_PT_STATE_LOCATION,
            TOR_PT_EXIT_ON_STDIN_CLOSE: raw.TOR_PT_EXIT_ON_STDIN_CLOSE,
            TOR_PT_OUTBOUND_BIND_ADDRESS_V4: TOR_PT_OUTBOUND_BIND_ADDRESS_V4,
            TOR_PT_OUTBOUND_BIND_ADDRESS_V6: TOR_PT_OUTBOUND_BIND_ADDRESS_V6,
        })
    }
}

#[derive(Debug, Deserialize)]
pub(crate) struct RawClientKey {
    #[serde(rename = "tor_pt_client_transports")]
    pub(crate) TOR_PT_CLIENT_TRANSPORTS: Vec<PtTransportName>,

    #[serde(rename = "tor_pt_proxy")]
    pub(crate) TOR_PT_PROXY: Option<Url>,
}

impl TryFrom<RawClientKey> for ClientKey {
    type Error = ClientKeyConfigError;
    fn try_from(value: RawClientKey) -> Result<Self, Self::Error> {
        if let Some(url) = value.TOR_PT_PROXY {
            validate_proxy_url(&url)?;
            return Ok(ClientKey {
                TOR_PT_CLIENT_TRANSPORTS: value.TOR_PT_CLIENT_TRANSPORTS,
                TOR_PT_PROXY: Some(url),
            });
        }
        Ok(ClientKey {
            TOR_PT_CLIENT_TRANSPORTS: value.TOR_PT_CLIENT_TRANSPORTS,
            TOR_PT_PROXY: None,
        })
    }
}
/// Settings which is needed at client side
#[derive(Debug, Serialize, Deserialize, Deftly)]
#[derive_deftly(DefineVariantError)]
#[serde(try_from = "RawClientKey")]
pub struct ClientKey {
    /// Specifies the PT protocols the client proxy should initialize, as a comma separated list of PT names.
    ///
    /// PTs SHOULD ignore PT names that it does not recognize.
    ///
    /// Parent processes MUST set this environment variable when launching a client-side PT proxy instance.
    ///
    /// Example:
    /// TOR_PT_CLIENT_TRANSPORTS=obfs2,obfs3,obfs4
    pub TOR_PT_CLIENT_TRANSPORTS: Vec<PtTransportName>,
    /// Specifies an upstream proxy that the PT MUST use when making outgoing network connections.
    ///
    /// It is a URI [RFC3986] of the format:
    /// `<proxy_type>://[<user_name>[:<password>][@]<ip>:<port>`
    ///
    /// This environment variable is OPTIONAL and MUST be omitted if there is no need to connect via an upstream proxy.
    ///
    /// Example
    /// TOR_PT_PROXY=socks5://user:pass@192.168.1.1:1080
    pub TOR_PT_PROXY: Option<Url>,
}

/// Validate a proxy Url
#[allow(clippy::collapsible_if)]
pub fn validate_proxy_url(spec: &Url) -> Result<(), ClientKeyConfigError> {
    if !SCHEMES.contains(&spec.scheme()) {
        return Err(ClientKeyConfigError::InvalidTOR_PT_PROXY {
            message: format!(
                "proxy URI has invalid scheme: {0} \n You should use: {1}",
                spec.scheme(),
                SCHEMES.join(", ")
            ),
        });
    }

    // when spec = http the path defaults to "/" instead of empty -_-
    if !spec.path().is_empty() {
        if !(spec.scheme() == "http" && spec.path() == "/") {
            return Err(ClientKeyConfigError::InvalidTOR_PT_PROXY {
                message: "proxy URI has a path defined ".to_string(),
            });
        }
    }
    if spec.query().is_some() {
        if !spec.query().unwrap().is_empty() {
            return Err(ClientKeyConfigError::InvalidTOR_PT_PROXY {
                message: "proxy URI has a query defined".to_string(),
            });
        }
    }
    if spec.fragment().is_some() {
        if !spec.fragment().unwrap().is_empty() {
            return Err(ClientKeyConfigError::InvalidTOR_PT_PROXY {
                message: "proxy URI has a fragment defined".to_string(),
            });
        }
    }
    if spec.port().is_none() {
        return Err(ClientKeyConfigError::InvalidTOR_PT_PROXY {
            message: "proxy URI lacks a port".to_string(),
        });
    }

    match spec.scheme() {
        "socks5" => {
            let username = spec.username();
            let passwd = spec.password();

            // if either password or username is specified, then both must be non-empty
            if !username.is_empty() || passwd.is_some() {
                if username.is_empty() || username.len() > 255 {
                    return Err(ClientKeyConfigError::InvalidTOR_PT_PROXY {
                        message: "proxy URI specified a invalid SOCKS5 username".to_string(),
                    });
                }
                if passwd.is_none() {
                    return Err(ClientKeyConfigError::InvalidTOR_PT_PROXY {
                        message: "proxy URI specified a invalid SOCKS5 password".to_string(),
                    });
                } else if let Some(p) = passwd {
                    if p.is_empty() || p.len() > 255 {
                        return Err(ClientKeyConfigError::InvalidTOR_PT_PROXY {
                            message: "proxy URI specified a invalid SOCKS5 password".to_string(),
                        });
                    }
                }
            }
        },
        "socks4a" => {
            if spec.password().is_some() {
                return Err(ClientKeyConfigError::InvalidTOR_PT_PROXY {
                    message: "proxy URI specified SOCKS4a and a password".to_string(),
                });
            }
        },
        "http" => {},
        _ => {
            return Err(ClientKeyConfigError::InvalidTOR_PT_PROXY {
                message: format!("proxy URI has invalid scheme: {}", spec.scheme()),
            });
        },
    }

    if spec.host_str().is_none() {
        return Err(ClientKeyConfigError::InvalidTOR_PT_PROXY {
            message: "proxy URI has missing host".to_string(),
        });
    }

    // not sure how better to combine host port.
    let mut sockaddr_string = String::from(spec.host_str().unwrap());
    sockaddr_string.push(':');
    sockaddr_string.push_str(&format!("{}", spec.port().unwrap()));
    let _ =
        resolve_addr(&sockaddr_string).map_err(|e| ClientKeyConfigError::InvalidTOR_PT_PROXY {
            message: format!("proxy URI has invalid host: {e}"),
        })?;

    Ok(())
}

/// return a Valid Socket Address
pub fn resolve_addr(addr: &str) -> Result<SocketAddr, ClientKeyConfigError> {
    let a = addr.as_ref();
    match SocketAddr::from_str(a) {
        Ok(sock_addr) => {
            if sock_addr.ip().is_unspecified() {
                return Err(ClientKeyConfigError::InvalidTOR_PT_PROXY {
                    message: format!("address string {a} lacks a host"),
                });
            }

            if sock_addr.port() == 0 {
                return Err(ClientKeyConfigError::InvalidTOR_PT_PROXY {
                    message: format!("address string {a} lacks a port"),
                });
            }
            Ok(sock_addr)
        },
        Err(e) => Err(ClientKeyConfigError::InvalidTOR_PT_PROXY {
            message: format!("\"{a}\" - {e}"),
        }),
    }
}

/// Specifies per-PT protocol configuration directives,
///  as a semicolon-separated list of <key>:<value> pairs,
///  where <key> is a PT name and <value> is a k=v string value
/// with options that are to be passed to the transport.
/// Example:
/// TOR_PT_SERVER_TRANSPORT_OPTIONS=scramblesuit:key=banana;automata:rule=110;automata:depth=3
#[derive(Debug, Serialize, Deserialize)]
pub struct TransportOption {
    /// Corresponding PtTransportName of Settings
    pub name: PtTransportName,
    /// Diffrent settings to PtTransport
    pub settings: HashMap<String, String>,
}

/// Settings which is needed at server side
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct RawServerKey {
    /// Example:
    /// TOR_PT_SERVER_TRANSPORTS=obfs3,scramblesuit
    #[serde(rename = "tor_pt_server_transports")]
    pub(crate) TOR_PT_SERVER_TRANSPORTS: Vec<PtTransportName>,

    /// Example:
    /// TOR_PT_SERVER_TRANSPORT_OPTIONS=scramblesuit:key=banana;automata:rule=110;automata:depth=3
    #[serde(rename = "tor_pt_server_transport_options")]
    pub(crate) TOR_PT_SERVER_TRANSPORT_OPTIONS: String,

    ///Example:
    /// TOR_PT_SERVER_BINDADDR=obfs3-198.51.100.1:1984,scramblesuit-127.0.0.1:4891
    #[serde(rename = "tor_pt_server_bindaddr")]
    pub(crate) TOR_PT_SERVER_BINDADDR: String,

    /// Specifies the destination that
    /// the PT reverse proxy should forward traffic to after transforming it as appropriate,
    ///  as an <address>:<port>.
    ///
    /// Connections to the destination specified via “TOR_PT_ORPORT” MUST only contain application payload.
    ///  If the parent process requires the actual source IP address of client connections (or other metadata),
    ///  it should set “TOR_PT_EXTENDED_SERVER_PORT” instead.
    ///
    ///  Example:
    /// TOR_PT_ORPORT==127.0.0.1:4200
    #[serde(rename = "tor_pt_orport")]
    pub(crate) TOR_PT_ORPORT: SocketAddr,

    /// Example:
    /// TOR_PT_EXTENDED_SERVER_PORT=127.0.0.1:4200
    #[serde(rename = "tor_pt_extended_server_port")]
    pub(crate) TOR_PT_EXTENDED_SERVER_PORT: SocketAddr,

    /// Example:
    /// TOR_PT_AUTH_COOKIE_FILE=/var/lib/tor/extended_orport_auth_cookie
    #[serde(rename = "tor_pt_auth_cookie_file")]
    pub(crate) TOR_PT_AUTH_COOKIE_FILE: PathBuf,
}

/// Many TransportOption
#[derive(Debug, Serialize, Deserialize)]
pub struct TransportOptions {
    /// Many TransportOption
    pub options: Vec<TransportOption>,
}
impl FromStr for TransportOptions {
    type Err = ConfigError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // scramblesuit:key=banana
        let options = s
            .split(';')
            .filter_map(|x| x.split_once(':'))
            .map(|(first, second)| {
                let (second_f, second_s) =
                    second
                        .split_once('=')
                        .ok_or_else(|| ConfigError::InvalidConfigErr {
                            side: MODE.clone().into(),
                            message: "Invalid TransportOptions Config".to_string(),
                        })?;

                Ok(TransportOption {
                    name: first.to_string(),
                    settings: HashMap::from([(second_f.to_string(), second_s.to_string())]),
                })
            })
            .collect::<Result<Vec<TransportOption>, ConfigError>>()?;

        Ok(TransportOptions { options: options })
    }
}

impl TryFrom<RawServerKey> for ServerKey {
    type Error = ConfigError;

    fn try_from(raw: RawServerKey) -> Result<Self, Self::Error> {
        let x = TransportOptions::from_str(raw.TOR_PT_SERVER_TRANSPORT_OPTIONS.as_str())?;
        let TOR_PT_SERVER_TRANSPORT_OPTIONS = match x.options {
            x if x.is_empty() => None,
            _ => Some(x),
        };
        // obfs3-198.51.100.1:1984,scramblesuit-127.0.0.1:4891
        let TOR_PT_SERVER_BINDADDR: Vec<HashMap<PtTransportName, SocketAddr>> = raw
            .TOR_PT_SERVER_BINDADDR
            .split(',') // ["obfs3-198.51.100.1:1984", "scramblesuit-127.0.0.1:4891"]
            .filter_map(|x| x.split_once('-')) // ["obfs3", "198.51.100.1:1984"]
            .map(|(name, bind_addr)| {
                let addr = bind_addr
                    .parse::<SocketAddr>()
                    .expect("Invalid TOR_PT_SERVER_BINDADDR");
                HashMap::from([(name.to_string(), addr)])
            })
            .collect();

        Ok(ServerKey {
            TOR_PT_SERVER_TRANSPORTS: raw.TOR_PT_SERVER_TRANSPORTS,
            TOR_PT_SERVER_TRANSPORT_OPTIONS: TOR_PT_SERVER_TRANSPORT_OPTIONS,
            TOR_PT_SERVER_BINDADDR: TOR_PT_SERVER_BINDADDR,
            TOR_PT_ORPORT: raw.TOR_PT_ORPORT,
            TOR_PT_EXTENDED_SERVER_PORT: raw.TOR_PT_EXTENDED_SERVER_PORT,
            TOR_PT_AUTH_COOKIE_FILE: raw.TOR_PT_AUTH_COOKIE_FILE,
        })
    }
}
/// Settings which is needed at server side
#[derive(Debug, Serialize, Deserialize)]
#[serde(try_from = "RawServerKey")]
pub struct ServerKey {
    /// Specifies the PT protocols the server proxy should initialize, as a comma separated list of PT names.
    /// PTs SHOULD ignore PT names that it does not recognize.
    /// Example:
    /// TOR_PT_SERVER_TRANSPORTS=obfs3,scramblesuit
    pub TOR_PT_SERVER_TRANSPORTS: Vec<PtTransportName>,
    /// Specifies per-PT protocol configuration directives,
    ///  as a semicolon-separated list of <key>:<value> pairs,
    ///  where <key> is a PT name and <value> is a k=v string value
    /// with options that are to be passed to the transport.
    /// Example:
    /// TOR_PT_SERVER_TRANSPORT_OPTIONS=scramblesuit:key=banana;automata:rule=110;automata:depth=3
    pub TOR_PT_SERVER_TRANSPORT_OPTIONS: Option<TransportOptions>,
    /// A comma separated list of <key>-<value> pairs,
    /// where <key> is a PT name and <value> is the <address>:<port>
    /// on which it should listen for incoming client connections.
    ///Example:
    /// TOR_PT_SERVER_BINDADDR=obfs3-198.51.100.1:1984,scramblesuit-127.0.0.1:4891
    pub TOR_PT_SERVER_BINDADDR: Vec<HashMap<PtTransportName, SocketAddr>>,
    /// Specifies the destination that
    /// the PT reverse proxy should forward traffic to after transforming it as appropriate,
    ///  as an <address>:<port>.
    /// Connections to the destination specified via “TOR_PT_ORPORT” MUST only contain application payload.
    ///  If the parent process requires the actual source IP address of client connections (or other metadata),
    ///  it should set “TOR_PT_EXTENDED_SERVER_PORT” instead.
    ///  Example:
    /// TOR_PT_ORPORT==127.0.0.1:4200
    pub TOR_PT_ORPORT: SocketAddr,
    /// Specifies the destination that the PT reverse proxy should forward traffic to,
    /// via the Extended ORPort protocol [EXTORPORT] as an <address>:<port>.
    /// The Extended ORPort protocol allows the PT reverse proxy to communicate per-connection metadata
    /// such as the PT name and client IP address/port to the parent process.
    /// If the parent process does not support the ExtORPort protocol,
    /// it MUST set “TOR_PT_EXTENDED_SERVER_PORT” to an empty string.
    /// Example:
    /// TOR_PT_EXTENDED_SERVER_PORT=127.0.0.1:4200
    pub TOR_PT_EXTENDED_SERVER_PORT: SocketAddr,
    /// Specifies an absolute filesystem path to the Extended ORPort authentication cookie,
    ///  required to communicate with the Extended ORPort specified via “TOR_PT_EXTENDED_SERVER_PORT”.
    /// If the parent process is not using the ExtORPort protocol for incoming traffic,
    /// “TOR_PT_AUTH_COOKIE_FILE” MUST be omitted.
    /// Example:
    /// TOR_PT_AUTH_COOKIE_FILE=/var/lib/tor/extended_orport_auth_cookie
    pub TOR_PT_AUTH_COOKIE_FILE: PathBuf,
}
