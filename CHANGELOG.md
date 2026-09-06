## [0.0.0-alpha.2] - 2026-09-06

### Features
- *(pt_core)* Add ExtOrPort support for PT connections
- *(pt_config)* Add ConfigKey parsing and validation
- *(pt_tracing)* Implement PT tracing functionality
- Add ServerKey and ClientKey with validation
- Add `create_all_path()` and `create_keys()` functions for key generation
- Add `Builder` derive macro for configuration building
- *(pt_config)* Make generated `FromU8Index` functions inherit enum visibility
- Add `TorPtCommunicator` trait for PT operations
- Add safe cookie authentication for ExtOrPort
- Add support for version detection during runtime

### Bug Fixes
- *(extorport)* Improve error handling and protocol parsing
- Fix toml workspace dependency conflicts
- Fix typos in ExtOrPort protocol implementation

### Documentation
- Add comprehensive PT architecture guide (`pt-architecture.md`)
- Improve README with better examples
- Add contributing guidelines (`CONTRIBUTING.md`)
- Add CLA and code style preferences
- *(extorport)* Document ExtOrPort usage in pt_core
- Add TODO list for future improvements

### Refactor
- *(pt_core)* Organize ORPort implementations into separate modules
- *(pt_tracing)* Streamline tracing implementation
- Reorganize configuration module

### Performance
- *(No performance changes in this release)*

### Miscellaneous Tasks
- Add maintenance scripts
- Configure rustfmt and linting
- Set up Codeberg mirror

---