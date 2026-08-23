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
#![deny(clippy::ref_option_ref)]
#![deny(clippy::string_slice)] // See arti#2571
#![deny(clippy::unchecked_time_subtraction)]
#![deny(clippy::unnecessary_wraps)]
#![deny(clippy::unused_async)]
#![deny(clippy::unwrap_used)]
//! <!-- @@ end lint list
#![allow(clippy::print_stderr)]
#![allow(clippy::print_stdout)]
//! For detailed information, see [the spec] https://spec.torproject.org/pt-spec/ipc.html
use anyhow::{Result, anyhow, bail};
use std::sync::OnceLock;
use tracing::level_filters::LevelFilter;
use tracing::{debug, error, info, warn};
use tracing_subscriber::{self};

/// The SEVERITY value indicate at which logging level the message applies.
/// The accepted values for <Severity> are: error, warning, notice, info, debug
#[repr(u8)]
#[derive(Clone, Copy)]
#[non_exhaustive]
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

/// pt_tracing stores the config of pt_tracing
pub static PT_TRACING: OnceLock<PtTracing> = OnceLock::new();

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

/// Configuration for pt_tracing.
///
/// Users can customize the logging behavior via different functions.
///
/// # Example
///
/// ```rust
/// use pt_tracing::{PtTracing, SEVERITY};
///
/// fn main() -> anyhow::Result<()> {
///    // Examples of avaiable value in 2026/8/21
///    let (_debug, _info, _notice, _warning, _error) = (
///         SEVERITY::DEBUG,
///         SEVERITY::INFO,
///         SEVERITY::NOTICE,
///         SEVERITY::WARNING,
///         SEVERITY::ERROR,
///     );
///     let level = SEVERITY::NOTICE;
///     PtTracing::fmt()
///         .with_severity(level);
///     // Yes I used Result in every DEBUG, INFO, NOTICE, WARNING, ERROR function
///     PtTracing::debug("cool debug message")?;
///     PtTracing::info("cool info message")?;
///     // And so on
///     Ok(())
/// }
/// ```
pub struct PtTracing {
    /// The SEVERITY value indicate at which logging level the message applies.
    /// The accepted values for <Severity> are: error, warning, notice, info, debug
    pub severity: SEVERITY,
}

/// Functions of PtTracing's config
impl PtTracing {
    /// Get config From PT_TRACING
    pub fn get_config() -> &'static Self {
        match PT_TRACING.get() {
            None => {
                println!("LOG SEVERITY=error MESSAGE=\"pt_tracing is never inited\"");
                panic!("pt_tracing is never inited")
            },
            Some(pt_tracing) => {
                return pt_tracing;
            },
        }
    }
    /// init the tracing config with default severity which is NOTICE
    pub fn fmt() -> Self {
        PtTracing {
            severity: SEVERITY::NOTICE,
        }
    }
    /// change the SEVERITY of config
    pub fn with_severity(mut self, severity: SEVERITY) -> Self {
        self.severity = severity;
        self
    }

    /// init the tracing config
    /// This is decided as a sealed function,
    /// because we don't want to let user call directly the init(...),
    /// since our structure is still unstable
    /// TODO: add more flexible configuration
    pub fn try_init(self) -> Result<()> {
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
            .try_init()
            .map_err(|e| anyhow::anyhow!(e))?;

        match PT_TRACING.set(self) {
            Err(_) => {
                println!("LOG SEVERITY=error MESSAGE=\"pt_tracing already inited\"");
                bail!("pt_tracing already inited")
            },
            Ok(_) => {
                PtTracing::notice("PT_TRACING is set")?;
            },
        }
        Ok(())
    }
}

impl PtTracing {
    /// Print a debug message, conform with Tor-Pt Spec
    pub fn debug(message: &str) -> Result<()> {
        debug!("LOG SEVERITY=debug MESSAGE=\"{}\"", message);
        Ok(())
    }
    /// Print an info message, conform with Tor-Pt Spec
    pub fn info(message: &str) -> Result<()> {
        info!("LOG SEVERITY=info MESSAGE=\"{}\"", message);
        Ok(())
    }

    /// Print an error message, conform with Tor-Pt Spec
    pub fn error(message: &str) -> Result<()> {
        error!("LOG SEVERITY=error MESSAGE=\"{}\"", message);
        Ok(())
    }

    /// Print a notice message, conform with Tor-Pt Spec
    /// Unfortunately tracing do not have "notice" level. So we need to use manual if+println! instead.
    pub fn notice(message: &str) -> Result<()> {
        // Severity enum values:
        // Error = 0, Warn = 1, Notice = 2, Info = 3, Debug = 4, Trace = 5
        // Notice messages should be printed when severity is DEBUG(1)、INFO(2)、NOTICE(3)
        match PT_TRACING
            .get()
            .ok_or_else(|| {
                println!("LOG SEVERITY=error MESSAGE=\"pt_tracing is never inited\"");
                anyhow!("pt_tracing is never inited")
            })?
            .severity
        {
            SEVERITY::DEBUG | SEVERITY::INFO | SEVERITY::NOTICE => {
                // NOTE: we used warn!(..) in notice, but it's acceptable
                // since only SEVERITY::DEBUG | SEVERITY::INFO | SEVERITY::NOTICE will entry this branch
                warn!("LOG SEVERITY=notice MESSAGE=\"{}\"", message);
            },
            SEVERITY::WARNING | SEVERITY::ERROR => {},
        }
        Ok(())
    }

    /// Print a warning message, conform with Tor-Pt Spec
    pub fn warn(message: &str) -> Result<()> {
        warn!("LOG SEVERITY=warning MESSAGE=\"{}\"", message);
        Ok(())
    }
    /// After version negotiation has been completed, the PT proxy must then
    /// validate that all of the required environment variables are provided,
    /// and that all of the configuration values supplied are well formed.
    ///
    /// At any point, if there is an error encountered related to configuration
    /// supplied via the environment variables, it MAY respond with an error
    /// message and terminate.
    ///
    /// `ENV-ERROR <ErrorMessage>`
    ///
    /// The “ENV-ERROR” message is used to signal the PT proxy’s failure to parse
    /// the configuration environment variables (3.2).
    ///
    /// The `<ErrorMessage>` SHOULD consist of a useful error message that can be
    /// used to diagnose and correct the root cause of the failure.
    ///
    /// PT proxies MUST terminate after outputting a “ENV-ERROR” message.
    ///
    /// Example:
    ///
    /// `ENV-ERROR No TOR_PT_AUTH_COOKIE_FILE when TOR_PT_EXTENDED_SERVER_PORT set`
    pub fn env_error(msg: &str) {
        println!("ENV-ERROR {}", msg);
    }
    /// When a PT proxy first starts up, it must determine which version of the
    /// Pluggable Transports Specification to use to configure itself.
    ///
    /// It does this via the `TOR_PT_MANAGED_TRANSPORT_VER` (3.2.1) environment
    /// variable which contains all of the versions supported by the application.
    ///
    /// Upon determining the version to use, or lack thereof, the PT proxy
    /// responds with one of two messages.
    ///
    /// `VERSION-ERROR <ErrorMessage>`
    ///
    /// The “VERSION-ERROR” message is used to signal that there was no compatible
    /// Pluggable Transport Specification version present in the
    /// `TOR_PT_MANAGED_TRANSPORT_VER` list.
    ///
    /// The `<ErrorMessage>` SHOULD be set to “no-version” for historical reasons
    /// but MAY be set to a useful error message instead.
    ///
    /// PT proxies MUST terminate after outputting a “VERSION-ERROR” message.
    pub fn version_error(msg: &str) {
        println!("VERSION-ERROR {}", &msg)
    }
    /// After negotiating the Pluggable Transport Specification version, PT client
    /// proxies MUST first validate `TOR_PT_PROXY` (3.2.2) if it is set, before
    /// initializing any transports.
    ///
    /// Assuming that an upstream proxy is provided, PT client proxies MUST
    /// respond with a message indicating that the proxy is valid, supported, and
    /// will be used OR a failure message.
    pub fn proxy_done() {
        println!("PROXY DONE")
    }
    /// The `VERSION` message is used to signal the Pluggable Transport
    /// Specification version that the PT proxy will use to configure its
    /// transports and communicate with the parent process.
    ///
    /// The version for the environment values and reply messages specified
    /// by the PT Specification is `1`.
    ///
    /// PT proxies MUST either report an error and terminate, or output a
    /// `VERSION` message before moving on to client/server proxy initialization
    /// and configuration.
    ///
    /// Example:
    ///
    /// `VERSION 1`
    pub fn version(version: &str) {
        println!("VERSION {}", version);
    }

    /// The `PROXY-ERROR` message is used to signal that the upstream proxy
    /// specified by `TOR_PT_PROXY` is malformed, unsupported, or otherwise
    /// unusable.
    ///
    /// PT proxies MUST terminate immediately after outputting a
    /// `PROXY-ERROR` message.
    ///
    /// Example:
    ///
    /// `PROXY-ERROR SOCKS 4 upstream proxies unsupported.`
    pub fn proxy_error(msg: &str) {
        println!("PROXY-ERROR {}", msg);
    }

    /// The `CMETHOD` message is used to signal that a requested PT transport
    /// has been launched, the protocol which the parent should use to make
    /// outgoing connections, and the IP address and port that the PT transport
    /// is listening on.
    ///
    /// The protocol MUST be either `socks4` or `socks5`.
    ///
    /// Example:
    ///
    /// `CMETHOD trebuchet socks5 127.0.0.1:19999`
    pub fn cmethod(transport: &str, proxy_type: &str, address: &str) {
        println!("CMETHOD {} {} {}", transport, proxy_type, address);
    }

    /// The `CMETHOD-ERROR` message is used to signal that a requested PT
    /// transport was unable to be launched.
    ///
    /// Example:
    ///
    /// `CMETHOD-ERROR trebuchet no rocks available`
    pub fn cmethod_error(transport: &str, msg: &str) {
        println!("CMETHOD-ERROR {} {}", transport, msg);
    }

    /// The `CMETHODS DONE` message signals that the PT proxy has finished
    /// initializing all of the transports that it is capable of handling.
    ///
    /// Upon sending the `CMETHODS DONE` message, the PT proxy initialization
    /// is complete.
    pub fn cmethods_done() {
        println!("CMETHODS DONE");
    }

    /// The `SMETHOD` message is used to signal that a requested PT transport
    /// has been launched, the protocol which will be used to handle incoming
    /// connections, and the IP address and port that clients should use to
    /// reach the reverse proxy.
    ///
    /// The optional `options` field is used to pass additional per-transport
    /// information back to the parent process.
    ///
    /// Example:
    ///
    /// `SMETHOD trebuchet 198.51.100.1:19999`
    ///
    /// `SMETHOD rot_by_N 198.51.100.1:2323 ARGS:N=13`
    pub fn smethod(transport: &str, address: &str, options: Option<&str>) {
        match options {
            Some(options) => {
                println!("SMETHOD {} {} {}", transport, address, options);
            },
            None => {
                println!("SMETHOD {} {}", transport, address);
            },
        }
    }

    /// The `SMETHOD-ERROR` message is used to signal that a requested PT
    /// transport reverse proxy was unable to be launched.
    ///
    /// Example:
    ///
    /// `SMETHOD-ERROR trebuchet no cows available`
    pub fn smethod_error(transport: &str, msg: &str) {
        println!("SMETHOD-ERROR {} {}", transport, msg);
    }
    /// The `SMETHODS DONE` message signals that the PT proxy has finished
    /// initializing all of the transports that it is capable of handling.
    ///
    /// Upon sending the `SMETHODS DONE` message, the PT proxy initialization
    /// is complete.
    pub fn smethods_done() {
        println!("SMETHODS DONE");
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

    ///! wkwkwk I just copy-pasted pt-spec
    impl PtTracing {
        /// Print a notice message, conform with Tor-Pt Spec
        /// Unfortunately tracing do not have "notice" level. So we need to use manual if+println! instead.
        /// This is only for test
        pub fn notice_for_test(message: &str) -> Result<()> {
            // Severity enum values:
            // Error = 0, Warn = 1, Notice = 2, Info = 3, Debug = 4, Trace = 5
            // Notice messages should be printed when severity is DEBUG(1)、INFO(2)、NOTICE(3)
            match Self::get_config().severity {
                SEVERITY::DEBUG | SEVERITY::INFO | SEVERITY::NOTICE => {
                    println!("LOG SEVERITY=notice MESSAGE={}", message);
                },
                SEVERITY::WARNING | SEVERITY::ERROR => {
                    anyhow::bail!("error!");
                },
            }
            Ok(())
        }
    }
    #[test]
    fn test_for_notice() -> anyhow::Result<()> {
        PtTracing::fmt().with_severity(SEVERITY::DEBUG).try_init();
        PtTracing::notice_for_test("notice").unwrap();
        Ok(())
    }
}
