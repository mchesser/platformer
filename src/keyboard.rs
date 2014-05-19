use std::mem;
use sdl2;
use sdl2::keycode::KeyCode;
use sdl2::scancode::ScanCode;
use collections::HashMap;

pub struct KeyboardState {
    last: HashMap<ScanCode, bool>,
    current: HashMap<ScanCode, bool>,
}

impl KeyboardState {
    pub fn new() -> KeyboardState {
        KeyboardState {
            last: sdl2::keyboard::get_keyboard_state(),
            current: sdl2::keyboard::get_keyboard_state(),
        }
    }

    pub fn update(&mut self) {
        self.last = mem::replace(&mut self.current, sdl2::keyboard::get_keyboard_state());
    }

    pub fn is_keydown(&self, keycode: KeyCode) -> bool {
        let scancode = sdl2::keyboard::get_scancode_from_key(keycode);
        *self.current.get(&scancode)
    }

    pub fn is_new_keypress(&self, keycode: KeyCode) -> bool {
        let scancode = sdl2::keyboard::get_scancode_from_key(keycode);
        *self.current.get(&scancode) && !*self.last.get(&scancode)
    }
}
