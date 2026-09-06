// File: rust_pt\pt_config\src\configs\keys.rs
// Directory: rust_pt\pt_config\src\configs
// Filename: keys.rs
//======================================================================

//! Some structure are marked as exhausive
//! since most of the structure follows Tor-Spec
//! That's why they will not change easily
#![allow(non_snake_case)]
#![allow(clippy::struct_field_names)]
use anyhow::Result;
use derive_deftly::Deftly;
use pt_err::ConfigError;
use pt_err::derive_deftly_template_DefineVariantError;
use pt_tracing::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt::Write;
use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr};
use std::path::PathBuf;
use std::str::FromStr;
use url::Url;
/// PTs SHOULD ignore PT names that it does not recognize.
/// So using String is acceptable, since we won't validate it.
type PtTransportName = String;
use crate::variable::{CURRENT_TRANSPORT_VER, SCHEMES};

/// `RawCommonKey` is used to validate that the config is valid
#[derive(Debug, Deserialize)]
pub(crate) struct RawCommonKey {
    #[serde(rename = "tor_pt_managed_transport_ver")]
    /// For field documentation, see the corresponding fields in [`CommonKey`].
    TOR_PT_MANAGED_TRANSPORT_VER: String,

    #[serde(rename = "tor_pt_state_location")]
    /// For field documentation, see the corresponding fields in [`CommonKey`].
    TOR_PT_STATE_LOCATION: PathBuf,

    #[serde(rename = "tor_pt_exit_on_stdin_close")]
    /// For field documentation, see the corresponding fields in [`CommonKey`].
    TOR_PT_EXIT_ON_STDIN_CLOSE: i8,

    #[serde(rename = "tor_pt_outbound_bind_address_v4")]
    /// For field documentation, see the corresponding fields in [`CommonKey`].
    TOR_PT_OUTBOUND_BIND_ADDRESS_V4: Option<Ipv4Addr>,

    #[serde(rename = "tor_pt_outbound_bind_address_v6")]
    /// For field documentation, see the corresponding fields in [`CommonKey`].
    TOR_PT_OUTBOUND_BIND_ADDRESS_V6: Option<Ipv6Addr>,
}
/// Splits `s` by commas and collects the results into a `Vec<String>`.
pub fn separate_with(s: &str, comma: &str) -> Vec<String> {
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
#[allow(clippy::exhaustive_structs)]
pub struct CommonKey {
    /// Comma-separated list of Pluggable Transport specification versions
    /// supported by the parent process.
    /// Example:
    /// `TOR_PT_MANAGED_TRANSPORT_VER=1,1a,2b,radish_is_a_valid_ver`
    pub TOR_PT_MANAGED_TRANSPORT_VER: Vec<String>,

    /// Directory where the Pluggable Transport may persist state.
    /// Example:
    /// `TOR_PT_STATE_LOCATION=/var/lib/tor/pt_state`/
    pub TOR_PT_STATE_LOCATION: PathBuf,

    /// Whether the PT should gracefully exit when stdin is closed.
    /// Example:
    /// `TOR_PT_EXIT_ON_STDIN_CLOSE=1`
    pub TOR_PT_EXIT_ON_STDIN_CLOSE: i8,

    /// Optional IPv4 source address for outbound connections.
    ///
    /// If unset, the system default source address is used.
    /// Example:
    /// `TOR_PT_OUTBOUND_BIND_ADDRESS_V4=203.0.113.4`
    pub TOR_PT_OUTBOUND_BIND_ADDRESS_V4: Ipv4Addr,

    /// Optional IPv6 source address for outbound connections.
    ///
    /// If unset, the system default source address is used.
    /// Example:
    /// `TOR_PT_OUTBOUND_BIND_ADDRESS_V6`=[`2001:db8::4`]
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
                    message: format!("Invalid TOR_PT_OUTBOUND_BIND_ADDRESS_V4: {e}"),
                })?,
        };
        let TOR_PT_OUTBOUND_BIND_ADDRESS_V6 = match raw.TOR_PT_OUTBOUND_BIND_ADDRESS_V6 {
            Some(x) if !x.is_loopback() => x,
            _ => "::"
                .parse::<Ipv6Addr>()
                .map_err(|e| ConfigError::InvalidConfigErr {
                    message: format!("Invalid TOR_PT_OUTBOUND_BIND_ADDRESS_V6: {e}"),
                })?,
        };
        let version = separate_with(&raw.TOR_PT_MANAGED_TRANSPORT_VER, ",");
        if !version
            .iter()
            .any(|v| CURRENT_TRANSPORT_VER.contains(&v.as_str()))
        {
            return Err(ConfigError::UnsupportedVer {
                message: format!("Supported version: {}", CURRENT_TRANSPORT_VER.join(", ")),
            });
        }
        Ok(CommonKey {
            TOR_PT_MANAGED_TRANSPORT_VER: version,
            TOR_PT_STATE_LOCATION: raw.TOR_PT_STATE_LOCATION,
            TOR_PT_EXIT_ON_STDIN_CLOSE: raw.TOR_PT_EXIT_ON_STDIN_CLOSE,
            TOR_PT_OUTBOUND_BIND_ADDRESS_V4,
            TOR_PT_OUTBOUND_BIND_ADDRESS_V6,
        })
    }
}

/// Used to validate that the config is valid
#[derive(Debug, Deserialize)]
pub(crate) struct RawClientKey {
    #[serde(rename = "tor_pt_client_transports")]
    /// For field documentation, see the corresponding fields in [`ClintKey`].
    pub(crate) TOR_PT_CLIENT_TRANSPORTS: Vec<PtTransportName>,

    #[serde(rename = "tor_pt_proxy")]
    /// For field documentation, see the corresponding fields in [`ClientKey`].
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
#[allow(clippy::exhaustive_structs)]
pub struct ClientKey {
    /// Specifies the PT protocols the client proxy should initialize, as a comma separated list of PT names.
    ///
    /// PTs SHOULD ignore PT names that it does not recognize.
    ///
    /// Parent processes MUST set this environment variable when launching a client-side PT proxy instance.
    ///
    /// Example:
    /// `TOR_PT_CLIENT_TRANSPORTS=obfs2,obfs3,obfs4`
    pub TOR_PT_CLIENT_TRANSPORTS: Vec<PtTransportName>,
    /// Specifies an upstream proxy that the PT MUST use when making outgoing network connections.
    ///
    /// It is a URI [RFC3986] of the format:
    /// `<proxy_type>://[<user_name>[:<password>][@]<ip>:<port>`
    ///
    /// This environment variable is OPTIONAL and MUST be omitted if there is no need to connect via an upstream proxy.
    ///
    /// Example
    /// `TOR_PT_PROXY=socks5://user:pass@192.168.1.1:1080`
    pub TOR_PT_PROXY: Option<Url>,
}

/// Validate a proxy Url
/// checks that the provided proxy URL:
/// - Uses a supported scheme (`socks5`, `socks4a`, or `http`).
/// - Has a valid host and port.
/// - Has no query parameters or fragments.
/// - Has valid credentials for the specified scheme.
/// - Can be resolved to a socket address.
///
/// # Panics
/// Panics if the URL is missing a host or port after the initial validation checks.
/// This should not happen in practice because the function checks for these earlier,
/// but the `expect()` calls are used to simplify the code.
///
/// # Errors
/// Returns `ClientKeyConfigError::InvalidTOR_PT_PROXY` if:
/// - The URL scheme is not supported (`socks5`, `socks4a`, or `http`).
/// - The URL contains a query string or fragment.
/// - The URL has a non-empty path (except for `http` scheme with path `/`).
/// - The URL is missing a port.
/// - The URL is missing a host.
/// - The URL has invalid credentials:
///   - For `socks5`: username is empty but password is provided, or vice versa.
///   - For `socks5`: username or password exceeds 255 characters.
///   - For `socks4a`: a password is provided (not supported).
/// - The host cannot be resolved to a socket address.
/// # NOTE:
/// this function is taken from ptrs and we changed it slighly
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
    if let Some(query) = spec.query() {
        if !query.is_empty() {
            return Err(ClientKeyConfigError::InvalidTOR_PT_PROXY {
                message: "proxy URI has a query defined".to_string(),
            });
        }
    }
    if let Some(fragment) = spec.fragment() {
        if !fragment.is_empty() {
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
    let mut sockaddr_string = String::from(spec.host_str().expect("Proxy URL missing host"));
    sockaddr_string.push(':');
    // Write to a normal String will never fail
    let _ = write!(
        sockaddr_string,
        "{}",
        spec.port().expect("Proxy URL missing port")
    );
    let _ =
        resolve_addr(&sockaddr_string).map_err(|e| ClientKeyConfigError::InvalidTOR_PT_PROXY {
            message: format!("proxy URI has invalid host: {e}"),
        })?;

    Ok(())
}

/// return a Valid Socket Address
/// This function parses a string like `"127.0.0.1:8080"` into a `SocketAddr`,
/// and performs additional validation to ensure the address is usable.
/// # Errors
/// Returns `ClientKeyConfigError::InvalidTOR_PT_PROXY` if:
/// - The string cannot be parsed as a valid socket address (e.g., malformed format).
/// - The IP address is unspecified (e.g., `0.0.0.0` or `::`).
/// - The port is `0`.
pub fn resolve_addr(a: &str) -> Result<SocketAddr, ClientKeyConfigError> {
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
/// `TOR_PT_SERVER_TRANSPORT_OPTIONS=scramblesuit:key=banana;automata:rule=110;automata:depth=3`
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[non_exhaustive]
pub struct TransportOption {
    /// Corresponding `PtTransportName` of Settings
    pub name: PtTransportName,
    /// Diffrent settings to `PtTransport`
    pub settings: HashMap<String, String>,
}

/// Settings which is needed at server side
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct RawServerKey {
    /// Example:
    /// `TOR_PT_SERVER_TRANSPORTS=obfs3,scramblesuit`
    #[serde(rename = "tor_pt_server_transports")]
    pub(crate) TOR_PT_SERVER_TRANSPORTS: Vec<PtTransportName>,

    /// Example:
    /// `TOR_PT_SERVER_TRANSPORT_OPTIONS=scramblesuit:key=banana;automata:rule=110;automata:depth=3`
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
    /// Connections to the destination specified via “`TOR_PT_ORPORT`” MUST only contain application payload.
    ///  If the parent process requires the actual source IP address of client connections (or other metadata),
    ///  it should set “`TOR_PT_EXTENDED_SERVER_PORT`” instead.
    ///
    ///  Example:
    /// `TOR_PT_ORPORT==127.0.0.1:4200`
    #[serde(rename = "tor_pt_orport")]
    pub(crate) TOR_PT_ORPORT: SocketAddr,

    /// Example:
    /// `TOR_PT_EXTENDED_SERVER_PORT=127.0.0.1:4200`
    #[serde(rename = "tor_pt_extended_server_port")]
    pub(crate) TOR_PT_EXTENDED_SERVER_PORT: SocketAddr,

    /// Example:
    /// `TOR_PT_AUTH_COOKIE_FILE=/var/lib/tor/extended_orport_auth_cookie`
    #[serde(rename = "tor_pt_auth_cookie_file")]
    pub(crate) TOR_PT_AUTH_COOKIE_FILE: PathBuf,
}

/// Many `TransportOption`
/// This is exhaustive, because we will not change it
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[allow(clippy::exhaustive_structs)]
pub struct TransportOptions {
    /// Many `TransportOption`
    pub options: Vec<TransportOption>,
}
impl FromStr for TransportOptions {
    type Err = ConfigError;
    /// Input: "scramblesuit:key=banana;automata:rule=110;automata:depth=3"
    /// Output: [
    ///   `TransportOption` { name: "scramblesuit", settings: {"key": "banana"} },
    ///   `TransportOption` { name: "automata", settings: {"rule": "110"} },
    ///   `TransportOption` { name: "automata", settings: {"depth": "3"} },
    /// ]
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
                            message: "Invalid TransportOptions Config".to_string(),
                        })?;

                Ok(TransportOption {
                    name: first.to_string(),
                    settings: HashMap::from([(second_f.to_string(), second_s.to_string())]),
                })
            })
            .collect::<Result<Vec<TransportOption>, ConfigError>>()?;

        Ok(TransportOptions { options })
    }
}

impl TransportOption {
    /// To generate a `TransportOption::settings`
    #[must_use]
    pub fn generate_settings(pairs: &[(&str, &str)]) -> HashMap<String, String> {
        pairs
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect()
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
        // Pluggable transport proxies SHOULD issue a warning
        // if they are instructed to connect to a non-localhost Extended ORPort.
        if !raw.TOR_PT_EXTENDED_SERVER_PORT.ip().is_loopback() {
            PtTracing::warn(&format!(
                "Extended ORPort is not on localhost: {}",
                raw.TOR_PT_EXTENDED_SERVER_PORT
            ))?;
        }
        Ok(ServerKey {
            TOR_PT_SERVER_TRANSPORTS: raw.TOR_PT_SERVER_TRANSPORTS,
            TOR_PT_SERVER_TRANSPORT_OPTIONS,
            TOR_PT_SERVER_BINDADDR,
            TOR_PT_ORPORT: raw.TOR_PT_ORPORT,
            TOR_PT_EXTENDED_SERVER_PORT: raw.TOR_PT_EXTENDED_SERVER_PORT,
            TOR_PT_AUTH_COOKIE_FILE: raw.TOR_PT_AUTH_COOKIE_FILE,
        })
    }
}
/// Settings which is needed at server side
#[derive(Debug, Serialize, Deserialize)]
#[serde(try_from = "RawServerKey")]
#[allow(clippy::exhaustive_structs)]
pub struct ServerKey {
    /// Specifies the PT protocols the server proxy should initialize, as a comma separated list of PT names.
    /// PTs SHOULD ignore PT names that it does not recognize.
    /// Example:
    /// `TOR_PT_SERVER_TRANSPORTS=obfs3,scramblesuit`
    pub TOR_PT_SERVER_TRANSPORTS: Vec<PtTransportName>,
    /// Specifies per-PT protocol configuration directives,
    ///  as a semicolon-separated list of <key>:<value> pairs,
    ///  where <key> is a PT name and <value> is a k=v string value
    /// with options that are to be passed to the transport.
    /// Example:
    /// `TOR_PT_SERVER_TRANSPORT_OPTIONS=scramblesuit:key=banana;automata:rule=110;automata:depth=3`
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
    /// Connections to the destination specified via “`TOR_PT_ORPORT`” MUST only contain application payload.
    ///  If the parent process requires the actual source IP address of client connections (or other metadata),
    ///  it should set “`TOR_PT_EXTENDED_SERVER_PORT`” instead.
    ///  Example:
    /// `TOR_PT_ORPORT==127.0.0.1:4200`
    pub TOR_PT_ORPORT: SocketAddr,
    /// Specifies the destination that the PT reverse proxy should forward traffic to,
    /// via the Extended `ORPort` protocol [EXTORPORT] as an <address>:<port>.
    /// The Extended `ORPort` protocol allows the PT reverse proxy to communicate per-connection metadata
    /// such as the PT name and client IP address/port to the parent process.
    /// If the parent process does not support the `ExtORPort` protocol,
    /// it MUST set “`TOR_PT_EXTENDED_SERVER_PORT`” to an empty string.
    /// Example:
    /// `TOR_PT_EXTENDED_SERVER_PORT=127.0.0.1:4200`
    pub TOR_PT_EXTENDED_SERVER_PORT: SocketAddr,
    /// Specifies an absolute filesystem path to the Extended `ORPort` authentication cookie,
    ///  required to communicate with the Extended `ORPort` specified via “`TOR_PT_EXTENDED_SERVER_PORT`”.
    /// If the parent process is not using the `ExtORPort` protocol for incoming traffic,
    /// “`TOR_PT_AUTH_COOKIE_FILE`” MUST be omitted.
    /// Example:
    /// `TOR_PT_AUTH_COOKIE_FILE=/var/lib/tor/extended_orport_auth_cookie`
    pub TOR_PT_AUTH_COOKIE_FILE: PathBuf,
}

#[cfg(test)]
mod test {
    // @@ begin test lint list maintained by maint/add_warning @@
    #![allow(clippy::bool_assert_comparison)]
    #![allow(clippy::clone_on_copy)]
    #![allow(clippy::dbg_macro)]
    #![allow(clippy::mixed_attributes_style)]
    #![allow(clippy::print_stderr)]
    #![allow(clippy::print_stdout)]
    #![allow(clippy::single_char_pattern)]
    #![allow(clippy::unwrap_used)]
    #![allow(clippy::unchecked_time_subtraction)]
    #![allow(clippy::useless_vec)]
    #![allow(clippy::needless_pass_by_value)]
    #![allow(clippy::string_slice)] // See arti#2571
    #![allow(clippy::pedantic)]
    //! <!-- @@ end test lint list maintained by maint/add_warning @@ -->
    #![allow(unused)]
    use super::*;

    /// Input: "scramblesuit:key=banana;automata:rule=110;automata:depth=3"
    /// Output: [
    ///   TransportOption { name: "scramblesuit", settings: {"key": "banana"} },
    ///   TransportOption { name: "automata", settings: {"rule": "110"} },
    ///   TransportOption { name: "automata", settings: {"depth": "3"} },
    /// ]
    #[test]
    fn test_for_from_str_TransportOptions() {
        assert_eq!(
            TransportOptions::from_str(
                "scramblesuit:key=banana;automata:rule=110;automata:depth=3"
            )
            .unwrap(),
            TransportOptions {
                options: vec![
                    TransportOption {
                        name: "scramblesuit".to_string(),
                        settings: TransportOption::generate_settings(&[("key", "banana")])
                    },
                    TransportOption {
                        name: "automata".to_string(),
                        settings: TransportOption::generate_settings(&[("rule", "110")])
                    },
                    TransportOption {
                        name: "automata".to_string(),
                        settings: TransportOption::generate_settings(&[("depth", "3")])
                    },
                ]
            }
        );
    }
}
