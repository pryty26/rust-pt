// File: rust_pt\pt_core\src\init.rs
// Directory: rust_pt\pt_core\src
// Filename: init.rs
//======================================================================
use crate::orport::extorport::ExtOrPort;
use crate::orport::traits::ClientExtOrPortProtocol;
use crate::{OrPortKind, Pt};
use anyhow::Result;
use pt_config::configs::keys::PtTransportName;
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
                let config = self.get_server_config_mut()?;
                let filtered =
                    smethod_filter(&config.TOR_PT_SERVER_TRANSPORTS, &supported_transports);
                config.TOR_PT_SERVER_TRANSPORTS = filtered;
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
    pub fn try_init(&mut self) -> Result<()> {
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
        Ok(())
    }
    /// try to connect into extorport
    /// This will change the `extorport` field of the `Pt` instance
    ///
    /// # Errors
    /// if Pt is not initialized
    pub async fn try_extorport(&mut self) -> Result<(), PtError> {
        let server_key = self.get_server_config()?;
        if let Some(ext_addr) = server_key.TOR_PT_EXTENDED_SERVER_PORT
            && let Some(ext_cookie_file) = server_key.TOR_PT_AUTH_COOKIE_FILE.clone()
        {
            // Handling `ExtOrPort`
            self.extorport = {
                let mut ext_orport: ExtOrPort = ExtOrPort::builder()
                    .with_addr(ext_addr)
                    .with_auth_cookie_file(ext_cookie_file);
                ext_orport.connect().await?;
                Some(ext_orport)
            };
        } else {
            return Err(PtError::ExtOrPortNotAvailable(
                "invalid or empty Config".to_string(),
            ));
        }
        Ok(())
    }

    /// Connect to the `ExtOrPort`
    /// if there is not a `ExtOrPort`
    /// we will use `OrPort` instead
    ///
    /// # Errors
    /// if Pt is not initialized, or `connect` fails
    /// (See [`ExtOrPort::connect`])
    pub async fn connect_or(&mut self) -> Result<OrPortKind, PtError> {
        if self.is_client()? {
            return Err(PtError::ClientOrPortUnavailable);
        }
        match self.try_extorport().await {
            Ok(()) => Ok(OrPortKind::ExtOrPort),
            Err(PtError::ExtOrPortNotAvailable(_)) => {
                // Try to connect to the OrPort
                if let Some(addr) = self.get_server_config()?.TOR_PT_ORPORT {
                    self.orport = Some(TcpStream::connect(addr).await?);
                    return Ok(OrPortKind::OrPort);
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

impl Pt {
    /// As of September 2026, the pt-spec does not state when the server
    /// should return `OKAY` or `DENY`.
    /// But in C-tor and goptlib, code shows that server will return `Okay` after sending `done`
    /// Therefore, this function is here
    ///
    /// # Errors
    /// - if tokio `write_all(...)` returns Error
    /// - if the stream is missing
    /// - [`ExtOrPortError::StreamMissing`] if the reader half is not set,
    ///   i.e. `connect()` has not completed successfully.
    /// - Any error returned by `PtTracing::info`, typically when
    ///   `PT_TRACING` is unset.
    /// - [`ExtOrPortError::Other`] if reading from the server fails,
    ///   including `UnexpectedEof` when the server closes the connection
    ///   mid-frame.
    /// - [`ExtOrPortError::Other`] if the command or length bytes cannot be
    ///   converted to `u16` / `ExtOrPortReply`.
    /// - the connection is never inited
    /// - the connection is not `ExtOrPort`
    pub async fn done_wait(&mut self) -> Result<(), PtError> {
        if let Some(extorport) = &mut self.extorport {
            ExtOrPort::done_wait(extorport).await?;
        } else {
            return Err(PtError::NotExtOrPort(
                "calling done requires an established ExtOrPort connection".to_string(),
            ));
        }
        Ok(())
    }

    /// Sends a `USERADDR` command to the `ExtOrPort` server, informing it of the
    /// TCP/IP address of the client connecting through the pluggable transport.
    ///
    /// The address MUST be in one of the following formats:
    ///   - `1.2.3.4:5678`
    ///   - `[1:2::3:4]:5678`
    ///
    /// Other formats MAY be accepted by current Tor versions, but transports
    /// MUST NOT send them.
    ///
    /// # Note
    /// User have to make sure the `addr` is valid
    /// # Errors
    /// - if tokio `write_all(...)` returns an Error.
    /// - [`PtError::NotExtOrPort`] if no `ExtOrPort` connection has been
    ///   established, i.e. the `extorport` field is `None`.
    /// - Any error returned by [`ExtOrPort::user_addr`], including
    ///   [`ExtOrPortError::InvalidUserAddr`] if the address fails to parse.
    pub async fn user_addr(&mut self, client_addr: String) -> Result<(), PtError> {
        if let Some(extorport) = &mut self.extorport
            && let Some(writer) = &mut extorport.writer
        {
            ExtOrPort::user_addr(writer, client_addr).await?;
        } else {
            return Err(PtError::NotExtOrPort(
                "calling user_addr requires an established ExtOrPort connection".to_string(),
            ));
        }
        Ok(())
    }

    /// Sends a `TRANSPORT` command to the `ExtOrPort` server, informing it of the
    /// name of the pluggable transport in use.
    ///
    /// # Errors
    /// - if tokio `write_all(...)` returns an Error.
    /// - [`PtError::NotExtOrPort`] if no `ExtOrPort` connection has been
    ///   established, i.e. the `extorport` field is `None`.
    /// - Any error returned by [`ExtOrPort::transport`], including
    ///   [`ExtOrPortError::PtNameTooLong`] if the transport name exceeds
    ///   `u16::MAX` bytes.
    pub async fn transport(&mut self, pt_name: PtTransportName) -> Result<(), PtError> {
        if let Some(extorport) = &mut self.extorport
            && let Some(writer) = &mut extorport.writer
        {
            ExtOrPort::transport(writer, pt_name).await?;
        } else {
            return Err(PtError::NotExtOrPort(
                "calling transport requires an established ExtOrPort connection".to_string(),
            ));
        }
        Ok(())
    }
    /// Sends the full `ExtOrPort` handshake for a new client connection:
    /// `TRANSPORT` → `USERADDR` → `DONE`/`OKAY`.
    ///
    /// This is the convenience wrapper most callers should use instead of
    /// calling [`Pt::transport`], [`Pt::user_addr`], and [`Pt::done_wait`]
    /// individually.
    ///
    /// # Errors
    /// - [`PtError::NotExtOrPort`] if no `ExtOrPort` connection has been
    ///   established.
    /// - Any error returned by [`Pt::transport`], including
    ///   [`ExtOrPortError::PtNameTooLong`] if `pt_name` exceeds `u16::MAX` bytes.
    /// - Any error returned by [`Pt::user_addr`], including
    ///   [`ExtOrPortError::InvalidUserAddr`] if `client_addr` fails to parse.
    /// - Any error returned by [`Pt::done_wait`], including
    ///   [`ExtOrPortError::StreamMissing`] if the reader half is not set, or
    ///   [`ExtOrPortError::Other`] on an I/O failure while awaiting `OKAY`.
    pub async fn finish(
        &mut self,
        pt_name: PtTransportName,
        client_addr: String,
    ) -> Result<(), PtError> {
        self.transport(pt_name).await?;
        self.user_addr(client_addr).await?;
        self.done_wait().await?;
        Ok(())
    }
}
