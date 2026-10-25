## v0.0.0-beta.0

### 🚀 Features

- *(pt_core)* Add OrProdKind::connect_or and connect_or_tcpstream

### 🐛 Bug Fixes

- *(socks)* Add SocksConn address conversion and wire up upstream relay

### 🚜 Refactor

- *(socks)* [**breaking**] Represent target address with HostAddr enum
- *(pt-core)* Add ORPort producers and update connection handling

### ⚙️ Miscellaneous Tasks

- Update README
- Store the old CHANGELOG
- Docs change

### 💼 Other

- 0.0.0-beta.0
## [0.0.0-alpha.5] - 2026-10-13

### 🚀 Features

- *(pt-core)* Add OrPort fallback and server config helpers
- *(pt_core)* Return OrPortKind from connect_or
- *(pt_config)* Send PROXY DONE/ERROR on proxy validation
- Change the error types
- *(pt_core)* Add SOCKS5 protocol constants to variables
- *(pt_core)* Add SOCKS5 protocol constants to variables
- A dummy example pt
- *(pt_core)* Add ExtOrPort handshake helpers on Pt
- *(pt-core)* Add transport and proxy scheme configuration
- *(pt)* Add transport filtering and switch PT protocol output to tracing
- *(pt_core)* Validate proxy URL scheme in filter_url
- Add full feature to workspace crates and gate extorport behind cfg
- *(socks)* Add SOCKS5 core implementation
- Refactor ORPort handling and improve SOCKS forwarding

### 🐛 Bug Fixes

- *(extorport)* Replace non-constant-time with constant-time comparison
- Remove unneccesary allow
- Remove reality-pt from the members

### 📚 Documentation

- Update CHANGELOG for v0.0.0-alpha.3
- Update architecture file
- *(pt_core)* Document sup_transports/proxy_schemes validation

### 🚜 Refactor

- *(pt_core)* Use OwnedWriteHalf in ExtOrPort protocol and add done_wait
- *(pt_err)* Make UnsupportedAuthTypes carry a reason string
- *(pt_tracing)* Emit PT protocol messages via error!
- *(init)* Split transport filters into smethod/cmethod variants
- *(pt_core)* Narrow Pt fields to pub(crate) and add split_extorport
- Drop HTTP proxy support and add SOCKS4

### 🎨 Styling

- Apply rustfmt

### ⚙️ Miscellaneous Tasks

- Update project's website
- Improve website's html
- Update dependencies
- Check and update the file tree
- Update CLA
- Remove temporaryly the reality-pt due to the roadmap
- Stop tracking reality-pt
- Add CI
- Add workflow for CI
- Add cache and clippy
- Update CI rules
- Fix CI
- Replace --all with --workspace --all-targets --features full -- -D clippy::pedantic for CI
- Update CI
- Update CI
- Update the lint
- *(aspt)* Tidy example, satisfy clippy
- Improve license
- Improve gitignore
- Improve crate documentation and format SOCKS modules

### 💼 Other

- Add read_exact/write_all to Pt and ExtOrPort
- V0.0.0-alpha.5
## [0.0.0-alpha.3] - 2026-09-20

### 🚀 Features

- Enhance contributing guidelines, README, and improve parsing logic for TransportOptions
- *(extorport)* Add client-side receiver for ExtOrPort replies, improve Docs and Resolve some problem
- *(pt_core)* Add ExtOrPort reply handling and Pt initialization

### 🐛 Bug Fixes

- *(pt_config)* Fix macros error message by use format!()
- Resolve critical issues in config, macros, and protocol

### 📚 Documentation

- Tweak README.md and add github mirror (github.com/pryty26/rust-pt)
- Update pt-architecture
- *(README)* Add Marat K to contributors wall
- Removed outdated panics comments
- Add 41_un as a Contributor
- Improve README

### 🚜 Refactor

- *(extorport)* Replace rec_panic with typed errors

### ⚙️ Miscellaneous Tasks

- Tweak changelog
- *(clippy)* Add clippy::pedantic and comprehensive clippy fixes, docs, and Cargo metadata improvements
- Add robots.txt
- Update .gitignore
- Rename cargo.toml -> Cargo.toml
- Improve website

### 💼 Other

- Final test before v0.0.0-beta (all features done)
## [0.0.0-alpha.2] - 2026-09-06

### 🐛 Bug Fixes

- Resolve toml workspace dependency and extorport protocol issues
- Resolve toml workspace dependency and extorport protocol issues
- *(extorport)* Fix extorport protocol

### 📚 Documentation

- Rewrite pt-architecture.md
- Add last update for pt-architecture
- Add markdown support for pt-architecture

### ⚙️ Miscellaneous Tasks

- Add CHANGELOG and cliff.toml
- Add changelogs folder for storing old changelogs

### 💼 Other

- 0.0.0-alpha.2
## [0.0.0-alpha.1] - 2026-09-05

### 🚀 Features

- Add create_all_path() and create_keys() functions for key generation
- Add ServerKey and ClientKey
- Add validate for serverkey
- Add macros
- Add validate ClientKey
- Historical Time!
- *(pt_tracing)* Pt_tracing done
- Check whether we support the version which is given by user
- Add test and new functions
- *(pt_config)* Make generated FromU8Index functions inherit enum visibility
- Add ConfigKey to parse config
- Add trait TorPtCommunicator for PtTracing and prelude
- Add from discriminant
- *(pt_core)* Add ExtOrPort
- Add safe_cookie_authentication() and detailed code for extorport
- Add Builder derive macro and integrate into ExtOrPort

### 🐛 Bug Fixes

- Improve pt_tracing
- Add  to replace the pub for Builder
- *(extorport)* Improve error handling and protocol parsing
- *(extorport)* Typo

### 📚 Documentation

- *(README.md)* Improve README.md
- *(pt_config::macros)* Improve docs of macros
- Improve docs
- Add CONTRIBUTE.md
- Add CONTRIBUTE.md
- Improve docs and add rustfmt.toml
- Add TODO
- Add CLA.md and improve README.md
- Add pt-architecture.md
- Change docs
- Improve pt_tracing and add docs
- Add codeberg as our new mirror
- Change CONTRIBUTE.md into CONTRIBUTING.md so gitlab and codeberg could detect it
- 'add code style we prefer' for CONTRIBUTING.md
- *(extorport)* Docs for ExtOrPort using in pt_core

### 🚜 Refactor

- Config
- *(pt_tracing)* Refactor pt_tracing
- *(pt_core)* Add orport/* for clearly storing extended and normal orport implementions

### 🎨 Styling

- Apply cargo fmt and slighly change README

### ⚙️ Miscellaneous Tasks

- Add gitignore
- *(maint)* Reorder lint attributes
- Add .gitlab
- Someday
- Add template
- Remove GitLab CI configuration
- *(maint)* Add maint scripts

### 💼 Other

- Add SideMode and env loading
- CONTRIBUTE.md
- Change lint and Improve some code
- Change the type of CURRENT_TRANSPORT_VER into [&str; 1]
- Add license
- Rust_pt v0.0.0-alpha.1
