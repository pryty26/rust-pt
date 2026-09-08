# rust-pt Architecture

This document describes the high-level architecture of rust-pt, a Rust framework for implementing Tor Pluggable Transports (PT) according to the pt-spec and ext-orport-spec. The framework is designed as a modular workspace of crates, each addressing a specific concern of the PT lifecycle.

## 1. Overall Structure

The project is organised as a Cargo workspace with the following main members:

- `pt_err` – Common error types and utility macros.
- `pt_tracing` – Logging and PT‑specific control message output.
- `pt_config` – Environment variable parsing, configuration structures, and derive macros.
- `pt_core` – Extended ORPort (ExtORPort) authentication and protocol implementation.
- `transports/reality-pt` – Example transport that exercises the framework (work in progress).

The workspace is configured with a unified version and shared metadata. All internal crates depend on each other via path dependencies during development.

## 2. Crate Responsibilities

### 2.1 pt_err
Centralises error definitions used across the framework. It provides three main error enums:

- `ExtOrPortError` – For failures during ExtORPort negotiation and authentication (e.g., invalid static header, unsupported auth types, hash mismatch).
- `ConfigError` – For environment configuration parsing failures (e.g., invalid proxy URL, unsupported version).
- `VariantError` – For lookup failures when converting between enum variants and strings or discriminants.

The crate also defines two derive macros via `derive_deftly`:

- `OtherFromError` – Automatically implements `From<anyhow::Error>` and `From<std::io::Error>` for enums that have an `Other(String)` variant, simplifying error propagation.
- `DefineVariantError` – Generates a dedicated `*ConfigError` enum for a struct, creating `Invalid<Field>` and `<Field>Unfound` variants for each field, useful for detailed validation errors.

### 2.2 pt_tracing
Provides a logging and message output system compliant with the PT specification. It is built on `tracing` and `tracing-subscriber`, but customises output to match the required format (e.g., `LOG SEVERITY=... MESSAGE=...`).

Key components:

- `SEVERITY` – Enum representing log levels (DEBUG, INFO, NOTICE, WARNING, ERROR). Implements conversion to `tracing::level_filters::LevelFilter`.
- `PtTracing` – Configuration struct stored in a global `OnceLock`. It offers a builder-style interface (`fmt()`, `with_severity()`, `try_init()`) to set up the subscriber.
- `TorPtCommunicator` – Trait that defines static methods for all PT‑required messages, such as `version()`, `cmethod()`, `smethod()`, `env_error()`, `proxy_done()`, etc. Each method outputs the exact string expected by the Tor parent process.
- `rec_panic!` – Macro that logs an error via `TorPtCommunicator::error` and then panics. Used for unrecoverable protocol errors where termination is required.

Notice messages are handled specially because `tracing` does not have a notice level; the implementation checks the configured severity and prints only when the severity is DEBUG, INFO, or NOTICE.

### 2.3 pt_config
Handles all environment variable parsing and validation as specified in the PT specification. It uses `serde` and `envy` to deserialize from the environment into Rust structures.

Main types:

- `CommonKey` – Shared settings: version list, state directory, exit‑on‑stdin‑close flag, and optional outbound bind addresses (IPv4/IPv6). It performs validation (e.g., checks that `TOR_PT_EXIT_ON_STDIN_CLOSE` is 0 or 1, and ignores loopback addresses for bind).
- `ClientKey` – Client‑side settings: list of client transports and an optional proxy URL. The proxy URL is validated against supported schemes (socks5, socks4a, http) with credential checks.
- `ServerKey` – Server‑side settings: server transports, transport options, bind addresses, ORPort, Extended ORPort, and authentication cookie file path. It also emits a warning if the Extended ORPort is not bound to localhost.
- `TransportOptions` – Parses the `TOR_PT_SERVER_TRANSPORT_OPTIONS` string into a vector of `TransportOption` (each with a name and a hashmap of key‑value settings).
- `ConfigKey` – An untagged enum that holds either a server or client configuration, plus the common key. Its `init()` method attempts to deserialize both `ServerKey` and `ClientKey`; exactly one must succeed, otherwise it panics with a descriptive error.

The crate also exports several derive macros:

- `FromString` – Implements `From<Enum> for String`, `TryFrom<String>`, `TryFrom<&str>`, and `FromStr` for enums, matching variant names.
- `FromU8Index` – Adds `from_u8_index()` and `get_index()` for enums with consecutive integer discriminants (starting from 0).
- `FromDiscriminant` – Adds `from_discriminant()` to retrieve a variant from its numeric discriminant (useful for raw protocol values).
- `Builder` – For structs, generates a `builder()` method that sets default values via `#[deftly(default = "...")]` attributes and provides `with_<field>()` setters.

Environment loading is done once via `dotenvy` in a `LazyLock` static.

### 2.4 pt_core
Implements the client‑side logic for the Extended ORPort protocol, as defined in `ext-orport-spec`. It is the core networking and authentication layer.

Key modules:

- `orport/extorport.rs` – Defines the `ExtOrPort` struct, which holds the target address, cookie file path, and current state. It implements a builder pattern via `derive_deftly(Builder)`. The main method is `connect()`, an async function that:
  1. Establishes a TCP connection to the ExtORPort.
  2. Reads the server's `AuthTypes` list (a null‑terminated sequence of bytes) into a fixed‑size buffer `AuthNegBuf`.
  3. Selects a supported authentication type (currently only `SAFE_COOKIE`, value 1) and sends it back.
  4. Transitions to the `SafeCookieAuthentication` state and calls the private `safe_cookie_authentication()` method.
- `safe_cookie_authentication()` – Implements the SAFE_COOKIE handshake:
  - Reads the cookie file (validating the static header "! Extended ORPort Auth Cookie !\x0a") to obtain the 32‑byte CookieString.
  - Generates a random 32‑byte ClientNonce and sends it.
  - Receives 64 bytes: ServerHash (32) and ServerNonce (32). It verifies the ServerHash using HMAC‑SHA256 with the label "ExtORPort authentication server-to-client hash".
  - Computes ClientHash (with the client‑to‑server label) and sends it.
  - Reads the final Status byte; must be 1 for success, otherwise returns an error.
- `orport/protocol.rs` – Implements the `ClientExtOrPortProtocol` trait, which provides methods to send post‑authentication commands: `done()` (0x0000), `user_addr()` (0x0001 with a validated address string), and `transport()` (0x0002 with the PT name). Each command follows the format: COMMAND (2‑byte big‑endian) + BODYLEN (2‑byte big‑endian) + BODY.
- `orport/traits.rs` – Defines the `ClientExtOrPortProtocol` trait, which is marked `#[async_trait]` and can be implemented by any type that needs to send these commands.
- `variables.rs` – Exports the static header constant for the cookie file.

State machines are used to track the progress of authentication (`ExtOrPortState` and `SafeCookieState`). Errors during I/O or cryptography are converted to `ExtOrPortError`; critical failures (e.g., connection closed unexpectedly) trigger `rec_panic!`.

### 2.5 transports/reality-pt
An example transport that uses the framework. It is intended to demonstrate how to build a PT with rust-pt. Currently it contains minimal code (download module, main entry, variable definitions) and serves as a testbed for the API.

## 3. Data Flow – Client Transport Initialisation

A typical client‑side PT using rust-pt would follow these steps:

1. **Load configuration** – Call `ConfigKey::init()` to parse environment variables. This will panic on invalid or missing required variables, as per PT spec requirements.
2. **Set up logging** – Call `PtTracing::fmt().with_severity(SEVERITY::NOTICE).try_init().unwrap()` to initialise the global tracing subscriber and store the severity.
3. **Create ExtOrPort instance** – Use the builder: `ExtOrPort::builder().with_addr(socket_addr).with_auth_cookie_file(path).build()`.
4. **Connect and authenticate** – Call `extorport.connect().await`. This returns a `TcpStream` after successful authentication.
5. **Send protocol commands** – Split the stream into read/write halves, then call `ExtOrPort::user_addr()` and `ExtOrPort::transport()` as needed, followed by `done()`.
6. **Tunnel traffic** – After `done()`, the remaining bytes on the stream are ordinary OR traffic. The transport can now forward application data.

## 4. Concurrency Model

All I/O operations are asynchronous using `tokio`. The framework does not spawn threads internally; concurrency is left to the caller (e.g., using `tokio::spawn` per client connection). This design keeps the core simple and allows integration with various async runtimes.

## 5. Error Handling Philosophy

The framework distinguishes between recoverable and unrecoverable errors:

- Recoverable errors (e.g., invalid user address, unsupported auth type) are returned as `Result` variants from the appropriate error enums.
- Unrecoverable errors (e.g., connection closed by the server during authentication, missing required environment variables) trigger a panic after logging via `rec_panic!`. This is intentional because the PT proxy cannot function without a valid connection or configuration, and termination is required by the PT specification.

## 6. Extensibility Points

- **New authentication methods** – Add a new variant to `AuthTypes` and extend the state machine in `ExtOrPort` with a new authentication function.
- **Additional protocol commands** – Extend the `ClientExtOrPortProtocol` trait with new command methods and implement them in `protocol.rs`.
- **Custom configuration fields** – Add fields to `CommonKey`, `ClientKey`, or `ServerKey` and adjust the `TryFrom` implementations for validation; the `Builder` macro can be reused.
- **Different logging backends** – The `pt_tracing` crate is built on `tracing`, but the `TorPtCommunicator` trait could be implemented by other types to redirect output.

## 7. Current Limitations

- Only `SAFE_COOKIE` authentication is supported; other methods (e.g., `NULL`) are not implemented.
- The framework does not parse server responses (e.g., `OKAY`, `DENY`) after sending commands; the caller must handle them directly on the stream.
- The `reality-pt` transport is not yet complete; it serves more as a placeholder.
- The configuration parsing assumes all required variables are present and valid; partial or malformed input leads to panic.

## 8. Dependencies Overview

- Async runtime: `tokio` (with full features).
- Serialisation: `serde`, `envy`, `serde_json`, `toml`.
- Cryptography: `hmac`, `sha2`, `getrandom` (for nonce generation).
- Error handling: `thiserror`, `anyhow`.
- Macros: `derive_deftly` for custom derives; `tracing` for logging.
- URL parsing: `url` (for proxy URI validation).
- Additional utilities: `dotenvy` for `.env` loading.

## 9. Conclusion

rust-pt provides a solid foundation for building Tor Pluggable Transports in Rust, with a clear separation of concerns, thorough adherence to the PT specifications, and a flexible macro system to reduce boilerplate. The architecture is asynchronous, modular, and designed to be extended with new transports and authentication schemes.