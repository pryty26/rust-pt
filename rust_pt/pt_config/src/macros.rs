use derive_deftly::define_derive_deftly;
use pt_err::ConfigError;
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
}
