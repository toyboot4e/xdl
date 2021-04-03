/*!
Platform-dependent implementations
*/

#[cfg(feature = "sdl2")]
mod sdl2;
#[cfg(feature = "sdl2")]
pub use self::sdl2::*;

#[cfg(feature = "rokol")]
mod rokol;
#[cfg(feature = "rokol")]
pub use self::rokol::*;

#[cfg(not(any(feature = "sdl2", feature = "rokol")))]
mod dummy {
    pub type ExternalKey = u32;
    pub type Event = ();

    use crate::input::{keyboard::Key, Input};
    use std::collections::HashMap;

    impl Input {
        pub fn new() -> Self {
            unimplemented!()
        }

        pub fn event(&mut self, _ev: &Event) {
            unimplemented!()
        }

        pub fn on_end_frame(&mut self) {
            unimplemented!()
        }
    }

    pub fn key_translation() -> HashMap<ExternalKey, Key> {
        unimplemented!()
    }

    impl Keyboard {
        pub fn event(&mut self, ev: &Event) {
            unimplemented!()
        }
    }
}

#[cfg(not(any(feature = "sdl2", feature = "rokol")))]
pub use dummy::*;
