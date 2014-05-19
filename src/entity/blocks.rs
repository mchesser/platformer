use game::entity::Object;
use game::map::Map;
use game::sprite::{Animation, AnimationPlayer};

use gmath::vectors::Vec2;
use gmath::shapes::Rect;

use sdl2::render::Renderer;

pub struct DamageBlock {
    block_rect: Rect,
    damage: f32,
    animation_player: AnimationPlayer,
}

impl DamageBlock {
    pub fn new(block_rect: Rect, damage: f32, animation: Animation) -> DamageBlock {
        DamageBlock {
            block_rect: block_rect,
            damage: damage,
            animation_player: AnimationPlayer::new(animation),
        }
    }
}

impl Object for DamageBlock {
    fn position(&self) -> Vec2<f32> {
        Vec2::new(self.block_rect.x, self.block_rect.y) 
    }
    
    fn set_position(&mut self, new_pos: Vec2<f32>) {
        self.block_rect.x = new_pos.x;
        self.block_rect.y = new_pos.y; 
    }
    
    fn bounds(&self) -> Rect {
        self.block_rect
    }
    
    fn update(&mut self, _: &Map, secs: f32) {
        self.animation_player.update(secs);
    }
    
    fn draw(&self, camera: Vec2<i32>, renderer: &Renderer) {
        let pos = Vec2::new(self.block_rect.x as i32, self.block_rect.y as i32) - camera;
        self.animation_player.draw(pos, renderer);
    }
}
