// File: rust_pt\pt_config\src\macros.rs
// Directory: rust_pt\pt_config\src
// Filename: macros.rs
//======================================================================

//! For grammar of derive-deftly:
//! See: https://docs.rs/derive-deftly/1.11.5/derive_deftly/doc_reference/index.html
//! TODO(rust-pt#1):
//! For codes like:
//! _ => { return Err(pt_err::VariantError::UnfoundError {
//!    message: ...
//! });
//! We could iterate the enum using $vname/$vindex to tell the user
//! what kind of String/Index/else would be valid
//! But leave it for now, we should open an Issue for that  
//! ---
//! TODO(rust-pt#2):
//! Consider making these macros compatible with structs.
//! We should use $(if is_struct {...}) and $fname
use derive_deftly::define_derive_deftly;
define_derive_deftly! {
    /// Implement to string via iterate the enum
    /// # Errors
    /// `from_str()` and `Ex::try_from` Returns `UnfoundError`
    /// if the string does not match any enum variant.
    ///
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
    use std::string::ToString;
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
                e => { 
                    return Err(pt_err::VariantError::UnfoundError {
                    message: format!("{}", e) 
                });
            }
            }
        }
    }
    impl TryFrom<&str> for $ttype {
        type Error = pt_err::VariantError;
        fn try_from(value: &str) -> Result<Self, Self::Error> {
            match value {
                $(
                    stringify!($vname) => Ok($vpat),
                )
                e => { return Err(pt_err::VariantError::UnfoundError {
                    message: format!("{}", e) 
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
                e => { return Err(pt_err::VariantError::UnfoundError {
                    message: format!("{}", e) 
                });
            }
            }
        }
    }
}

define_derive_deftly! {
    /// The enum values must be consecutive integers starting from 0.
    /// # Errors
    /// Returns `pt_err::VariantError::UnfoundError` if the index does not match any enum variant.
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
    ///     fn `from_u8_index(v`: usize) -> Result<Self, `pt_err::VariantError`> {
    ///         match v {
    ///             0 => `Ex::A`,
    ///             1 => `Ex::B`,
    ///             _ => {
    ///                 return `Err(pt_err::VariantError::UnfoundError` {
    ///                     message: "".`to_string()`
    ///                 });
    ///             }
    ///         }
    ///     }
    ///     // Else funtions....
    /// }
    export FromU8Index for enum:
    impl $ttype {
        $tvis fn from_u8_index(v: usize) -> Result<Self, pt_err::VariantError> {
            match v {
                $(
                    $vindex => Ok($vpat),
                )
                e => { return Err(pt_err::VariantError::UnfoundError {
                    message: format!("{}", e) 
                });
            }
            }
        }
        $tvis fn get_index(&self) -> Result<usize, pt_err::VariantError> {
            match self {
                $(
                    $vpat => Ok($vindex),
                )
                e => { return Err(pt_err::VariantError::UnfoundError {
                    message: format!("{}", *e as usize) 
                });
            }
            }
        }
    }
}

define_derive_deftly! {
    /// Get the variant from discriminant
    /// # Errors
    /// Returns `pt_err::VariantError::UnfoundError` if the index does not match any enum variant.
    /// # Example
    /// ```rust
    /// pub use derive_deftly::{define_derive_deftly, Deftly};
    /// pub use pt_config::derive_deftly_template_FromDiscriminant;
    /// pub use anyhow::Result;
    /// #[repr(u8)]
    /// #[derive(Deftly)]
    /// #[derive_deftly(FromDiscriminant)]
    /// enum Ex {
    ///     A = 0,
    ///     B = 1,
    /// }
    /// fn main() -> Result<()> {
    ///     let x = Ex::from_discriminant(0 as usize).unwrap();
    ///     match x {
    ///         Ex::A => println!("success"),
    ///         _ => panic!("from_discriminant failed")
    ///     }
    ///     Ok(())
    /// }
    /// ```
    export FromDiscriminant for enum:
    impl $ttype {
        /// Get the variant from discriminant
        $tvis fn from_discriminant(v: usize) -> Result<Self, pt_err::VariantError> {
            $(
                let $< discriminant_ $vname > = $vpat as usize;
            )
            match v {
            $(
                $< discriminant_ $vname > => Ok($vpat),
            )
            e => { return Err(pt_err::VariantError::UnfoundError {
                    message: format!("{}", v) 
                });
            },
        }
    }
    }
}

define_derive_deftly! {
    /// Everything field in a structure which is implementing this macro must have:
    /// #[deftly(default = "...")]
    /// And if you want String:
    /// default = "\"example\".to_string()"
    ///
    /// Some other:
    /// #[deftly(default = "8080")]        // i32
    /// #[deftly(default = "3.14")]        // f64
    /// #[deftly(default = "true")]        // bool
    /// #[deftly(default = "42")]          // u32, i64 And so on
    /// For `IpAddr`:
    /// #[deftly(default = "IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))")]
    ///
    /// So just use the normal expression which you would like to use at the code.
    /// Even:
    /// { let x = 10; x * 2 }
    /// Works in this case
    /// (Oh unfortunatly it's too flexible that makes it seems like some JavaScript)
    ///
    /// ```rust
    /// use derive_deftly::{Deftly};
    /// use std::net::{IpAddr, Ipv4Addr};
    /// use pt_config::derive_deftly_template_Builder;
    /// use std::path::PathBuf;
    /// #[derive(Deftly, PartialEq, Eq, Clone)]
    /// #[derive_deftly(Builder)]
    /// struct Ex {
    ///     #[deftly(default = "\"example\".to_string()")]
    ///     name: String,
    ///     #[deftly(default = "{ let x = 10; x * 2 }")]
    ///     age: i32,
    ///     #[deftly(default = "\"./rust\"")]
    ///     path: PathBuf,
    ///     #[deftly(default = "IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))")]
    ///     ip: IpAddr,
    ///     #[deftly(default = "{
    ///          let x = 1 * 5;
    ///          if x == 5 {
    ///             let y = x * 2;
    ///             y
    ///         } else { 33 }
    ///     }")]
    ///     crazy_calculation: i32,
    /// }
    ///
    /// fn main() {
    ///     let mut ex = Ex::builder()
    ///     .with_age(18)
    ///     .with_name("henry".to_string());
    ///     assert_eq!(ex.crazy_calculation, 10);
    ///     assert_eq!(ex.age, 18);
    ///     assert_eq!(ex.name, "henry".to_string());
    ///     ()
    /// }
    ///
    ///
    /// ```
    export Builder for struct:

    impl $ttype {
        /// Build a default structure
        $tvis fn builder() -> Self {
            Self {
                $(
                    ${if fmeta(default) {
                        $fname: ${fmeta(default) as expr}.into(),
                    } else {
                        compile_error!("field {} needs a default value", $fname),
                    }}
                )
            }
        }
        $(  /// Set different fields
            $tvis fn $< with_ $fname >(mut self, value: $ftype) -> Self {
                self.$fname = value;
                self
            }
        )
    }
}
