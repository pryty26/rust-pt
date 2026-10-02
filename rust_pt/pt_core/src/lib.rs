// File: rust_pt\pt_core\src\lib.rs
// Directory: rust_pt\pt_core\src
// Filename: lib.rs
//======================================================================

// @@ begin lint list
#![allow(renamed_and_removed_lints)] // @@REMOVE_WHEN(ci_arti_stable)
#![allow(unknown_lints)] // @@REMOVE_WHEN(ci_arti_nightly)
#![allow(clippy::cognitive_complexity)] // See arti#2556
#![allow(clippy::collapsible_if)] // See arti#2342
#![allow(clippy::let_unit_value)] // This can reasonably be done for explicitness
#![allow(clippy::needless_lifetimes)] // See arti#1765
#![allow(clippy::needless_raw_string_hashes)] // complained-about code is fine, often best
#![allow(clippy::result_large_err)] // temporary workaround for arti#587
#![allow(clippy::significant_drop_in_scrutinee)] // arti/-/merge_requests/588/#note_2812945
#![allow(clippy::uninlined_format_args)]
#![allow(mismatched_lifetime_syntaxes)] // temporary workaround for arti#2060
#![warn(missing_docs)]
#![warn(noop_method_call)]
#![warn(unreachable_pub)]
#![warn(clippy::all)]
#![warn(clippy::manual_ok_or)]
#![warn(clippy::needless_borrow)]
#![warn(clippy::needless_pass_by_value)]
#![warn(clippy::option_option)]
#![warn(clippy::rc_buffer)]
#![warn(clippy::semicolon_if_nothing_returned)]
#![warn(clippy::trait_duplication_in_bounds)]
#![warn(clippy::unseparated_literal_suffix)]
#![deny(clippy::await_holding_lock)]
#![deny(clippy::cargo_common_metadata)]
#![deny(clippy::cast_lossless)]
#![deny(clippy::checked_conversions)]
#![deny(clippy::debug_assert_with_mut_call)]
#![deny(clippy::exhaustive_enums)]
#![deny(clippy::exhaustive_structs)]
#![deny(clippy::expl_impl_clone_on_copy)]
#![deny(clippy::fallible_impl_from)]
#![deny(clippy::implicit_clone)]
#![deny(clippy::large_stack_arrays)]
#![deny(clippy::missing_docs_in_private_items)]
#![deny(clippy::mod_module_files)]
#![deny(clippy::print_stderr)]
#![deny(clippy::print_stdout)]
#![deny(clippy::ref_option_ref)]
#![deny(clippy::string_slice)] // See arti#2571
#![deny(clippy::unchecked_time_subtraction)]
#![deny(clippy::unnecessary_wraps)]
#![deny(clippy::unused_async)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::pedantic)] // This is not in Arti
//! <!-- @@ end lint list
use pt_config::configs::keys::PtTransportName;
/// Implementation for Extended and Normal `ORPort` for pluggable transports
pub mod orport;
/// Variables
pub mod variables;
use crate::orport::extorport::ExtOrPort;
use derive_deftly::Deftly;
use pt_config::configs::keys::{ClientKey, CommonKey, ServerKey};
use pt_config::derive_deftly_template_Builder;
use pt_config::prelude::*;
use pt_err::PtError;
use pt_tracing::prelude::*;
use tokio::net::TcpStream;
/// Init the Pt
pub mod init;

/// To use all the traits
pub mod prelude {
    pub use crate::Pt;
    pub use crate::orport::traits::*;
}
use std::collections::HashSet;
/// The core config for PT
/// User have to call the builder to build that
///
/// ```rust
/// use pt_core::Pt;
/// use pt_tracing::{prelude::*};
/// fn main() {
///     let pt = Pt::builder()
///         .with_severity(SEVERITY::INFO);
///     // Note that our builder do not have `build` func
///     // Then, user can call try_init(),
///     // but since we are not setting the env config in docs test,
///     // so just leave it for now
///     // pt.try_init();
///     // pt.connect_or();
/// }
///
/// ```
#[derive(Deftly)]
#[derive_deftly(Builder)] // See [`rust-pt\rust_pt\pt_config\src\macros.rs`]
#[non_exhaustive]
pub struct Pt {
    /// The log severity
    #[deftly(default = "SEVERITY::NOTICE")]
    pub severity: SEVERITY,
    /// The Config key, user must not call `with_config_key(...)` to change that
    /// User must call `try_init()` to get the Config Key
    #[deftly(default = "None")]
    pub config_key: Option<ConfigKey>,
    /// Optional `ExtOrPort` connection
    /// user must not call `with_extorport` to set that
    /// please call `Pt.try_extorport()`
    #[deftly(default = "None")]
    pub extorport: Option<ExtOrPort>,
    /// Optional `OrPort` connection,
    /// there should not have a `OrPort` connection,
    /// if there is already a `ExtOrPort` connection
    #[deftly(default = "None")]
    pub orport: Option<TcpStream>,
    /// A field used to automatically filter out unsupported transports
    /// Leaving for empty means skip
    #[deftly(default = "None")]
    pub sup_transports: Option<Vec<PtTransportName>>,
    /// A field used to automatically filter out unsupported proxy url
    /// Leaving for empty means skip
    #[deftly(default = "None")]
    pub proxy_schemes: Option<HashSet<String>>,
}

impl Pt {
    /// get the config
    /// # Errors
    /// if Pt is not initialized
    pub fn get(&self) -> Result<&ConfigKey, PtError> {
        self.config_key
            .as_ref()
            .ok_or_else(|| PtError::PtNotInitialized("Please call Pt.try_init()".to_string()))
    }
    /// Check whether config key of Pt is `ServerKey`
    /// # Errors
    /// if Pt is not initialized
    pub fn is_server(&self) -> Result<bool, PtError> {
        Ok(self.get()?.is_server())
    }
    /// Check whether config key of Pt is `ClientKey`
    /// # Errors
    /// if Pt is not initialized
    pub fn is_client(&self) -> Result<bool, PtError> {
        Ok(self.get()?.is_client())
    }
    /// Get the config, and ensure that it is `ServerConfig`.
    /// Only return the `server_config` part
    /// # Errors
    /// if Pt is not initialized, or config is not for `server`
    pub fn get_server_config(&self) -> Result<&ServerKey, PtError> {
        match self.get()? {
            ConfigKey::Server { server_key, .. } => Ok(server_key),
            ConfigKey::Client { .. } => {
                Err(PtError::NotInServer("get_server_config error".to_string()))
            },
        }
    }
    /// Get the config, and ensure that it is `ClientConfig`.
    /// Only return the `client_key` part.
    ///
    /// # Errors
    ///
    /// Returns an error
    /// - `Pt` is not initialized
    /// - the config is not for a client.
    pub fn get_client_config(&self) -> Result<&ClientKey, PtError> {
        match self.get()? {
            ConfigKey::Client { client_key, .. } => Ok(client_key),
            ConfigKey::Server { .. } => {
                Err(PtError::NotInClient("get_client_config error".to_string()))
            },
        }
    }
    /// get the config as mutable
    /// # Errors
    /// if Pt is not initialized
    pub fn get_mut(&mut self) -> Result<&mut ConfigKey, PtError> {
        self.config_key
            .as_mut()
            .ok_or_else(|| PtError::PtNotInitialized("Please call Pt.try_init()".to_string()))
    }
    /// Get the config as mutable, and ensure that it is `ClientConfig`.
    /// Only return the `client_key` part.
    ///
    /// # Errors
    ///
    /// Returns an error
    /// - `Pt` is not initialized
    /// - the config is not for a client.
    pub fn get_client_config_mut(&mut self) -> Result<&mut ClientKey, PtError> {
        match self.get_mut()? {
            ConfigKey::Client { client_key, .. } => Ok(client_key),
            ConfigKey::Server { .. } => Err(PtError::NotInClient(
                "get_client_config_mut error".to_string(),
            )),
        }
    }

    /// Get the config as mutable, and ensure that it is `ServerConfig`.
    /// Only return the `server_key` part.
    ///
    /// # Errors
    ///
    /// Returns an error
    /// - `Pt` is not initialized
    /// - the config is not for a server.
    pub fn get_server_config_mut(&mut self) -> Result<&mut ServerKey, PtError> {
        match self.get_mut()? {
            ConfigKey::Server { server_key, .. } => Ok(server_key),
            ConfigKey::Client { .. } => Err(PtError::NotInServer(
                "get_server_config_mut error".to_string(),
            )),
        }
    }
    /// Get the common config of the config
    /// # Errors
    ///
    /// Returns an error
    /// - `Pt` is not initialized
    pub fn get_common_config(&self) -> Result<&CommonKey, PtError> {
        Ok(self.get()?.common_key())
    }
    /// Check if the connection is `ExtOrPort`
    pub fn is_extorport(&self) -> bool {
        self.extorport.is_some()
    }
    /// Check if the connection is `OrPort`
    pub fn is_orport(&self) -> bool {
        self.orport.is_some()
    }
}

/// Which control connection was successfully established.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(clippy::exhaustive_enums)] // Unless pt-spec changes, this will not change
pub enum OrPortKind {
    /// Extended `ORPort` (the preferred path).
    ExtOrPort,
    /// Plain `ORPort` (fallback when `ExtOrPort` is unavailable).
    OrPort,
}
