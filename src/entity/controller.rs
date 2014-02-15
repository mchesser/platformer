use std::rc::Rc;
use std::cell::RefCell;

use game::map::Map;
use game::entity::Entity;
use keyboard::KeyboardState;

use gmath::vectors::Vec2;

use sdl2::keycode;

pub trait Controller {
    fn update(&mut self, map: &Map, secs: f32);
}

pub struct Player<T> {
    entity  : T,
    keyboard: Rc<RefCell<KeyboardState>>
}

impl<T: Entity> Controller for Player<T> {
    fn update(&mut self, map: &Map, secs: f32) {
        let keyboard = self.keyboard.borrow().borrow();

        let move_accel = self.entity.physical_properties().movement_accel;
        let x_accel =
            if keyboard.get().is_keydown(keycode::LeftKey) {
                -move_accel * if self.entity.is_on_ground() { 1.0 } else { 0.6 }
            }
            else if keyboard.get().is_keydown(keycode::RightKey) {
                move_accel * if self.entity.is_on_ground() { 1.0 } else { 0.6 }
            }
            else {
                0.0
            };
        let new_accel = Vec2::new(x_accel, self.entity.acceleration().y);
        self.entity.set_acceleration(new_accel);

        let jump_accel = self.entity.physical_properties().jump_accel;
        if self.entity.is_on_ground() && keyboard.get().is_new_keypress(keycode::UpKey) {
            let new_velocity = self.entity.velocity() + Vec2::new(0.0, -jump_accel);
            self.entity.set_velocity(new_velocity);
        }

        self.entity.update(map, secs);
    }
}
