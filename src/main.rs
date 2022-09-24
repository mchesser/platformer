use macroquad::prelude::*;

mod bitfont;
mod controller;
mod entity;
mod game;
mod map;
mod sprite;
mod tiles;

#[macroquad::main("Platformer")]
async fn main() {
    let mut game = game::Game::new().await.unwrap();
    loop {
        let dt = get_frame_time();

        game.update(dt);
        game.draw();

        next_frame().await
    }
}
