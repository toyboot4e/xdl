/*!
Input states for Rust-SDL2 (or any other)

# Getting started

Create [`Input`] and manage the lifecycle. See [`vi`] module for virtual input.

# TODOs

mouse, gamepad, touchpad, more virtual input, ..
*/

pub mod utils;
pub mod vi;

mod axis;
mod input;
mod platform;

pub use crate::{
    axis::*,
    input::{
        keyboard::{Key, Keyboard},
        Input,
    },
};
