//! XDL: An extensional input layer on top of Rust-SDL2
//!
//! Port of FNA input module + Virtual input.
//!
//! Intended for games with orghogonal grid maps.
//!
//! # Getting started
//!
//! Create [`Input`] with raw SDL window and manage their lifecycle.
//!
//! See [`vi`] module for virtual input.
//!
//! # WIP
//!
//! * WIP: keyboard input (key repeat?)
//! * WIP: mouse input
//! * not started: touch input

pub use num_enum;
pub use sdl2;

pub mod axis;
pub mod utils;
pub mod vi;

mod input;

pub use input::{
    keyboard::{Key, Keyboard},
    mouse::{Mouse, MouseInput},
    Input,
};
