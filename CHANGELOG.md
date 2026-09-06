## [v0.0.0-alpha.2]

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
- Resolve toml workspace dependency and extorport protocol issues
- Resolve toml workspace dependency and extorport protocol issues
- *(extorport)* Fix extorport protocol

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
- Rewrite pt-architecture.md
- Add last update for pt-architecture
- Add markdown support for pt-architecture

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
- Add CHANGELOG and cliff.toml
- Add changelogs folder for storing old changelogs

### 💼 Other

- Add SideMode and env loading
- CONTRIBUTE.md
- Change lint and Improve some code
- Change the type of CURRENT_TRANSPORT_VER into [&str; 1]
- Add license
- Rust_pt v0.0.0-alpha.1
- 0.0.0-alpha.2
