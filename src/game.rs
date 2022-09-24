use anyhow::Context;
use macroquad::{
    prelude::{Rect, UVec2, Vec2, WHITE},
    texture::{draw_texture, load_texture, Texture2D},
    window::{screen_height, screen_width},
};

use crate::{
    bitfont::BitFont,
    controller::{KeyboardController, NoneController, RandomController},
    entity::{
        blocks::DamageBlock,
        creature::{Creature, CreatureAnimations},
        Entity, PhysicalProperties,
    },
    map::Map,
    sprite::{Animation, Sprite, NEXT_ANIMATION_ID},
    tiles::{TileInfo, TileSet},
};

pub struct Game {
    map: Map,
    player: Entity<Creature, KeyboardController>,
    cat: Entity<Creature, RandomController>,
    lava: Vec<Entity<DamageBlock, NoneController<DamageBlock>>>,
    font: BitFont,
    camera: Vec2,
    background: Texture2D,
}

impl Game {
    pub async fn new() -> anyhow::Result<Self> {
        #[rustfmt::skip]
        let tile_info = vec![
            TileInfo { solid: false, friction: 0.0 },

            TileInfo { solid: true, friction: 1.0 },
            TileInfo { solid: true, friction: 1.0 },
            TileInfo { solid: true, friction: 1.0 },
            TileInfo { solid: true, friction: 1.0 },
            TileInfo { solid: true, friction: 1.0 },
            TileInfo { solid: true, friction: 1.0 },
            TileInfo { solid: true, friction: 1.0 },
            TileInfo { solid: true, friction: 1.0 },
            TileInfo { solid: true, friction: 1.0 },
            TileInfo { solid: true, friction: 1.0 },
            TileInfo { solid: true, friction: 1.0 },

            TileInfo { solid: true, friction: 1.0 },
            TileInfo { solid: true, friction: 1.0 },
            TileInfo { solid: true, friction: 1.0 },
            TileInfo { solid: true, friction: 1.0 },
            TileInfo { solid: true, friction: 1.0 },
            TileInfo { solid: true, friction: 1.0 },
            TileInfo { solid: true, friction: 1.0 },
            TileInfo { solid: true, friction: 1.0 },
            TileInfo { solid: true, friction: 1.0 },
            TileInfo { solid: true, friction: 1.0 },
            TileInfo { solid: true, friction: 1.0 },

            TileInfo { solid: true, friction: 1.0 },
            TileInfo { solid: true, friction: 1.0 },
            TileInfo { solid: true, friction: 1.0 },
            TileInfo { solid: true, friction: 1.0 },
            TileInfo { solid: true, friction: 1.0 },
            TileInfo { solid: true, friction: 1.0 },
            TileInfo { solid: true, friction: 1.0 },
            TileInfo { solid: true, friction: 1.0 },
            TileInfo { solid: true, friction: 1.0 },
            TileInfo { solid: true, friction: 1.0 },
            TileInfo { solid: true, friction: 1.0 },
        ];
        let tileset = TileSet {
            tile_size: 32,
            sprite: load_texture("./assets/tileset.png").await.context("failed to load tileset")?,
            tile_info,
        };
        let map =
            Map::load_map("./assets/maps/map1".as_ref(), tileset).context("Failed to load map")?;

        let human_spritesheet = load_texture("./assets/creatures/player.png")
            .await
            .context("Failed to load human sprite")?;
        let cat_spritesheet = load_texture("./assets/creatures/cat.png")
            .await
            .context("Failed to load cat sprite")?;
        let player = create_player(Vec2::new(50.0, 50.0), human_spritesheet);
        let cat = create_cat(Vec2::new(400.0, 50.0), cat_spritesheet);

        let lava_texture = load_texture("./assets/blocks/lava_anim.png")
            .await
            .context("Failed to load lava sprite")?;
        let lava = vec![
            create_lava_block(Vec2::new(100.0, 800.0), lava_texture),
            create_lava_block(Vec2::new(100.0 + 32.0, 800.0), lava_texture),
            create_lava_block(Vec2::new(100.0 + 64.0, 800.0), lava_texture),
        ];

        let background = load_texture("./assets/background.png")
            .await
            .context("Failed to load background image")?;
        let font_spritesheet =
            load_texture("./assets/fonts/Victoria.png").await.context("Failed to load font")?;
        let font = BitFont::new(32, 96, 8, 9, font_spritesheet);

        Ok(Self { map, player, cat, lava, font, camera: Vec2::ZERO, background })
    }

    pub fn update(&mut self, secs: f32) {
        let map = &self.map;
        self.player.update(map, secs);
        self.cat.update(map, secs);
        self.lava[0].update(map, secs);
        self.lava[1].update(map, secs);
        self.lava[2].update(map, secs);
    }

    pub fn draw(&mut self) {
        draw_texture(self.background, 0.0, 0.0, WHITE);

        // Center the camera on the player:
        let width = screen_width();
        let height = screen_height();

        self.camera = self.player.object.center() - Vec2::new(width / 2.0, height / 2.0);
        self.camera.clamp(Vec2::ZERO, self.map.size());

        let camera = self.camera.round();
        self.map.draw(camera);
        self.player.draw(camera);
        self.cat.draw(camera);
        self.lava[0].draw(camera);
        self.lava[1].draw(camera);
        self.lava[2].draw(camera);

        static TEST_STRING: &'static str = r#"Test string,
with multiple lines."#;
        self.font.draw_text(Vec2::ZERO, TEST_STRING);
    }
}

fn create_player(position: Vec2, spritesheet: Texture2D) -> Entity<Creature, KeyboardController> {
    let fw = 64;
    let fh = 128;
    let idle = Animation {
        id: NEXT_ANIMATION_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
        sprite: Sprite {
            spritesheet: spritesheet.clone(),
            offset: UVec2::ZERO,
            frame_width: fw,
            frame_height: fh,
            num_frames_x: 1,
            num_frames_y: 1,
        },
        repeat: true,
        frame_time: 0.0,
    };
    let walk = Animation {
        id: NEXT_ANIMATION_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
        sprite: Sprite {
            spritesheet: spritesheet.clone(),
            offset: UVec2::new(1 * fw, 0),
            frame_width: fw,
            frame_height: fh,
            num_frames_x: 6,
            num_frames_y: 1,
        },
        repeat: true,
        frame_time: 0.7,
    };
    let jump = Animation {
        id: NEXT_ANIMATION_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
        sprite: Sprite {
            spritesheet: spritesheet.clone(),
            offset: UVec2::new(7 * fw, 1 * fh),
            frame_width: fw,
            frame_height: fh,
            num_frames_x: 1,
            num_frames_y: 1,
        },
        repeat: true,
        frame_time: 0.6,
    };
    let fall = Animation {
        id: NEXT_ANIMATION_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
        sprite: Sprite {
            spritesheet: spritesheet.clone(),
            offset: UVec2::new(8 * fw, 1 * fh),
            frame_width: fw,
            frame_height: fh,
            num_frames_x: 2,
            num_frames_y: 1,
        },
        repeat: true,
        frame_time: 0.5,
    };

    Entity {
        object: Creature::new(
            position,
            Rect::new(14.0, 36.0, 32.0, 92.0),
            Rect::new(0.0, 0.0, 32.0, 32.0),
            PhysicalProperties {
                c_drag: 0.470,
                mass: 70.00,       // (kg)
                cross_area: 0.760, // (m^2)
                max_vel_x: 9.000,  // (m/s)
                stop_bonus: 6.000,
            },
            6.0,
            5.0,
            CreatureAnimations { idle, walk, jump, fall },
        ),
        controller: KeyboardController::new(),
    }
}

fn create_cat(position: Vec2, spritesheet: Texture2D) -> Entity<Creature, RandomController> {
    let fw = 40;
    let fh = 32;
    let idle = Animation {
        id: NEXT_ANIMATION_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
        sprite: Sprite {
            spritesheet,
            offset: UVec2::ZERO,
            frame_width: fw,
            frame_height: fh,
            num_frames_x: 1,
            num_frames_y: 1,
        },
        repeat: true,
        frame_time: 0.0,
    };
    let walk = Animation {
        id: NEXT_ANIMATION_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
        sprite: Sprite {
            spritesheet,
            offset: UVec2::new(0, fh),
            frame_width: fw,
            frame_height: fh,
            num_frames_x: 6,
            num_frames_y: 1,
        },
        repeat: true,
        frame_time: 0.7,
    };
    let jump = Animation {
        id: NEXT_ANIMATION_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
        sprite: Sprite {
            spritesheet,
            offset: UVec2::ZERO,
            frame_width: fw,
            frame_height: fh,
            num_frames_x: 1,
            num_frames_y: 1,
        },
        repeat: true,
        frame_time: 0.6,
    };
    let fall = Animation {
        id: NEXT_ANIMATION_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
        sprite: Sprite {
            spritesheet,
            offset: UVec2::ZERO,
            frame_width: fw,
            frame_height: fh,
            num_frames_x: 1,
            num_frames_y: 1,
        },
        repeat: true,
        frame_time: 0.5,
    };

    Entity {
        object: Creature::new(
            position,
            Rect::new(2.0, 2.0, 38.0, 30.0),
            Rect::new(0.0, 0.0, 32.0, 32.0),
            PhysicalProperties {
                c_drag: 0.470,
                mass: 70.00,       // (kg)
                cross_area: 0.760, // (m^2)
                max_vel_x: 4.000,  // (m/s)
                stop_bonus: 6.000,
            },
            6.0,
            5.0,
            CreatureAnimations { idle, walk, jump, fall },
        ),
        controller: RandomController::new(0.5),
    }
}

fn create_lava_block(
    pos: Vec2,
    spritesheet: Texture2D,
) -> Entity<DamageBlock, NoneController<DamageBlock>> {
    let lava_animation = Animation {
        id: NEXT_ANIMATION_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
        sprite: Sprite {
            spritesheet,
            offset: UVec2::ZERO,
            frame_width: 32,
            frame_height: 32,
            num_frames_x: 5,
            num_frames_y: 1,
        },
        frame_time: 0.06,
        repeat: true,
    };

    Entity {
        object: DamageBlock::new(Rect::new(pos.x, pos.y, 32.0, 32.0), 1.0, lava_animation),
        controller: NoneController::<DamageBlock>::new(),
    }
}
