/*!
Input state backed by Rust-SDL2, `rokol::app` or your implementation

# Getting started

Create [`Input`] and manage the lifecycle. See [`vi`] module for virtual input.

TODO: mouse, gamepad, touch
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
