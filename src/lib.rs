/*!

XDL: An extensional input layer on top of Rust-SDL2 or Rokol

Port of FNA input module + Virtual input. Intended for games with orthoghogonal grid maps.

# Getting started

Create [`Input`] with raw SDL window and manage the lifecycle.

See [`vi`] module for virtual input.

# WIP

* done: keyboard input
* WIP: mouse input
* WIP: gamepad
* not started: touch input

*/

pub mod axis;
pub mod utils;
pub mod vi;

mod input;

pub use crate::input::{
    keyboard::{Key, Keyboard},
    Input,
};
