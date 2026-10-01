// File: rust_pt\pt_core\src\variables.rs
// Directory: rust_pt\pt_core\src
// Filename: variables.rs
//======================================================================

#![allow(nonstandard_style)]
/// Cookie-file format
/// ```text
/// The format of the cookie-file is:
///
///      StaticHeader                                [32 octets]
///      Cookie                                      [32 octets]
///
///   Where,
///   + StaticHeader is the following string:
///     "! Extended ORPort Auth Cookie !\x0a"
///   + Cookie is the shared-secret. During the SAFE_COOKIE protocol, the
///     cookie is called CookieString.
/// ```
pub const StaticHeader: [u8; 32] = *b"! Extended ORPort Auth Cookie !\x0a";
/// [0x0000] DONE: no more information to give.
pub const CMD_DONE: u16 = 0x0000;
/// An address:port string of the client.
pub const CMD_USERADDR: u16 = 0x0001;
/// The PT name in effect on this connection.
pub const CMD_TRANSPORT: u16 = 0x0002;

/// SOCKS protocol version 5
pub const SocksVersion: u8 = 0x05;

/// No authentication required
pub const SocksAuthNoneRequired: u8 = 0x00;
/// Username/password authentication
pub const SocksAuthUsernamePassword: u8 = 0x02;
/// No acceptable authentication methods
pub const SocksAuthNoAcceptableMethods: u8 = 0xff;

/// CONNECT command
pub const SocksCmdConnect: u8 = 0x01;
/// Reserved byte
pub const SocksRsv: u8 = 0x00;

/// IPv4 address type
pub const SocksAtypeV4: u8 = 0x01;
/// Domain name address type
pub const SocksAtypeDomainName: u8 = 0x03;
/// IPv6 address type
pub const SocksAtypeV6: u8 = 0x04;

/// RFC 1929 authentication version
pub const SocksAuthRFC1929Ver: u8 = 0x01;
/// RFC 1929 authentication success
pub const SocksAuthRFC1929Success: u8 = 0x00;
/// RFC 1929 authentication failure
pub const SocksAuthRFC1929Fail: u8 = 0x01;

/// Succeeded
pub const SocksRepSucceeded: u8 = 0x00;
/// General SOCKS server failure
pub const SocksRepGeneralFailure: u8 = 0x01;
/// Connection not allowed by ruleset
pub const SocksRepConnectionNotAllowed: u8 = 0x02;
/// Network unreachable
pub const SocksRepNetworkUnreachable: u8 = 0x03;
/// Host unreachable
pub const SocksRepHostUnreachable: u8 = 0x04;
/// Connection refused
pub const SocksRepConnectionRefused: u8 = 0x05;
/// TTL expired
pub const SocksRepTTLExpired: u8 = 0x06;
/// Command not supported
pub const SocksRepCommandNotSupported: u8 = 0x07;
/// Address type not supported
pub const SocksRepAddressNotSupported: u8 = 0x08;