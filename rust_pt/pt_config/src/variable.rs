#![allow(missing_docs)]

pub const CURRENT_TRANSPORT_VER: &str = "1";
pub const MANAGED_VER: &str = "TOR_PT_MANAGED_TRANSPORT_VER";
pub const STATE_LOCATION: &str = "TOR_PT_STATE_LOCATION";
pub const CLIENT_TRANSPORTS: &str = "TOR_PT_CLIENT_TRANSPORTS";
pub const PROXY: &str = "TOR_PT_PROXY";
pub const SERVER_TRANSPORTS: &str = "TOR_PT_SERVER_TRANSPORTS";
pub const SERVER_TRANSPORT_OPTIONS: &str = "TOR_PT_SERVER_TRANSPORT_OPTIONS";
pub const SERVER_BINDADDR: &str = "TOR_PT_SERVER_BINDADDR";
pub const AUTH_COOKIE_FILE: &str = "TOR_PT_AUTH_COOKIE_FILE";
pub const ORPORT: &str = "TOR_PT_ORPORT";
pub const EXTENDED_SERVER_PORT: &str = "TOR_PT_EXTENDED_SERVER_PORT";
pub const EXIT_ON_STDIN_CLOSE: &str = "TOR_PT_EXIT_ON_STDIN_CLOSE";

/// Proxies which are supported
pub const SCHEMES: [&str; 3] = ["socks5", "socks4a", "http"];
