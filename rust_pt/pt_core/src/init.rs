// File: rust_pt\pt_core\src\init.rs
// Directory: rust_pt\pt_core\src
// Filename: init.rs
//======================================================================
use crate::orport::extorport::ExtOrPort;
use crate::{OrPortKind, Pt};
use pt_config::prelude::*;
use pt_err::PtError;
use pt_tracing::prelude::*;
use std::collections::HashSet;
use tokio::net::TcpStream;
/// Filters `strs`, keeping only elements present in `supported`, preserving order.
/// Used for `smethod` transports
#[must_use]
pub fn smethod_filter(strs: &[String], supported: &[String]) -> Vec<String> {
    strs.iter()
        .filter(|v| {
            if supported.contains(v) {
                true
            } else {
                PtTracing::smethod_error(v, "Unsupported smethod transport");
                false
            }
        })
        .cloned()
        .collect()
}

/// Filters `strs`, keeping only elements present in `supported`, preserving order.
/// Used for `cmethod` transports
#[must_use]
pub fn cmethod_filter(strs: &[String], supported: &[String]) -> Vec<String> {
    strs.iter()
        .filter(|v| {
            if supported.contains(v) {
                true
            } else {
                PtTracing::cmethod_error(v, "Unsupported cmethod transport");
                false
            }
        })
        .cloned()
        .collect()
}

/// Filters `strs`, keeping only elements present in `supported`, preserving order.
/// Will build a `HashSet` for performance
/// ( I know that may be useless, but what if there is a pro developer needs that?)
#[must_use]
pub fn smethod_hash_filter(strs: &[String], supported: &[String]) -> Vec<String> {
    let set: HashSet<&str> = supported.iter().map(String::as_str).collect();
    strs.iter()
        .filter(|v| {
            if set.contains(v.as_str()) {
                true
            } else {
                PtTracing::smethod_error(v, "Unsupported transport");
                false
            }
        })
        .cloned()
        .collect()
}

/// Filters `strs`, keeping only elements present in `supported`, preserving order.
/// Will build a `HashSet` for performance
/// ( I know that may be useless, but what if there is a pro developer needs that?)
#[must_use]
pub fn cmethod_hash_filter(strs: &[String], supported: &[String]) -> Vec<String> {
    let set: HashSet<&str> = supported.iter().map(String::as_str).collect();
    strs.iter()
        .filter(|v| {
            if set.contains(v.as_str()) {
                true
            } else {
                PtTracing::cmethod_error(v, "Unsupported transport");
                false
            }
        })
        .cloned()
        .collect()
}

impl Pt {
    /// Filters the PT's transports against `sup_transports`, dropping unsupported
    /// ones. Client filters `TOR_PT_CLIENT_TRANSPORTS` via `cmethod_filter`;
    /// server filters `TOR_PT_SERVER_TRANSPORTS` via `smethod_filter`.
    /// No-op if `sup_transports` is `None`.
    /// # Errors
    /// - Pt is never initiated
    pub fn filter_transports(&mut self) -> Result<(), PtError> {
        if let Some(supported_transports) = self.sup_transports.clone() {
            if self.is_client()? {
                let config = self.get_client_config_mut()?;
                let filtered =
                    cmethod_filter(&config.TOR_PT_CLIENT_TRANSPORTS, &supported_transports);
                config.TOR_PT_CLIENT_TRANSPORTS = filtered;
            } else {
                // It should be server
                let config = self.get_server_config_mut()?;
                let filtered =
                    smethod_filter(&config.TOR_PT_SERVER_TRANSPORTS, &supported_transports);
                config.TOR_PT_SERVER_TRANSPORTS = filtered;
            }
        }
        Ok(())
    }
    /// # Panics
    /// - if an unsupported scheme exists
    /// # Errors
    /// if the Pt is never inited
    pub fn filter_url(&mut self) -> Result<(), PtError> {
        #[allow(clippy::collapsible_if)] // This is for making sure that there `is_client()`
        if self.is_client()?
            && let Some(supported_schemes) = &self.proxy_schemes
        {
            if let Some(url) = &self.get_client_config()?.TOR_PT_PROXY {
                let scheme = url.scheme();
                if !supported_schemes.contains(scheme) {
                    PtTracing::env_error("Invalid proxy url");
                    panic!("Unsupported url {scheme}")
                }
            }
        }
        Ok(())
    }
}

impl Pt {
    /// Init the Pt
    /// Including configs, and Logs
    /// Do not init the logs, if you would call that
    ///
    /// # Note
    /// - We will automatically send `Proxy Done` and `Version`
    ///
    /// # Errors
    /// Could panic, if `PtTracing` inition fails
    ///
    /// # Panics
    /// -  if environment are not set or contains incorrect settings
    pub fn try_init(&mut self) -> Result<(), PtError> {
        PtTracing::fmt().with_severity(self.severity).try_init()?;
        PtTracing::notice("initing the Pt")?; // Could Panic (See docs of notice())
        self.config_key = {
            let config_key = ConfigKey::init();
            // Invariant: the `TOR_PT_MANAGED_TRANSPORT_VER` must not be empty
            // But we have already made sure in deserialization's validation, that it is not empty
            // Thus, it is safe here
            PtTracing::version(&config_key.common_key().TOR_PT_MANAGED_TRANSPORT_VER[0]);
            Some(config_key)
        }; // Could Panic (See docs of init())
        // Filter the unsupported transports
        self.filter_transports()?;
        self.filter_url()?;
        Ok(())
    }
    /// try to connect into extorport
    /// This will change the `extorport` field of the `Pt` instance
    ///
    /// # Errors
    /// if Pt is not initialized
    pub async fn try_extorport(&self) -> Result<ExtOrPort, PtError> {
        let server_key = self.get_server_config()?;
        if let Some(ext_addr) = server_key.TOR_PT_EXTENDED_SERVER_PORT
            && let Some(ext_cookie_file) = server_key.TOR_PT_AUTH_COOKIE_FILE.clone()
        {
            // Handling `ExtOrPort`
            let mut ext_orport: ExtOrPort = ExtOrPort::builder()
                .with_addr(ext_addr)
                .with_auth_cookie_file(ext_cookie_file);
            ext_orport.connect().await?;
            return Ok(ext_orport);
        }

        Err(PtError::ExtOrPortNotAvailable(
            "invalid or empty Config".to_string(),
        ))
    }
    /// Connect to the `ExtOrPort`
    /// if there is not a `ExtOrPort`
    /// we will use `OrPort` instead
    ///
    /// # Errors
    /// if Pt is not initialized, or `connect` fails
    /// (See [`ExtOrPort::connect`])
    pub async fn connect_or(&self) -> Result<OrPortKind, PtError> {
        if self.is_client()? {
            return Err(PtError::ClientOrPortUnavailable);
        }
        match self.try_extorport().await {
            Ok(extorport) => Ok(OrPortKind::ExtOrPort(extorport)),
            Err(PtError::ExtOrPortNotAvailable(_)) => {
                // Try to connect to the OrPort
                if let Some(addr) = self.get_server_config()?.TOR_PT_ORPORT {
                    return Ok(OrPortKind::OrPort(TcpStream::connect(addr).await?));
                }
                PtTracing::error("Config poisoned, OrPort and ExtOrPort both unavailable")?;
                Err(PtError::PtConfigPoisoned(
                    "OrPort and ExtOrPort both unavailable".to_string(),
                ))
            },
            Err(e) => Err(e),
        }
    }
}
