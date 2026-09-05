Pluggable Transport (PT) Architecture Documentation
Overview

This codebase implements a Tor Pluggable Transport proxy in Rust, following the pt-spec and ext-orport-spec. It supports both client-side and server-side operation, with a focus on the Extended ORPort (ExtORPort) authentication and communication protocol.

The project is organised into several crates:

    pt_core – Core ExtORPort authentication and protocol handling.

    pt_err – Centralised error types and macros.

    pt_config – Environment variable parsing, configuration structures, and derive macros.

    pt_tracing – Logging and PT‑specific message output (e.g., VERSION, CMETHOD).

pt_core – ExtORPort Implementation

This crate provides the client‑side logic for connecting to an Extended ORPort, authenticating via the SAFE_COOKIE mechanism, and sending protocol commands.
Key Modules

    orport/extorport.rs
    Contains the main ExtOrPort struct and its methods.

        ExtOrPort – Holds configuration (address, cookie file path, current state). Implements a builder pattern via derive_deftly(Builder).

        ExtOrPort::connect() – Asynchronous entry point:

            Connects to addr via TcpStream.

            Reads the server’s AuthTypes list (null‑terminated) using a fixed‑size buffer AuthNegBuf.

            Selects a supported AuthTypes (currently only SafeCookie = 1) and sends it back.

            Transitions to ExtOrPortState::SafeCookieAuthentication and calls safe_cookie_authentication().

        safe_cookie_authentication() – Implements the SAFE_COOKIE handshake:

            Reads cookie file, validates StaticHeader, obtains CookieString.

            Generates random ClientNonce (32 bytes) and sends it.

            Waits for ServerHash + ServerNonce (64 bytes total). Verifies ServerHash using server_hash().

            Computes ClientHash with client_hash() and sends it.

            Reads final Status byte – must be 1 for success.

        auth_cookie() – Reads the cookie file from disk and returns CookieString after header validation.

        mac_message() – HMAC‑SHA256 utility used for hash computations.

    orport/protocol.rs
    Implements the ClientExtOrPortProtocol trait for the post‑authentication phase:

        done() – sends 0x0000 command.

        user_addr() – sends 0x0001 with a validated SocketAddr string.

        transport() – sends 0x0002 with the PT name.
        All commands follow the format: COMMAND (2 bytes big‑endian) + BODYLEN (2 bytes) + BODY.

    orport/traits.rs
    Defines the ClientExtOrPortProtocol trait, which is the interface for sending protocol commands. It is designed to be implemented by ExtOrPort (and potentially others).

    variables.rs
    Exports the static header for the cookie file: "! Extended ORPort Auth Cookie !\x0a".

State Machines

    ExtOrPortState – AuthTypesNegotiation → SafeCookieAuthentication.

    SafeCookieState – SendClientNonce → RecvServerNonce → SendClientHash → RecvFinalResp.

Error Handling

All I/O and cryptographic errors are converted to ExtOrPortError (from pt_err) via ? and anyhow. Critical failures (e.g., connection closed) trigger rec_panic! (from pt_tracing) which logs and panics.
pt_err – Error Definitions and Macros

Centralised error types:

    ExtOrPortError – Specific errors for the ExtORPort protocol:

        InvalidStaticHeader

        UnsupportedAuthTypes

        EndAuthTypeUnfound

        InvalidServerHash, InvalidClientHash

        InvalidServerMsg, InvalidUserAddr

        Other(String) – wrapper for anyhow::Error and std::io::Error via derive_deftly(OtherFromError).

    ConfigError – Used by pt_config for environment parsing:

        InvalidConfigErr { message }

        UnsupportedVer { message }

        Other(String)

    VariantError – Used by FromString and FromDiscriminant macros when a string/index does not match an enum variant.

Macros (macros.rs)

    OtherFromError – Derives From<anyhow::Error> and From<std::io::Error> for any enum with an Other(String) variant.

    DefineVariantError – Generates a *ConfigError enum for a struct, creating Invalid<Field> and <Field>Unfound variants for each field.

pt_config – Configuration Management

This crate handles environment variables as defined in the PT specification.
Key Modules

    configs/configs.rs – Defines:

        CommonKey – shared settings (version, state dir, exit flag, outbound bind addresses).

        ClientKey – client‑specific: TOR_PT_CLIENT_TRANSPORTS, TOR_PT_PROXY.

        ServerKey – server‑specific: TOR_PT_SERVER_TRANSPORTS, TOR_PT_SERVER_TRANSPORT_OPTIONS, TOR_PT_SERVER_BINDADDR, TOR_PT_ORPORT, TOR_PT_EXTENDED_SERVER_PORT, TOR_PT_AUTH_COOKIE_FILE.

        TransportOptions – parsed from the semi‑colon separated string into a Vec<TransportOption> (each with name and settings map).

        validate_proxy_url() – checks proxy URI scheme, host, port, and credentials.

    configs/core.rs – ConfigKey enum (untagged) that holds either Server { server_key, common_key } or Client { client_key, common_key }. The init() method uses envy to deserialize from environment and panics on invalid/missing keys.

Derive Macros (macros.rs)

    FromString – Implements From<T> for String, TryFrom<String>, TryFrom<&str>, and FromStr for enums, matching against variant names.

    FromU8Index – Adds from_u8_index() and get_index() for enums with consecutive integer discriminants.

    FromDiscriminant – Adds from_discriminant() to map a discriminant value (as usize) to a variant.

    Builder – For structs: generates builder() with defaults (via #[deftly(default = "...")]) and with_<field>() setters.

Environment Loading

lib.rs initialises dotenvy once via a Lazy static, so .env files are loaded automatically.
pt_tracing – Logging and PT Messages

This crate provides a logging layer that outputs messages in the format expected by the Tor parent process (e.g., LOG SEVERITY=... MESSAGE=...). It also provides methods for PT‑specific control messages.
Core Types

    SEVERITY – Enum with DEBUG, INFO, NOTICE, WARNING, ERROR. Maps to tracing level filters.

    PtTracing – Configuration struct holding the current severity. Singleton via OnceLock.

        fmt() – returns a default instance with severity NOTICE.

        with_severity() – builder method.

        try_init() – initialises tracing_subscriber with compact, no‑time, no‑ANSI format and stores the config.

Trait: TorPtCommunicator

Defines associated types for each log level (all Result<()>) and static methods:

    debug, info, warn, error – log using tracing macros.

    notice – manually checks severity and prints via warn! (since tracing lacks a notice level).

    version, version_error, env_error, proxy_done, proxy_error, cmethod, cmethod_error, cmethods_done, smethod, smethod_error, smethods_done – output the exact PT control messages to stdout.

Macro: rec_panic!

Logs an error via TorPtCommunicator::error and then panics with the same message. Used for unrecoverable protocol failures.
Data Flow Summary (Client‑Side)

    Initialise – pt_config::ConfigKey::init() reads environment, panics on error.

    Create ExtOrPort – build with ExtOrPort::builder().with_addr(...).with_auth_cookie_file(...).build().

    Connect & Authenticate – connect() performs:

        TCP connection to ExtORPort.

        AuthTypes negotiation (reads AuthNegBuf).

        SAFE_COOKIE handshake (cookie file, nonce exchange, hash verification).

    Send Protocol Commands – Use done(), user_addr(), transport() on the TcpStream (split into read/write halves).

    Tunnel Traffic – After done(), all subsequent bytes are raw OR traffic (not handled in this codebase).

Concurrency & Asynchrony

    The entire crate uses tokio for async I/O (TcpStream, AsyncReadExt, AsyncWriteExt).

    No internal threading; concurrency is expected to be handled by the caller (e.g., using tokio::spawn per connection).

Testing

    Unit tests are present in pt_config::configs::test and pt_tracing::test.

    They use environment variable manipulation and direct assertion of parsed structures.

Dependencies (Key External)

    tokio – async runtime and I/O.

    hmac, sha2 – HMAC‑SHA256 for cookie authentication.

    serde, envy – deserialisation from environment.

    derive_deftly – custom derive macros for builders and conversions.

    thiserror, anyhow – error handling.

    tracing, tracing-subscriber – logging.

    url – proxy URI parsing.

    getrandom – random nonce generation.

Extensibility

    New authentication types can be added by extending AuthTypes and adding new states to ExtOrPortState.

    New protocol commands can be added to the trait ClientExtOrPortProtocol.

    The Builder macro allows easy configuration of ExtOrPort and other structs without boilerplate.

Current Limitations (as per extorport.md)

    Receiving/parsing of server responses (e.g., OKAY, DENY) is not implemented; the proxy is responsible for handling them outside this crate.

    Only SAFE_COOKIE authentication is supported.