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

/// Implementation for Extended and Normal `ORPort` for pluggable transports
pub mod orport;
/// Variables
pub mod variables;
use derive_deftly::Deftly;
use pt_config::derive_deftly_template_Builder;
use pt_config::prelude::*;
use pt_err::PtError;
use pt_tracing::prelude::*;

use crate::orport::extorport::ExtOrPort;
/// Init the Pt
pub mod init;

/// To use all the traits
pub mod prelude {
    pub use crate::orport::traits::*;
}

/// The core config for PT
/// User have to call the builder to build that
/// ```rust
/// use pt_core::Pt;
/// use pt_tracing::{prelude::*};
/// fn main() {
///     let pt = Pt::builder()
///         .with_severity(SEVERITY::INFO);
///     // Then, user can call try_init(),
///     // but since we are not setting the env config in docs test,
///     // so just leave it for now
///     // pt.try_init();
/// }
///
/// ```
#[derive(Deftly)]
#[derive_deftly(Builder)]
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
}

impl Pt {
    /// get self
    /// # Errors
    /// if Pt is not initialized
    pub(crate) fn get(&self) -> Result<&ConfigKey, PtError> {
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
}
