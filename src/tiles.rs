use sdl2::render::Texture;
use sdl2::render::Renderer;
use sdl2::rect::Rect;

pub struct TileInfo {
    pub solid: bool,
    pub friction: f32,
}

pub struct TileSet {
    pub tile_size: i32,
    pub sprite: Texture,
    pub tile_info: Vec<TileInfo>,
}

impl TileSet {
    pub fn id(&self, id_num: u16) -> TileInfo {
        *self.tile_info.get(id_num as uint)
    }

    pub fn draw(&self, id_num: u16, dest_rect: Rect, renderer: &Renderer) {
        let source_rect = Rect::new(id_num as i32 * self.tile_size, 0,
                                    self.tile_size, self.tile_size);
        renderer.copy(&self.sprite, Some(source_rect), Some(dest_rect));
    }
}
