use std::rc::Rc;
use std::cell::RefCell;

use game::entity;
use game::entity::Entity;
use game::entity::PhysicalProperties;
use game::map::Map;
use game::sprite::{Sprite, Animation};

use gmath::vectors::Vec2;
use gmath::shapes::Rect;

use keyboard::KeyboardState;

use sdl2::keycode;
use sdl2::render::Renderer;
use sdl2::render::Texture;

pub struct Player {
    accel      : Vec2<f32>,
    vel        : Vec2<f32>,
    pos        : Vec2<f32>,
    base_bounds: Rect,
    base_hitbox: Rect,
    keyboard   : Rc<RefCell<KeyboardState>>,
    on_ground  : bool,
    properties : PhysicalProperties,
    animation  : Animation,
}

impl Entity for Player {
    fn acceleration(&self) -> Vec2<f32> { self.accel }
    fn set_acceleration(&mut self, new_accel: Vec2<f32>) { self.accel = new_accel; }
    fn velocity(&self) -> Vec2<f32> { self.vel }
    fn set_velocity(&mut self, new_vel: Vec2<f32>) { self.vel = new_vel; }
    fn position(&self) -> Vec2<f32> { self.pos }
    fn set_position(&mut self, new_pos: Vec2<f32>) { self.pos = new_pos }
    fn physical_properties(&self) -> PhysicalProperties { self.properties }
    fn bounds(&self) -> Rect { self.base_bounds.move_vec(self.pos) }
    fn is_on_ground(&self) -> bool { self.on_ground }
    fn hit_y(&mut self, value: bool) { self.on_ground = value }
}

impl Player {
    pub fn new(position: Vec2<f32>, spritesheet: Rc<~Texture>,
            keyboard: Rc<RefCell<KeyboardState>>) -> Player {

        let sprite = Sprite {
            spritesheet: spritesheet,
            offset: Vec2::new(32*2, 0),
            frame_width: 32*2,
            frame_height: 64*2,
            num_frames_x: 6,
            num_frames_y: 1,
        };

        Player {
            accel: Vec2::new(0.0, 9.8),
            vel: Vec2::zero(),
            pos: position,
            base_bounds: Rect::new(0.0, 0.0, 32.0, 90.0),
            base_hitbox: Rect::new(0.0, 0.0, 32.0, 32.0),
            keyboard: keyboard,
            on_ground: false,
            properties: PhysicalProperties {
                c_drag        : 0.470,
                mass          : 70.00, // (kg)
                acting_area   : 0.760, // (m^2)
                movement_accel: 5.000,
                max_velocity  : 7.000, // (m/s)
                jump_accel    : 5.000, // (m/s)
                jump_time     : 0.000, // (secs)
                stopping_bonus: 1.000,
            },
            animation: Animation::new(sprite, 0.2),
        }
    }

    pub fn update(&mut self, map: &Map, secs: f32) {
        self.handle_input();
        entity::physics(self, map, secs);
        self.animation.update(secs);
    }

    fn handle_input(&mut self) {
        let keyboard = self.keyboard.borrow().borrow();

        self.accel.x =
            if keyboard.get().is_keydown(keycode::LeftKey) {
                -self.properties.movement_accel * if self.on_ground { 1.0 } else { 0.6 }
            }
            else if keyboard.get().is_keydown(keycode::RightKey) {
                self.properties.movement_accel * if self.on_ground { 1.0 } else { 0.6 }
            }
            else {
                0.0
            };

        if keyboard.get().is_new_keypress(keycode::UpKey) {
            self.vel = self.vel + Vec2::new(0.0, -self.properties.jump_accel);
        }
    }

    pub fn draw(&self, renderer: &Renderer) {
        let pos = Vec2::new(self.pos.x as i32, self.pos.y as i32);
        self.animation.draw(pos, renderer);
    }
}
