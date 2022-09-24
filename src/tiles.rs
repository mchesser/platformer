use macroquad::{
    prelude::{Rect, WHITE},
    texture::{draw_texture_ex, DrawTextureParams, Texture2D},
};

#[derive(Copy, Clone)]
pub struct TileInfo {
    pub solid: bool,
    pub friction: f32,
}

pub struct TileSet {
    pub tile_size: i32,
    pub sprite: Texture2D,
    pub tile_info: Vec<TileInfo>,
}

impl TileSet {
    pub fn id(&self, id_num: u16) -> TileInfo {
        self.tile_info[id_num as usize]
    }

    pub fn draw(&self, id_num: u16, dest_rect: Rect) {
        let source_rect = Rect::new(
            (id_num as i32 * self.tile_size) as f32,
            0.0,
            self.tile_size as f32,
            self.tile_size as f32,
        );
        draw_texture_ex(self.sprite, dest_rect.x, dest_rect.y, WHITE, DrawTextureParams {
            dest_size: Some(dest_rect.size()),
            source: Some(source_rect),
            ..Default::default()
        });
    }
}
