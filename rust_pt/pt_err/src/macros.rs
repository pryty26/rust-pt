use derive_deftly::define_derive_deftly;

define_derive_deftly! {
    /// Macro which will automatically create Invalid$variant and $variantUnfound
    ///  for every variant or fields
    /// It helps user to match errros.
    /// # Example
    /// ```rust
    /// use std::fmt::Display;
    /// use thiserror::*; 
    /// pub use derive_deftly::{define_derive_deftly, Deftly};
    /// pub use pt_err::derive_deftly_template_DefineVariantError;
    /// pub use anyhow::Result;
    /// #[derive(Debug, Clone, Deftly)]
    /// #[derive_deftly(DefineVariantError)]
    /// struct Ex2 {
    ///     A: String,
    /// }
    /// fn main() -> Result<()> {
    ///     let e = Ex2ConfigError::InvalidA {
    ///         message: "Very cool".to_string()
    ///     };
    ///     println!("{:?}", e);
    ///     Ok(())
    /// }
    /// ```
    /// Would derive:
    /// pub enum Ex2ConfigError {
    ///     ConfigError { message: String },
    ///     InvalidA { message: String },
    ///     AUnfound { message: String },
    /// }
    export DefineVariantError:
    ${if is_enum {
        /// Errors which could cause during the Config parsing.
        #[derive(Debug, Error, PartialEq)]
        pub enum $<$tname ConfigError> {
            #[error("Config Error:{message}")]
            ConfigError {
                message: String
            }
            $(
            /// Configuration error with additional context.
            #[error("Invalid config:{message}")]
                $< Invalid $vname > {
                    message: String
                },
            #[error("Config does not exist: {message}")]
            $< $vname Unfound > {
                message: String,
            }
            )
        }
    } is_struct {
        /// Error for new structure
        #[derive(Debug, thiserror::Error, PartialEq)]
        pub enum $<$tname ConfigError> {
            /// Some not specified error
            #[error("Config Error:{message}")]
            ConfigError {
                /// Additional context
                message: String
            },
            $(
            /// Invalid config
            #[error("Invalid config:{message}")]
                $< Invalid $fname > {
                    /// Additional context
                    message: String
                },
            /// Config does not exist
            #[error("Config does not exist: {message}")]
                $< $fname Unfound > {
                    /// Additional context
                    message: String,
                },
            )
        }
    }}
}