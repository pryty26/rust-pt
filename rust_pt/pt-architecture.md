### pt-architecture.md 

### Author: pryty26

### Structure of rust_pt

Table of Contents
1.* = pt_config
2.* = pt_core
3.* = pt_err
4.* = pt_tracing

1. pt_config = Config parsing:

📦src
 ┣ 📂configs
 ┃ ┣ 📜configs.rs
 ┃ ┗ 📜core.rs
 ┣ 📜configs.rs
 ┣ 📜lib.rs
 ┣ 📜macros.rs
 ┗ 📜variable.rs

1.1 src/configs:
In the configs dir we have the detailed Config parsing, 

1.1.1: in configs/configs.rs we indicated:

ServerKey
ClientKey
And
CommonKey

Which are the most important structures in the pt_config

We also implicated try_from to them through RawServerKey, RawCommonKey and RawClientKey.

We try to follow the pt-spec.txt

1.1.2 core.rs

We are going to implicate a function which uses envy to deserialize 

ServerKey
And
CommonKey

Or

ClientKey
And
CommonKey

1.2 lib.rs:

pub mod every module

defines variable "MODE" which could be changed during the config parsing

1.3 macros.rs

uses derive-deftly for defining some pre-macro which could be useful:

```rust
#[repr(u8)]
#[derive(Debug, Clone, Copy, Deftly)]
#[derive_deftly(FromString)]
```
FromString implicates:
impl From<$ttype> for String
impl TryFrom<String> for $ttype
And
impl std::str::FromStr for $ttype

to enum through iterating variants.

And:
FromU8Index implicates
```rust 
fn from_u8_index(v: usize) -> Result<Self, pt_err::VariantError> 
// And
fn get_index(&self) -> Result<usize, pt_err::VariantError> 
```

Note(pryty26): The enum values must be consecutive integers starting from 0.

1.4 variable.rs

Defines some variable

4. 
📦pt_tracing
 ┣ 📂src
 ┃ ┗ 📜lib.rs
 ┗ 📜Cargo.toml

4.1 This is much simpler crate, because we do not need to do very much thing on here

There is only one main structure:

```rust 
/// Config of pt_tracing,
/// user could change it via different function
pub struct PtTracing {
    /// The SEVERITY value indicate at which logging level the message applies.
    /// The accepted values for <Severity> are: error, warning, notice, info, debug
    pub severity: SEVERITY,
}
```

But FWIW, I think looking to my code or pt-spec.txt is best way to learn it...

4.2 Useful functions:

For initing(Yes I indeed copy-pasted docs test codes):
```rust
use pt_tracing::{PtTracing, SEVERITY};

fn main() -> anyhow::Result<()> {
    // Available severity levels:
    // DEBUG, INFO, NOTICE, WARNING, ERROR
    // let level = SEVERITY::INFO; // Or else
    let level = SEVERITY::NOTICE;
    // Or:
    // let pt_tracing = PtTracing::default_init()?;
    let pt_tracing = PtTracing::new(level).init()?;
    // Yes I used Result in every DEBUG, INFO, NOTICE, WARNING, ERROR function
    pt_tracing.debug("cool debug message")?;
    pt_tracing.info("cool info message")?;
    // And so on
    Ok(())
}
```

Also, for detailed information, I believe you must read pt-spec.txt
Spec is always best docs. 🤪

TODO(rust_pt#5): add more docs about architecture