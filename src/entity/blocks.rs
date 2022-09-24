use macroquad::prelude::{Rect, Vec2};

use crate::{
    entity::Object,
    map::Map,
    sprite::{Animation, AnimationPlayer},
};

pub struct DamageBlock {
    block_rect: Rect,
    _damage: f32,
    animation_player: AnimationPlayer,
}

impl DamageBlock {
    pub fn new(block_rect: Rect, damage: f32, animation: Animation) -> Self {
        Self {
            block_rect,
            _damage: damage,
            animation_player: AnimationPlayer::new(animation),
        }
    }
}

impl Object for DamageBlock {
    fn position(&self) -> Vec2 {
        Vec2::new(self.block_rect.x, self.block_rect.y)
    }

    fn set_position(&mut self, new_pos: Vec2) {
        self.block_rect.x = new_pos.x;
        self.block_rect.y = new_pos.y;
    }

    fn bounds(&self) -> Rect {
        self.block_rect
    }

    fn update(&mut self, _: &Map, secs: f32) {
        self.animation_player.update(secs);
    }

    fn draw(&self, camera: Vec2) {
        self.animation_player.draw(self.block_rect.point() - camera);
    }
}
