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
//! <!-- @@ end lint list
//! For detailed information, see [the spec] https://spec.torproject.org/pt-spec/ipc.html
use anyhow::Result;
use tracing::level_filters::LevelFilter;
use tracing::{debug, error, info, warn};
use tracing_subscriber::{self};
/// The SEVERITY value indicate at which logging level the message applies.
/// The accepted values for <Severity> are: error, warning, notice, info, debug
#[repr(u8)]
#[derive(Clone, Copy)]
pub enum SEVERITY {
    /// Sets the log level to **DEBUG**, displaying all messages with a severity of **DEBUG** or higher.
    DEBUG = 1,
    /// Sets the log level to **INFO**, displaying all messages with a severity of **INFO** or higher.
    INFO = 2,
    /// Sets the log level to **NOTICE**, displaying all messages with a severity of **NOTICE** or higher.
    NOTICE = 3,
    /// Sets the log level to **WARNING**, displaying all messages with a severity of **WARNING** or higher.
    WARNING = 4,
    /// Sets the log level to **ERROR**, displaying all messages with a severity of **ERROR** or higher.
    ERROR = 5,
}

/// Messages needed in Pt
/// TODO: do that bro!
pub enum PtMessages {}

impl From<SEVERITY> for LevelFilter {
    fn from(severity: SEVERITY) -> Self {
        match severity {
            SEVERITY::DEBUG => LevelFilter::DEBUG,
            SEVERITY::INFO => LevelFilter::INFO,
            // NOTICE: The tracing subscriber's max_level is set to WARN, so only WARN and ERROR
            // messages pass through. NOTICE-level messages are handled separately in `notice()`.
            SEVERITY::WARNING | SEVERITY::NOTICE => LevelFilter::WARN,
            SEVERITY::ERROR => LevelFilter::ERROR,
        }
    }
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
            .with_max_level(self.severity)
            .compact()
            .with_level(false)
            .with_target(false)
            .with_thread_ids(false)
            .with_thread_names(false)
            .with_file(false)
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
    /// Return a default PtTracingConfig
    pub fn default_config() -> Self {
        PtTracing::new(SEVERITY::NOTICE)
    }
    /// Init a default tracing_subscriber
    pub fn default_init() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        Self::default_config().init()?;
        Ok(())
    }
    /// Print a debug message, conform with Tor-Pt Spec
    pub fn debug(&self, message: String) -> Result<()> {
        debug!("LOG SEVERITY=debug MESSAGE={}", message);
        Ok(())
    }
    /// Print an info message, conform with Tor-Pt Spec
    pub fn info(&self, message: String) -> Result<()> {
        info!("LOG SEVERITY=info MESSAGE={}", message);
        Ok(())
    }

    /// Print an error message, conform with Tor-Pt Spec
    pub fn error(&self, message: String) -> Result<()> {
        error!("LOG SEVERITY=error MESSAGE={}", message);
        Ok(())
    }

    /// Print a notice message, conform with Tor-Pt Spec
    pub fn notice(&self, message: String) -> Result<()> {
        if self.severity as usize > 2 {
            println!("LOG SEVERITY=notice MESSAGE={}", message);
        }
        Ok(())
    }

    /// Print a warning message, conform with Tor-Pt Spec
    pub fn warn(&self, message: String) -> Result<()> {
        warn!("LOG SEVERITY=warning MESSAGE={}", message);
        Ok(())
    }
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
    //! <!-- @@ end test lint list maintained by maint/add_warning @@ -->
    #![allow(unused)]
    use super::*;

    #[test]
    fn test_something() {
        todo!("remember add some test wkwkwkwwk");
    }
}
