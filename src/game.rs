use std::rc::Rc;
use std::cell::RefCell;
use std::num::clamp;
use std::io::File;

use sdl2::render::Renderer;
use sdl2_image::LoadTexture;

use gmath::vectors::Vec2;
use game::entity::Entity;
use game::entity::creature::Creature;
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
    camera: Vec2<i32>
}

impl Game {
    pub fn new(keyboard: Rc<RefCell<KeyboardState>>, renderer: &Renderer) -> Game {
        let tile_info = box [
            TileInfo { solid: false, friction: 0.0 },
            TileInfo { solid: true , friction: 1.0 },
            TileInfo { solid: true , friction: 1.0 },
            TileInfo { solid: true , friction: 1.0 },
            TileInfo { solid: true , friction: 1.0 },
        ];

        let tileset_texture = renderer.load_texture(&Path::new("./assets/tileset.png"))
                .ok().expect("Failed to load tileset");

        let tileset = Rc::new(TileSet {
            tile_size: 32,
            sprite: tileset_texture,
            tile_info: tile_info
        });

        let mut map_file = File::open(&Path::new("./assets/maps/map1"))
                .ok().expect("Failed to load map");

        let player_spritesheet = Rc::new(renderer.load_texture(&Path::new("./assets/player.png"))
                .ok().expect("Failed to load player sprite"));

        let player = Player {
            entity: Creature::new(Vec2::new(50.0, 50.0), player_spritesheet.clone()),
            keyboard: keyboard
        };


        Game {
            map: Map::load_map(&mut map_file, tileset.clone()),
            player: player,
            tileset: tileset,
            camera: Vec2::zero(),
        }
    }

    pub fn update(&mut self, secs: f32) {
        let map = &self.map;
        self.player.update(map, secs);
    }

    pub fn draw(&mut self, renderer: &Renderer) {
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
