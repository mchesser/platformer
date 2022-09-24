use macroquad::{
    prelude::{Rect, UVec2, Vec2, WHITE},
    texture::{draw_texture_ex, DrawTextureParams, Texture2D},
};

pub static NEXT_ANIMATION_ID: std::sync::atomic::AtomicUsize =
    std::sync::atomic::AtomicUsize::new(0);

#[derive(Clone)]
pub struct Sprite {
    pub spritesheet: Texture2D,
    pub offset: UVec2,
    pub frame_width: u32,
    pub frame_height: u32,
    pub num_frames_x: u32,
    pub num_frames_y: u32,
}

impl Sprite {
    pub fn draw(&self, frame: UVec2, pos: Vec2, flip_x: bool) {
        assert!(frame.x < self.num_frames_x);
        assert!(frame.y < self.num_frames_y);

        let source_rect = Rect::new(
            self.offset.x as f32 + frame.x as f32 * self.frame_width as f32,
            self.offset.y as f32 + frame.y as f32 * self.frame_height as f32,
            self.frame_width as f32,
            self.frame_height as f32,
        );

        let pos = pos.round();
        let dest_rect = Rect::new(pos.x, pos.y, self.frame_width as f32, self.frame_height as f32);

        draw_texture_ex(self.spritesheet, dest_rect.x, dest_rect.y, WHITE, DrawTextureParams {
            dest_size: Some(dest_rect.size()),
            source: Some(source_rect),
            flip_x,
            ..Default::default()
        });
    }
}

#[derive(Clone)]
pub struct Animation {
    pub id: usize,
    pub sprite: Sprite,
    pub frame_time: f32,
    pub repeat: bool,
}

pub struct AnimationPlayer {
    pub speed_up: f32,
    animation: Animation,
    frame: UVec2,
    wait_time: f32,
    stopped: bool,
    flip_x: bool,
}

impl AnimationPlayer {
    pub fn new(animation: Animation) -> Self {
        Self {
            animation,
            frame: UVec2::ZERO,
            speed_up: 1.0,
            wait_time: 0.0,
            stopped: false,
            flip_x: false,
        }
    }

    pub fn play(&mut self, animation: Animation) {
        if self.animation.id != animation.id {
            self.animation = animation;
            self.reset();
        }
    }

    pub fn reset(&mut self) {
        self.frame = UVec2::ZERO;
        self.wait_time = 0.0;
        self.speed_up = 1.0;
        self.stopped = false;
    }

    pub fn flip_horizontal(&mut self, flip: bool) {
        self.flip_x = flip;
    }

    pub fn update(&mut self, secs: f32) {
        if !self.stopped && self.animation.frame_time != 0.0 {
            self.wait_time += secs;
            while self.wait_time > self.animation.frame_time * self.speed_up {
                self.wait_time -= self.animation.frame_time * self.speed_up;
                self.frame.x = if self.frame.x + 1 < self.animation.sprite.num_frames_x {
                    self.frame.x + 1
                }
                else {
                    if !self.animation.repeat {
                        self.stopped = true;
                        break;
                    }
                    else {
                        0
                    }
                };
            }
        }
    }

    pub fn draw(&self, pos: Vec2) {
        self.animation.sprite.draw(self.frame, pos, self.flip_x);
    }
}
