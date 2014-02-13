use std::hashmap::HashMap;
use std::util;
use sdl2;
use sdl2::keycode::KeyCode;
use sdl2::scancode::ScanCode;


pub struct KeyboardState {
    priv last: ~HashMap<ScanCode, bool>,
    priv current: ~HashMap<ScanCode, bool>
}

impl KeyboardState {
    pub fn new() -> KeyboardState {
        KeyboardState {
            last: sdl2::keyboard::get_keyboard_state(),
            current: sdl2::keyboard::get_keyboard_state()
        }
    }

    pub fn update(&mut self) {
        // !!! FIXME: Change to mem::replace in latest rust
        self.last = util::replace(&mut self.current, sdl2::keyboard::get_keyboard_state());
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
