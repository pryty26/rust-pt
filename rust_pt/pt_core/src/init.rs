// File: rust_pt\pt_core\src\init.rs
// Directory: rust_pt\pt_core\src
// Filename: init.rs
//======================================================================

use crate::{OrPortKind, Pt};
use crate::orport::extorport::ExtOrPort;
use anyhow::Result;
use pt_config::prelude::*;
use pt_err::PtError;
use pt_tracing::prelude::*;
use tokio::net::TcpStream;

impl Pt {
    /// Init the Pt
    /// Including configs, and Logs
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