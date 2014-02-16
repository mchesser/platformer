use std::rc::Rc;
use std::cell::RefCell;
use std::num::clamp;
use std::io::File;

use sdl2::render::Renderer;
use sdl2::render::Texture;
use sdl2_image::LoadTexture;

use gmath::vectors::Vec2;
use gmath::shapes::Rect;
use game::entity::{Entity, PhysicalProperties};
use game::entity::creature::{Creature, CreatureAnimations};
use game::sprite::{Sprite, Animation};
use game::controller::Controller;
use game::controller::player::Player;
use game::map::Map;
use game::tiles::{TileSet, TileInfo};
use keyboard::KeyboardState;

mod map;
mod tiles;
mod entity;
mod controller;
mod sprite;

pub struct Game {
    map: Map,
    tileset: Rc<TileSet>,
    player: Player<Creature>,
    camera: Vec2<i32>,
    background: ~Texture,
}

impl Game {
    pub fn new(keyboard: Rc<RefCell<KeyboardState>>, renderer: &Renderer) -> Game {
        let tile_info = box [
            TileInfo { solid: false, friction: 0.0 },

            TileInfo { solid: true , friction: 1.0 },
            TileInfo { solid: true , friction: 1.0 },
            TileInfo { solid: true , friction: 1.0 },
            TileInfo { solid: true , friction: 1.0 },
            TileInfo { solid: true , friction: 1.0 },
            TileInfo { solid: true , friction: 1.0 },
            TileInfo { solid: true , friction: 1.0 },
            TileInfo { solid: true , friction: 1.0 },
            TileInfo { solid: true , friction: 1.0 },
            TileInfo { solid: true , friction: 1.0 },
            TileInfo { solid: true , friction: 1.0 },

            TileInfo { solid: true , friction: 1.0 },
            TileInfo { solid: true , friction: 1.0 },
            TileInfo { solid: true , friction: 1.0 },
            TileInfo { solid: true , friction: 1.0 },
            TileInfo { solid: true , friction: 1.0 },
            TileInfo { solid: true , friction: 1.0 },
            TileInfo { solid: true , friction: 1.0 },
            TileInfo { solid: true , friction: 1.0 },
            TileInfo { solid: true , friction: 1.0 },
            TileInfo { solid: true , friction: 1.0 },
            TileInfo { solid: true , friction: 1.0 },

            TileInfo { solid: true , friction: 1.0 },
            TileInfo { solid: true , friction: 1.0 },
            TileInfo { solid: true , friction: 1.0 },
            TileInfo { solid: true , friction: 1.0 },
            TileInfo { solid: true , friction: 1.0 },
            TileInfo { solid: true , friction: 1.0 },
            TileInfo { solid: true , friction: 1.0 },
            TileInfo { solid: true , friction: 1.0 },
            TileInfo { solid: true , friction: 1.0 },
            TileInfo { solid: true , friction: 1.0 },
            TileInfo { solid: true , friction: 1.0 },
        ];
        let tileset = Rc::new(TileSet {
            tile_size: 32,
            sprite: renderer.load_texture(&Path::new("./assets/tileset.png"))
                    .ok().expect("Failed to load tileset"),
            tile_info: tile_info
        });
        let map = Map::load_map(&mut File::open(&Path::new("./assets/maps/map1"))
                .ok().expect("Failed to load map"), tileset.clone());

        let player_spritesheet = Rc::new(renderer.load_texture(&Path::new("./assets/player.png"))
                .ok().expect("Failed to load player sprite"));

        let player = create_player(Vec2::new(50.0, 50.0), keyboard, player_spritesheet);

        let background = renderer.load_texture(&Path::new("./assets/background.png"))
                .ok().expect("Failed to load background image");

        Game {
            map: map,
            player: player,
            tileset: tileset,
            camera: Vec2::zero(),
            background: background
        }
    }

    pub fn update(&mut self, secs: f32) {
        let map = &self.map;
        self.player.update(map, secs);
    }

    pub fn draw(&mut self, renderer: &Renderer) {
        renderer.copy(self.background, None, None);

        // Center the camera on the player:
        let draw_rect = renderer.get_viewport();

        self.camera = self.player.entity.center()
                - Vec2::new((draw_rect.w as f32 / 2.0) as i32, (draw_rect.h as f32 / 2.0) as i32);
        self.camera.x = clamp(self.camera.x, 0, self.map.pixel_width() - draw_rect.w);
        self.camera.y = clamp(self.camera.y, 0, self.map.pixel_height() - draw_rect.h);

        self.map.draw(self.camera, renderer);
        self.player.entity.draw(self.camera, renderer);
    }
}

fn create_player(position: Vec2<f32>, keyboard: Rc<RefCell<KeyboardState>>,
        spritesheet: Rc<~Texture>) -> Player<Creature> {
    let fw = 64;
    let fh = 128;
    let idle = Animation {
        sprite: Sprite {
            spritesheet : spritesheet.clone(),
            offset      : Vec2::new(0, 0),
            frame_width : fw,
            frame_height: fh,
            num_frames_x: 1,
            num_frames_y: 1,
        },
        repeat: true,
        frame_time: 0.0
    };
    let walk = Animation {
        sprite: Sprite {
            spritesheet : spritesheet.clone(),
            offset      : Vec2::new(1*fw, 0),
            frame_width : fw,
            frame_height: fh,
            num_frames_x: 6,
            num_frames_y: 1,
        },
        repeat: true,
        frame_time: 0.7
    };
    let jump = Animation {
        sprite: Sprite {
            spritesheet : spritesheet.clone(),
            offset      : Vec2::new(7*fw, 1*fh),
            frame_width : fw,
            frame_height: fh,
            num_frames_x: 1,
            num_frames_y: 1,
        },
        repeat: true,
        frame_time: 0.6
    };
    let fall = Animation {
        sprite: Sprite {
            spritesheet : spritesheet.clone(),
            offset      : Vec2::new(8*fw, 1*fh),
            frame_width : fw,
            frame_height: fh,
            num_frames_x: 2,
            num_frames_y: 1,
        },
        repeat: true,
        frame_time: 0.5
    };

    Player {
        entity: Creature::new(
            position,
            Rect::new(14.0, 36.0, 32.0, 92.0),
            Rect::new(0.0, 0.0, 32.0, 32.0),
            PhysicalProperties {
                c_drag        : 0.470,
                mass          : 70.00, // (kg)
                acting_area   : 0.760, // (m^2)
                movement_accel: 6.000,
                max_velocity  : 9.000, // (m/s)
                jump_accel    : 5.000, // (m/s)
                jump_time     : 0.000, // (secs)
                stopping_bonus: 6.000,
            },
            CreatureAnimations {
                idle: idle,
                walk: walk,
                jump: jump,
                fall: fall,
            }
        ),
        keyboard: keyboard
    }
}
