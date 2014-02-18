use std::rand;
use std::rand::Rng;
use std::rand::XorShiftRng;

use gmath::vectors::Vec2;
use game::map::Map;
use game::entity::Entity;
use game::entity::creature::Creature;
use game::controller::Controller;

pub struct RandomWalker {
    entity: Creature,
    last_dir: f32,
    rng: XorShiftRng,
    move_time: f32,
    wait_time: f32,
}

impl RandomWalker {
    pub fn new(entity: Creature) -> RandomWalker {
        RandomWalker {
            entity: entity,
            last_dir: 0.0,
            rng: rand::weak_rng(),
            move_time: 1.0,
            wait_time: 0.0,
        }
    }
}

impl Controller for RandomWalker {
    fn update(&mut self, map: &Map, secs: f32) {
        self.wait_time += secs;
        if self.wait_time > self.move_time {
            let move_accel = self.entity.physical_properties().movement_accel;
            let x_accel = match self.rng.gen::<f32>() {
                dir if dir < 0.5  => 0.0,
                dir if dir < 0.75 => move_accel,
                _                 => -move_accel,
            };
            let new_accel = Vec2::new(x_accel, self.entity.acceleration().y);
            self.entity.set_acceleration(new_accel);
            self.wait_time -= self.move_time;
        }

        self.entity.update(map, secs);
    }
}
