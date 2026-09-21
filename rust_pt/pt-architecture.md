# rust-pt Architecture

This document describes the high-level architecture of rust-pt, a Rust framework for implementing Tor Pluggable Transports (PT) according to the pt-spec and ext-orport-spec. The framework is designed as a modular workspace of crates, each addressing a specific concern of the PT lifecycle.

## 1. Overall Structure

The project is organised as a Cargo workspace with the following main members:

- `pt_err` – Common error types and utility macros.
- `pt_tracing` – Logging and PT‑specific control message output.
- `pt_config` – Environment variable parsing, configuration structures, and derive macros.
- `pt_core` – Pt lifecycle, Extended ORPort (ExtORPort) authentication and protocol implementation.
- `transports/reality-pt` – Example transport that exercises the framework (work in progress).

The workspace is configured with a unified version and shared metadata. All internal crates depend on each other via path dependencies during development.

## 2. Crate Responsibilities

### 2.1 pt_err
Centralises error definitions used across the framework. It provides four main error enums:

- `ExtOrPortError` – For failures during ExtORPort negotiation and authentication (e.g., invalid static header, unsupported auth types, hash mismatch, stream missing).
- `PtError` – For failures at the `Pt` level (e.g., `Pt` not initialised, ExtORPort not set).
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

Notice messages are handled specially because `tracing` does not have a notice level; the implementation checks the configured severity and prints only when the severity is DEBUG, INFO, or NOTICE. If `PT_TRACING` is not yet set, `notice()` returns an error.

### 2.3 pt_config
Handles all environment variable parsing and validation as specified in the PT specification. It uses `serde` and `envy` to deserialize from the environment into Rust structures.

Main types:

- `CommonKey` – Shared settings: version list, state directory, exit‑on‑stdin‑close flag, and optional outbound bind addresses (IPv4/IPv6). It performs validation (e.g., checks that `TOR_PT_EXIT_ON_STDIN_CLOSE` is 0 or 1, and ignores loopback addresses for bind).
- `ClientKey` – Client‑side settings: list of client transports and an optional proxy URL. The proxy URL is validated against supported schemes (socks5, socks4a, http) with credential checks.
- `ServerKey` – Server‑side settings: server transports, transport options, bind addresses, ORPort, Extended ORPort, and authentication cookie file path. It also emits a warning if the Extended ORPort is not bound to localhost.
- `TransportOptions` – Parses the `TOR_PT_SERVER_TRANSPORT_OPTIONS` string into a vector of `TransportOption` (each with a name and a hashmap of key‑value settings). Backslash‑escaped `:`, `;` and `\` inside names, keys and values are supported.
- `ConfigKey` – An untagged enum that holds either a server or client configuration, plus the common key. Its `init()` method attempts to deserialize both `ServerKey` and `ClientKey`; exactly one must succeed, otherwise it panics with a descriptive error.

The crate also exports several derive macros:

- `FromString` – Implements `From<Enum> for String`, `TryFrom<String>`, `TryFrom<&str>`, and `FromStr` for enums, matching variant names.
- `IntoU8` – Implements `From<Enum> for u8` for enums with `#[repr(u8)]`.
- `FromU8Index` – Adds `from_u8_index()` and `get_index()` for enums with consecutive integer discriminants (starting from 0).
- `FromDiscriminant` – Adds `from_discriminant()` to retrieve a variant from its numeric discriminant (useful for raw protocol values).
- `Builder` – For structs, generates a `builder()` method that sets default values via `#[deftly(default = "...")]` attributes and provides `with_<field>()` setters.

Environment loading is done once via `dotenvy` in a `LazyLock` static.

### 2.4 pt_core
Implements the Pt lifecycle and the client‑side logic for the Extended ORPort protocol, as defined in `ext-orport-spec`. It is the core networking and authentication layer.

Key modules:

- `lib.rs`, `init.rs` – Define the `Pt` struct, which holds the log `severity`, an optional `ConfigKey`, and an optional `ExtOrPort`. It implements a builder pattern via `derive_deftly(Builder)`. `try_init()` initialises tracing and loads the config; `try_extorport()` will, for a Server config, build an `ExtOrPort` from `TOR_PT_EXTENDED_SERVER_PORT` and `TOR_PT_AUTH_COOKIE_FILE` and call `connect().await`. `get()` returns `Result<&ConfigKey, PtError>`, and `is_server()` / `is_client()` delegate to `ConfigKey`.
- `orport/extorport.rs` – Defines the `ExtOrPort` struct, which holds the target address, cookie file path, current state, and owned reader/writer halves. It implements a builder pattern via `derive_deftly(Builder)`. The main method is `connect()`, an async function that:
  1. Establishes a TCP connection to the ExtORPort.
  2. Reads the server's `AuthTypes` list (a null‑terminated sequence of bytes) into a fixed‑size buffer `AuthNegBuf`.
  3. Selects a supported authentication type (currently only `SAFE_COOKIE`, value 1) and sends it back.
  4. Transitions to the `SafeCookieAuthentication` state and calls the private `safe_cookie_authentication()` method.
  5. On success, stores the owned reader and writer halves on `self` and returns `Result<(), ExtOrPortError>` (the `TcpStream` itself is not returned).
- `safe_cookie_authentication()` – Implements the SAFE_COOKIE handshake:
  - Reads the cookie file (validating the static header "! Extended ORPort Auth Cookie !\x0a") to obtain the 32‑byte CookieString.
  - Generates a random 32‑byte ClientNonce and sends it.
  - Receives 64 bytes: ServerHash (32) and ServerNonce (32). It verifies the ServerHash using HMAC‑SHA256 with the label "ExtORPort authentication server-to-client hash".
  - Computes ClientHash (with the client‑to‑server label) and sends it.
  - Reads the final Status byte; must be 1 for success, otherwise returns an error.
- `recv_listen()` – Implements `ClientRecvExtOrPortProtocol`. A synchronous state machine (`ExtOrPortRecvState`: `CommandRecognition` → `LengthRecognition` → `BodyParsing` → `StateEnd`) that reads one reply frame and returns `ExtOrPortReply`. The bodies of `OKAY`, `DENY` and `Unknown` replies are read and discarded.
- `ExtOrPortReply` – `Okay` (0x1000), `Deny` (0x1001), `Unknown(u16)` and `NotReceived`. `from_discriminant` is hand-written because the derive macro cannot handle tuple variants.
- `orport/protocol.rs` – Implements the `ClientExtOrPortProtocol` trait, which provides methods to send post‑authentication commands: `done()` (0x0000), `user_addr()` (0x0001 with a validated address string), and `transport()` (0x0002 with the PT name). Each command follows the format: COMMAND (2‑byte big‑endian) + BODYLEN (2‑byte big‑endian) + BODY.
- `orport/traits.rs` – Defines the `ClientExtOrPortProtocol` trait (marked `#[async_trait]`), which can be implemented by any type that needs to send these commands, and the `ClientRecvExtOrPortProtocol` trait, which exposes `recv_listen(&mut self)`.
- `variables.rs` – Exports the static header constant for the cookie file, along with the `CMD_DONE`, `CMD_USERADDR` and `CMD_TRANSPORT` constants.

State machines are used to track the progress of authentication (`ExtOrPortState` and `SafeCookieState`). Errors during I/O or cryptography are converted to `ExtOrPortError`. The ExtORPort handshake now returns typed errors (e.g. `ServerClosedConnection`, `UnsupportedAuthTypes`, `InvalidServerHash`, `InvalidClientHash`, `StreamMissing`) rather than panicking; `check_msg()` no longer panics on EOF.

### 2.5 transports/reality-pt
An example transport that uses the framework. It is intended to demonstrate how to build a PT with rust-pt. Currently it contains minimal code (download module, main entry, variable definitions) and serves as a testbed for the API.

## 3. Data Flow – Client Transport Initialisation

A typical client‑side PT using rust-pt would follow these steps:

1. **Load configuration** – Call `ConfigKey::init()` to parse environment variables. This will panic on invalid or missing required variables, as per PT spec requirements.
2. **Set up logging** – Call `PtTracing::fmt().with_severity(SEVERITY::NOTICE).try_init().unwrap()` to initialise the global tracing subscriber and store the severity.
3. **Create ExtOrPort instance** – Use the builder: `ExtOrPort::builder().with_addr(socket_addr).with_auth_cookie_file(path).build()`.
4. **Connect and authenticate** – Call `extorport.connect().await`. This returns `Result<(), ExtOrPortError>` after successful authentication; the reader/writer halves are stored inside `self`.
5. **Send protocol commands** – Call `ExtOrPort::user_addr()` and `ExtOrPort::transport()` as needed, followed by `done()`. Then optionally call `extorport.recv_listen().await` to read the server's `OKAY` / `DENY` reply.
6. **Tunnel traffic** – After `done()`, the remaining bytes on the stream are ordinary OR traffic. The transport can now forward application data.

## 4. Concurrency Model

All I/O operations are asynchronous using `tokio`. The framework does not spawn threads internally; concurrency is left to the caller (e.g., using `tokio::spawn` per client connection). This design keeps the core simple and allows integration with various async runtimes. Because `ExtOrPort` owns its reader and writer halves, callers must not hold a borrow across the handshake.

## 5. Error Handling Philosophy

The framework distinguishes between recoverable and unrecoverable errors:

- Recoverable errors (e.g., invalid user address, unsupported auth type) are returned as `Result` variants from the appropriate error enums (`ExtOrPortError`, `PtError`, `ConfigError`, `VariantError`).
- Unrecoverable errors at startup (e.g., missing required environment variables) still trigger a panic after logging via `rec_panic!`, for example inside `ConfigKey::init()`. This is intentional because the PT proxy cannot function without a valid configuration, and termination is required by the PT specification.
- During the ExtORPort handshake, critical failures (e.g., the server closing the connection) are now returned as typed errors instead of panicking. `check_msg()` returns `ExtOrPortError::ServerClosedConnection` on EOF.

## 6. Extensibility Points

- **New authentication methods** – Add a new variant to `AuthTypes` and extend the state machine in `ExtOrPort` with a new authentication function.
- **Additional protocol commands** – Extend the `ClientExtOrPortProtocol` trait with new command methods and implement them in `protocol.rs`.
- **Custom reply handling** – `ExtOrPortRecvState` and `ExtOrPortRecvSettings` are public, and `ClientRecvExtOrPortProtocol` can be implemented on a user type to provide an alternative receiver.
- **Custom configuration fields** – Add fields to `CommonKey`, `ClientKey`, or `ServerKey` and adjust the `TryFrom` implementations for validation; the `Builder` macro can be reused.
- **Different logging backends** – The `pt_tracing` crate is built on `tracing`, but the `TorPtCommunicator` trait could be implemented by other types to redirect output.

## 7. Current Limitations

- Only `SAFE_COOKIE` authentication is supported; other methods (e.g., `NULL`) are not implemented.
- No SOCKS5 server is provided. Client‑side PTs must implement their own SOCKS handling.
- `reality-pt` is not yet complete; it serves more as a placeholder.
- The configuration parsing assumes all required variables are present and valid; partial or malformed input leads to panic.
- `extorport.md` states that server‑response parsing is intentionally not implemented, but `recv_listen()` now does parse `OKAY` / `DENY` / `Unknown`. The doc and the code are out of sync.
- `PtTracing` is stored in a global `OnceLock`; once `try_init()` succeeds, the configuration cannot be replaced within the same process, which complicates test isolation.

## 8. Dependencies Overview

- Async runtime: `tokio` (with full features).
- Serialisation: `serde`, `envy`.
- Cryptography: `hmac`, `sha2`, `getrandom` (for nonce generation).
- Error handling: `thiserror`, `anyhow`.
- Macros: `derive_deftly` for custom derives; `tracing` for logging.
- URL parsing: `url` (for proxy URI validation).
- Additional utilities: `dotenvy` for `.env` loading.

## 9. Conclusion

rust-pt is an early‑stage (`0.0.0-alpha`) framework for building Tor Pluggable Transports in Rust, with a clear separation of concerns, adherence to the PT specifications, and a flexible macro system to reduce boilerplate. The architecture is asynchronous and modular. It currently covers typed configuration parsing, PT‑spec‑compliant log output, and a working ExtORPort client (SAFE_COOKIE authentication plus reply reception); it does not yet cover the SOCKS5 server side of a client PT, and several modules are explicitly marked as incomplete.