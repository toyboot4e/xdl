/*!

Extensional input layer on top of Rust-SDL2 or Rokol

Port of FNA input module + virtual input, intended for games with orthoghogonal grid maps.

# Getting started

Create [`Input`] with and manage the lifecycle. See [`vi`] module for virtual input.

# WIP

* done: keyboard input
* WIP: mouse input
* WIP: gamepad
* not started: touch input

*/

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
