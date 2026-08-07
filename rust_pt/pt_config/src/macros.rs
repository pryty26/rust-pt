use derive_deftly::define_derive_deftly;
define_derive_deftly! {
    /// Implement to string via iterate the enum
    /// # Example
    /// ```rust
    /// pub use derive_deftly::{define_derive_deftly, Deftly};
    /// pub use pt_config::derive_deftly_template_FromString;
    /// use anyhow::Result;
    /// use std::str::FromStr;
    /// #[repr(u8)]
    /// #[derive(Debug, Clone, Copy, Deftly)]
    /// #[derive_deftly(FromString)]
    /// enum Ex {
    ///     A,
    ///     B,
    /// }
    ///
    /// fn main() -> Result<()> {
    ///     let x = Ex::try_from("A".to_string()).unwrap();
    ///     match x {
    ///         Ex::A => println!("success"),
    ///         _ => panic!("try_from failed")
    ///     }
    ///
    ///     let y = Ex::from_str("A").unwrap();
    ///     match y {
    ///         Ex::A => println!("success"),
    ///         _ => panic!("from_str failed")
    ///     }
    ///
    ///     let z = String::from(Ex::A);
    ///     let z_excepted = "Ex::A".to_string();
    ///     match z {
    ///         z_excepted => println!("success"),
    ///         _ => panic!("String::from failed")
    ///     }
    ///     Ok(())
    /// }
    /// ```
    export FromString for enum:
    impl From<$ttype> for String {
        fn from(value: $ttype) -> Self {
            match value {
                $(
                    $vpat => stringify!($vpat).to_string(),
                )
            }
        }
    }
    impl TryFrom<String> for $ttype {
        type Error = pt_err::VariantError;
        fn try_from(value: String) -> Result<Self, Self::Error> {
            match value.as_str() {
                $(
                    stringify!($vname) => Ok($vpat),
                )
                _ => { return Err(pt_err::VariantError::UnfoundError {
                    message: "".to_string()
                });
            }
            }
        }
    }
    impl std::str::FromStr for $ttype {
        type Err = pt_err::VariantError;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            match value {
                $(
                    stringify!($vname) => Ok($vpat),
                )
                _ => { return Err(pt_err::VariantError::UnfoundError {
                    message: "".to_string()
                });
            }
            }
        }
    }
}

define_derive_deftly! {
    /// The enum values must be consecutive integers starting from 0.
    /// # Example
    /// ```rust
    /// pub use derive_deftly::{define_derive_deftly, Deftly};
    /// pub use pt_config::derive_deftly_template_FromU8Index;
    /// pub use anyhow::Result;
    /// #[repr(u8)]
    /// #[derive(Debug, Clone, Copy, Deftly)]
    /// #[derive_deftly(FromU8Index)]
    /// enum Ex {
    ///     A = 0,
    ///     B = 1,
    /// }
    ///
    /// fn main() -> Result<()> {
    ///     let x = Ex::from_u8_index(0 as usize).unwrap();
    ///     match x {
    ///         Ex::A => println!("success"),
    ///         _ => panic!("Ex::from_u8_index failed")
    ///     }
    ///     let y = Ex::B.get_index().unwrap();
    ///     match y {
    ///         1 => println!("get_index() success"),
    ///         err => panic!("get_index() failed: {}", err),
    ///     }
    ///     Ok(())
    /// }
    /// ```
    /// would build ->
    /// impl Ex {
    ///     fn from_u8_index(v: usize) -> Result<Self, pt_err::VariantError> {
    ///         match v {
    ///             0 => Ex::A,
    ///             1 => Ex::B,
    ///             _ => {
    ///                 return Err(pt_err::VariantError::UnfoundError {
    ///                     message: "".to_string()
    ///                 });
    ///             }
    ///         }
    ///     }
    /// }
    ///
    ///
    /// TODO: I use $vindex due $v_explicit_discriminant is not stable yet
    export FromU8Index for enum:
    impl $ttype {
        fn from_u8_index(v: usize) -> Result<Self, pt_err::VariantError> {
            match v {
                $(
                    $vindex => Ok($vpat),
                )
                _ => { return Err(pt_err::VariantError::UnfoundError {
                    message: "".to_string()
                });
            }
            }
        }
        fn get_index(&self) -> Result<usize, pt_err::VariantError> {
            match self {
                $(
                    $vpat => Ok($vindex),
                )
                _ => { return Err(pt_err::VariantError::UnfoundError {
                    message: "".to_string()
                });
            }
            }
        }
    }

}
