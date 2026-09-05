// File: rust_pt\pt_tracing\src\traits.rs
// Directory: rust_pt\pt_tracing\src
// Filename: traits.rs
//======================================================================

/// Print a debug message, conform with Tor-Pt Spec
pub trait TorPtCommunicator {
    /// Output for debug
    type DebugOutput;
    /// Output for info
    type InfoOutput;
    /// Output for error
    type ErrorOutput;
    /// Output for notice
    type NoticeOutput;
    /// Output for warn
    type WarnOutput;

    /// Print an info message, conform with Tor-Pt Spec
    fn debug(message: &str) -> Self::DebugOutput;

    /// Print an info message, conform with Tor-Pt Spec
    fn info(message: &str) -> Self::InfoOutput;

    /// Print an error message, conform with Tor-Pt Spec
    fn error(message: &str) -> Self::ErrorOutput;

    /// Print a notice message, conform with Tor-Pt Spec
    /// Unfortunately tracing do not have "notice" level. So we need to use manual if+println! instead.
    fn notice(message: &str) -> Self::NoticeOutput;

    /// Print a warning message, conform with Tor-Pt Spec
    fn warn(message: &str) -> Self::WarnOutput;

    /// After version negotiation has been completed, the PT proxy must then
    /// validate that all of the required environment variables are provided,
    /// and that all of the configuration values supplied are well formed.
    ///
    /// At any point, if there is an error encountered related to configuration
    /// supplied via the environment variables, it MAY respond with an error
    /// message and terminate.
    ///
    /// `ENV-ERROR <ErrorMessage>`
    ///
    /// The "ENV-ERROR" message is used to signal the PT proxy's failure to parse
    /// the configuration environment variables (3.2).
    ///
    /// The `<ErrorMessage>` SHOULD consist of a useful error message that can be
    /// used to diagnose and correct the root cause of the failure.
    ///
    /// PT proxies MUST terminate after outputting a "ENV-ERROR" message.
    ///
    /// Example:
    ///
    /// `ENV-ERROR No TOR_PT_AUTH_COOKIE_FILE when TOR_PT_EXTENDED_SERVER_PORT set`
    fn env_error(msg: &str);

    /// When a PT proxy first starts up, it must determine which version of the
    /// Pluggable Transports Specification to use to configure itself.
    ///
    /// It does this via the `TOR_PT_MANAGED_TRANSPORT_VER` (3.2.1) environment
    /// variable which contains all of the versions supported by the application.
    ///
    /// Upon determining the version to use, or lack thereof, the PT proxy
    /// responds with one of two messages.
    ///
    /// `VERSION-ERROR <ErrorMessage>`
    ///
    /// The "VERSION-ERROR" message is used to signal that there was no compatible
    /// Pluggable Transport Specification version present in the
    /// `TOR_PT_MANAGED_TRANSPORT_VER` list.
    ///
    /// The `<ErrorMessage>` SHOULD be set to "no-version" for historical reasons
    /// but MAY be set to a useful error message instead.
    ///
    /// PT proxies MUST terminate after outputting a "VERSION-ERROR" message.
    fn version_error(msg: &str);

    /// After negotiating the Pluggable Transport Specification version, PT client
    /// proxies MUST first validate `TOR_PT_PROXY` (3.2.2) if it is set, before
    /// initializing any transports.
    ///
    /// Assuming that an upstream proxy is provided, PT client proxies MUST
    /// respond with a message indicating that the proxy is valid, supported, and
    /// will be used OR a failure message.
    fn proxy_done();

    /// The `VERSION` message is used to signal the Pluggable Transport
    /// Specification version that the PT proxy will use to configure its
    /// transports and communicate with the parent process.
    ///
    /// The version for the environment values and reply messages specified
    /// by the PT Specification is `1`.
    ///
    /// PT proxies MUST either report an error and terminate, or output a
    /// `VERSION` message before moving on to client/server proxy initialization
    /// and configuration.
    ///
    /// Example:
    ///
    /// `VERSION 1`
    fn version(version: &str);

    /// The `PROXY-ERROR` message is used to signal that the upstream proxy
    /// specified by `TOR_PT_PROXY` is malformed, unsupported, or otherwise
    /// unusable.
    ///
    /// PT proxies MUST terminate immediately after outputting a
    /// `PROXY-ERROR` message.
    ///
    /// Example:
    ///
    /// `PROXY-ERROR SOCKS 4 upstream proxies unsupported.`
    fn proxy_error(msg: &str);

    /// The `CMETHOD` message is used to signal that a requested PT transport
    /// has been launched, the protocol which the parent should use to make
    /// outgoing connections, and the IP address and port that the PT transport
    /// is listening on.
    ///
    /// The protocol MUST be either `socks4` or `socks5`.
    ///
    /// Example:
    ///
    /// `CMETHOD trebuchet socks5 127.0.0.1:19999`
    fn cmethod(transport: &str, proxy_type: &str, address: &str);

    /// The `CMETHOD-ERROR` message is used to signal that a requested PT
    /// transport was unable to be launched.
    ///
    /// Example:
    ///
    /// `CMETHOD-ERROR trebuchet no rocks available`
    fn cmethod_error(transport: &str, msg: &str);

    /// The `CMETHODS DONE` message signals that the PT proxy has finished
    /// initializing all of the transports that it is capable of handling.
    ///
    /// Upon sending the `CMETHODS DONE` message, the PT proxy initialization
    /// is complete.
    fn cmethods_done();

    /// The `SMETHOD` message is used to signal that a requested PT transport
    /// has been launched, the protocol which will be used to handle incoming
    /// connections, and the IP address and port that clients should use to
    /// reach the reverse proxy.
    ///
    /// The optional `options` field is used to pass additional per-transport
    /// information back to the parent process.
    ///
    /// Example:
    ///
    /// `SMETHOD trebuchet 198.51.100.1:19999`
    ///
    /// `SMETHOD rot_by_N 198.51.100.1:2323 ARGS:N=13`
    fn smethod(transport: &str, address: &str, options: Option<&str>);

    /// The `SMETHOD-ERROR` message is used to signal that a requested PT
    /// transport reverse proxy was unable to be launched.
    ///
    /// Example:
    ///
    /// `SMETHOD-ERROR trebuchet no cows available`
    fn smethod_error(transport: &str, msg: &str);

    /// The `SMETHODS DONE` message signals that the PT proxy has finished
    /// initializing all of the transports that it is capable of handling.
    ///
    /// Upon sending the `SMETHODS DONE` message, the PT proxy initialization
    /// is complete.
    fn smethods_done();
}
