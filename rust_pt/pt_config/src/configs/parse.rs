use crate::configs::configs::{ClientKey, ServerKey, CommonKey, ClientKeyConfigError};
use url::Url;
use pt_err::ConfigError;

#[allow(clippy::collapsible_if)]
pub(crate) fn validate_proxy_url(spec: &Url) -> Result<(), ClientKeyConfigError> {
    const SCHEMES: [&str; 3] = ["socks5", "socks4a", "http"];
    if !SCHEMES.contains(&spec.scheme()) {
        return Err(ClientKeyConfigError::InvalidTOR_PT_PROXY {
            message: format!("proxy URI has invalid scheme: {}", spec.scheme()),
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
        }
        "socks4a" => {
            if spec.password().is_some() {
                return Err(ClientKeyConfigError::InvalidTOR_PT_PROXY {
                    message: "proxy URI specified SOCKS4a and a password".to_string(),
                });
            }
        }
        "http" => {}
        _ => {
            return Err(ClientKeyConfigError::InvalidTOR_PT_PROXY {
                message: format!("proxy URI has invalid scheme: {}", spec.scheme()),
            });
        }
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
    let _ = resolve_addr(&sockaddr_string)
        .map_err(|e| ClientKeyConfigError::InvalidTOR_PT_PROXY {
            message: format!("proxy URI has invalid host: {e}"),
        })?;

    Ok(())
}