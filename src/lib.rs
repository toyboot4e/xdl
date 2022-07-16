/*!
Input states for Rust-SDL2 (or any other)

# Getting started

Create [`Input`] and manage the lifecycle. See [`vi`] module for virtual input.

# TODOs

easier serde, mouse, gamepad, touchpad, more virtual input, ..
*/

pub mod backend;
pub mod utils;
pub mod vi;

mod axis;
mod input;

pub use crate::{
    axis::*,
    input::{
        keyboard::{Key, Keyboard},
        Input,
    },
};

/// Updates [`Input`] for a specific platform such as SDL2
pub trait Backend {
    type Event;
    type Key;

    fn on_event(&self, input: &mut Input, ev: &Self::Event);
    fn on_end_frame(&self, input: &mut Input);
}
