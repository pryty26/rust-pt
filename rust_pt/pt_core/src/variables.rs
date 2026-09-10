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
