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
#[doc(hidden)]
pub use derive_deftly;
use thiserror::Error;
/// Diffrent macros for Error
mod macros;

/// Errors which could cause during the Config parsing.
#[derive(Debug, Error, PartialEq)]
pub enum ConfigError {
    /// Configuration error with additional context.
    /// # Example
    /// ```rust
    /// use pt_err::ConfigError;
    /// fn main() {
    ///     assert_eq!(ConfigError::InvalidConfigErr {
    ///         side: "client".to_string(),
    ///         message:"I want to eat radishes".to_string()
    ///     }.to_string(),
    ///     "Invalid client Config: I want to eat radishes")
    /// }
    /// ```
    #[error("Invalid {side} Config: {message}")]
    InvalidConfigErr {
        /// Which side the pt is running
        /// could be Server, Client or Unknown
        side: String,
        /// Additional context of the Error
        message: String,
    },
    /// Configuration error caused by unsupported version
    #[error("Unsupported Version {message}")]
    UnsupportedVer {
        /// Additional context of the Error
        message: String,
    },
}
/// Errors which could cause by variant
#[derive(Debug, Error, PartialEq)]
pub enum VariantError {
    /// Error indicates that the Variant hasn't found
    /// # Example
    /// ```rust
    /// use pt_err::VariantError;
    /// fn main() {
    ///     let _ = VariantError::UnfoundError {message:"I want to eat radishes".to_string()};
    /// }
    /// ```
    #[error("Variant Unfound {message}")]
    UnfoundError {
        /// Additional context of the Error
        message: String,
    },
}
