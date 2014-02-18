use game::map::Map;

pub mod player;
pub mod ai;

pub trait Controller {
    fn update(&mut self, map: &Map, secs: f32);
}
