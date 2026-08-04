// @@ begin lint list
#![allow(renamed_and_removed_lints)] // @@REMOVE_WHEN(ci_arti_stable)
#![allow(unknown_lints)] // @@REMOVE_WHEN(ci_arti_nightly)
#![warn(missing_docs)]
#![warn(noop_method_call)]
#![warn(unreachable_pub)]
#![warn(clippy::all)]
#![deny(clippy::await_holding_lock)]
#![deny(clippy::cargo_common_metadata)]
#![deny(clippy::cast_lossless)]
#![deny(clippy::checked_conversions)]
#![allow(clippy::cognitive_complexity)] // See arti#2556
#![deny(clippy::debug_assert_with_mut_call)]
#![deny(clippy::exhaustive_enums)]
#![deny(clippy::exhaustive_structs)]
#![deny(clippy::expl_impl_clone_on_copy)]
#![deny(clippy::fallible_impl_from)]
#![deny(clippy::implicit_clone)]
#![deny(clippy::large_stack_arrays)]
#![warn(clippy::manual_ok_or)]
#![deny(clippy::missing_docs_in_private_items)]
#![warn(clippy::needless_borrow)]
#![warn(clippy::needless_pass_by_value)]
#![warn(clippy::option_option)]
#![deny(clippy::print_stderr)]
#![deny(clippy::print_stdout)]
#![warn(clippy::rc_buffer)]
#![deny(clippy::ref_option_ref)]
#![warn(clippy::semicolon_if_nothing_returned)]
#![warn(clippy::trait_duplication_in_bounds)]
#![deny(clippy::unchecked_time_subtraction)]
#![deny(clippy::unnecessary_wraps)]
#![warn(clippy::unseparated_literal_suffix)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::mod_module_files)]
#![allow(clippy::let_unit_value)] // This can reasonably be done for explicitness
#![allow(clippy::uninlined_format_args)]
#![allow(clippy::significant_drop_in_scrutinee)] // arti/-/merge_requests/588/#note_2812945
#![allow(clippy::result_large_err)] // temporary workaround for arti#587
#![allow(clippy::needless_raw_string_hashes)] // complained-about code is fine, often best
#![allow(clippy::needless_lifetimes)] // See arti#1765
#![allow(mismatched_lifetime_syntaxes)] // temporary workaround for arti#2060
#![allow(clippy::collapsible_if)] // See arti#2342
#![deny(clippy::unused_async)]
#![deny(clippy::string_slice)] // See arti#2571
//! <!-- @@ end lint list
//! For detailed information, see [the spec] https://spec.torproject.org/pt-spec/ipc.html
use anyhow::Result;
use tracing::{debug, error, info, warn};
use tracing_subscriber;
/// The SEVERITY value indicate at which logging level the message applies.
/// The accepted values for <Severity> are: error, warning, notice, info, debug
#[repr(u8)]
#[derive(Clone, Copy)]
pub(crate) enum SEVERITY {
    DEBUG = 1,
    LOG = 2,
    NOTICE = 3,
    WARNING = 4,
    ERROR = 5,
}
/// Config of pt_tracing,
/// user could change it via different function
pub struct PtTracing {
    /// The SEVERITY value indicate at which logging level the message applies.
    /// The accepted values for <Severity> are: error, warning, notice, info, debug
    pub severity: SEVERITY,
}
impl PtTracing {
    /// init the config with PtTracingConfig
    /// TODO: add more flexible configuration
    pub fn init(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        tracing_subscriber::fmt()
            .compact()
            .with_level(false)
            .with_target(false)
            .with_thread_ids(false)
            .with_thread_names(false)
            .with_file(false)
            .with_line_number(false)
            .with_line_number(false)
            .with_ansi(false)
            .without_time()
            .try_init()?;
        Ok(())
    }
    /// Return a new PtTracingConfig
    pub fn new(severity: SEVERITY) -> Self {
        PtTracing { severity: severity }
    }
    /// Print a debug message, conform with Tor-Pt Spec
    pub fn debug(&self, message: String) -> Result<()> {
        debug!("LOG SEVERITY=debug MESSAGE={}", message);
        Ok(())
    }
    /// Print an info message, conform with Tor-Pt Spec
    pub fn info(&self, message: String) -> Result<()> {
        info!("LOG SEVERITY=error MESSAGE={}", message);
        Ok(())
    }

    /// Print an error message, conform with Tor-Pt Spec
    pub fn error(&self, message: String) -> Result<()> {
        error!("LOG SEVERITY=error MESSAGE={}", message);
        Ok(())
    }

    /// Print a notice message, conform with Tor-Pt Spec
    pub fn notice(&self, message: String) -> Result<()> {
        if self.severity {}
        println!("LOG SEVERITY=notice MESSAGE={}", message);
        Ok(())
    }

    /// Print a warning message, conform with Tor-Pt Spec
    pub fn warn(&self, message: String) -> Result<()> {
        warn!("LOG SEVERITY=warn MESSAGE={}", message);
        Ok(())
    }
}
