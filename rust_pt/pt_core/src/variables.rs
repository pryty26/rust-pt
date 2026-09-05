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
