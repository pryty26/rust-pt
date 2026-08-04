use derive_deftly::define_derive_deftly;
define_derive_deftly! {
    /// Implement to string via iterate the enum
    /// O
    export FromString for enum:
    impl From<$ttype> for String {
        fn from(value: $ttype) -> Self {
            match value {
                $(
                    $vpat => stringify!($vtype).to_string(),
                )
            }
        }
    }
    impl TryFrom<String> for $ttype {
        type Error = pt_err::VariantError;
        fn try_from(value: String) -> Result<Self, Self::Error> {
            match value.as_str() {
                $(
                    stringify!($vtype) => Ok($vpat),
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
                    stringify!($vtype) => Ok($vpat),
                )
                _ => { return Err(pt_err::VariantError::UnfoundError {
                    message: "".to_string()
                });
            }
            }
        }
    }
}
