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

here we have the most important public functions

```rust
pub fn pt_config_init(...) {...}
```

it uses envy to deserialize 

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


TODO(rust_pt#5): add more docs about architecture