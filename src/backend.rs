/*!
[`Backend`] implementations

[`Backend`]: ../Backend
*/

#[cfg(feature = "sdl2")]
mod sdl2_support;

#[cfg(feature = "sdl2")]
pub extern crate sdl2;

#[cfg(feature = "sdl2")]
pub use self::sdl2_support::SdlBackend;
