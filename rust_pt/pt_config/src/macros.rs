use derive_deftly::define_derive_deftly;
use pt_err::ConfigError;
define_derive_deftly! {
    /// Build a structure with check,
    /// This function will create a function which validates
    build_with_check for struct: 
    impl $ttype {
        $tvis fn build( $( $fname: $ftype ,)) -> Result<Self, ConfigError> {
            let config  = ${tmeta(build_with_check(build_func)) as path};
            ${tmeta(build_with_check(validate_func)) as path}(config)?;
            Ok(config)
        }
    }
}