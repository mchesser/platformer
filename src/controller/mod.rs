use std::rand;
use std::rand::Rng;
use std::rand::XorShiftRng;
use std::rc::Rc;
use std::cell::RefCell;

use gmath::vectors::Vec2;
use game::entity::Physics;
use game::entity::creature::Creature;
use keyboard::KeyboardState;
use sdl2::keycode;

pub trait Controller<A> {
    fn update(&mut self, object: &mut A, secs: f32);
}

/// A controller that controls objects using the keyboard
pub struct KeyboardController {
    keyboard: Rc<RefCell<KeyboardState>>
}

impl KeyboardController {
    pub fn new(keyboard: Rc<RefCell<KeyboardState>>) -> KeyboardController {
        KeyboardController {
            keyboard: keyboard,
        }
    }
}

impl Controller<Creature> for KeyboardController {
    fn update(&mut self, object: &mut Creature, _secs: f32) {
        let keyboard = self.keyboard.borrow().borrow();

        let move_accel = object.move_accel;
        let x_accel =
            if keyboard.get().is_keydown(keycode::LeftKey) {
                -move_accel * if object.is_on_ground() { 1.0 } else { 0.6 }
            }
            else if keyboard.get().is_keydown(keycode::RightKey) {
                move_accel * if object.is_on_ground() { 1.0 } else { 0.6 }
            }
            else {
                0.0
            };
        let new_accel = Vec2::new(x_accel, object.acceleration().y);
        object.set_acceleration(new_accel);

        let jump_accel = object.jump_accel;
        if object.is_on_ground() && keyboard.get().is_new_keypress(keycode::UpKey) {
            let new_velocity = object.velocity() + Vec2::new(0.0, -jump_accel);
            object.set_velocity(new_velocity);
        }
    }
}

/// A controller that controls objects using randomness
pub struct RandomController {
    rng: XorShiftRng,
    move_time: f32,
    wait_time: f32,
}

impl RandomController {
    pub fn new(move_time: f32) -> RandomController {
        RandomController {
            rng: rand::weak_rng(),
            move_time: move_time,
            wait_time: 0.0,
        }
    }
}

impl Controller<Creature> for RandomController {
    fn update(&mut self, object: &mut Creature, secs: f32) {
        self.wait_time += secs;
        if self.wait_time > self.move_time {
            let move_accel = object.move_accel;
            let x_accel = match self.rng.gen::<f32>() {
                dir if dir < 0.5  => 0.0,
                dir if dir < 0.75 => move_accel,
                _                 => -move_accel,
            };
            let new_accel = Vec2::new(x_accel, object.acceleration().y);
            object.set_acceleration(new_accel);
            self.wait_time -= self.move_time;
        }
    }
}
