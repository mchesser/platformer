use macroquad::prelude::{UVec2, Vec2};
use macroquad::texture::Texture2D;

use crate::sprite::Sprite;

/// A bitfont that can be used for rendering text
pub struct BitFont {
    ascii_offset: u8,
    sprite: Sprite,
}

impl BitFont {
    /// Create a new bitfont
    /// # Arguments
    /// `ascii_offset` - The offset of the ascii code to the bitfont
    /// `num_chars` - The number of characters in the texture
    /// `char_width` - The width of a single character in pixels
    /// `char_height` - The height of a single character in pixels
    /// `texture` - The bitfont texture
    /// # Return
    /// A bit font using the texture specified
    pub fn new(
        ascii_offset: u8,
        num_chars: u32,
        char_width: u32,
        char_height: u32,
        texture: Texture2D,
    ) -> BitFont {
        BitFont {
            ascii_offset,
            sprite: Sprite {
                spritesheet: texture,
                offset: UVec2::ZERO,
                frame_width: char_width,
                frame_height: char_height,
                num_frames_x: num_chars,
                num_frames_y: 1,
            },
        }
    }

    /// Draw text on the renderer
    /// # Arguments
    /// `position` - The position to draw the text at
    /// `text` - The text to draw
    /// `renderer` - The renderer
    pub fn draw_text(&self, position: Vec2, text: &str) {
        let mut pos = position;
        for char in text.bytes() {
            if char == b'\n' {
                pos.y += self.sprite.frame_height as f32;
                pos.x = position.x;
            }
            else if char == b'\r' {
                // Discard
            }
            else {
                let char = char - self.ascii_offset;
                self.sprite.draw([char as u32, 0].into(), pos, false);
                pos.x += self.sprite.frame_width as f32;
            }
        }
    }
}
