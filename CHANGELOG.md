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
