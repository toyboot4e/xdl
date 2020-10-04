pub mod keyboard;
pub mod mouse;

use sdl2::event::Event;

use keyboard::Keyboard;
use mouse::Mouse;

/// All of the input states
#[derive(Debug, Clone)]
pub struct Input {
    pub kbd: Keyboard,
    pub mouse: Mouse,
}

impl Input {
    pub fn new(win: *mut sdl2::sys::SDL_Window) -> Self {
        Self {
            kbd: Keyboard::default(),
            mouse: Mouse::new(win),
        }
    }
}

/// Lifecycle
impl Input {
    pub fn event(&mut self, ev: &Event) {
        self.kbd.event(ev);
        self.mouse.event(ev);
    }

    pub fn update(&mut self) {
        self.mouse.update();
    }

    pub fn on_end_frame(&mut self) {
        self.kbd.on_end_frame();
        self.mouse.on_end_frame();
    }
}
