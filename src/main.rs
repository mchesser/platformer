extern mod sdl2;
extern mod sdl2_image;
extern mod native;

use std::rc::Rc;
use std::cell::RefCell;

use sdl2::{event, video, render};
use timer::Timer;
use keyboard::KeyboardState;

mod timer;
mod game;
mod gmath;
mod keyboard;

static WIN_WIDTH: int = 800;
static WIN_HEIGHT: int = 600;

#[start]
#[cfg(not(test))]
fn start(argc: int, argv: **u8) -> int {
    native::start(argc, argv, main)
}

#[main]
fn main() {
    static WHITE: sdl2::pixels::Color = sdl2::pixels::RGB(0xFF, 0xFF, 0xFF);

    // Initialise SDL
    sdl2::init([sdl2::InitVideo]);

    // Initialise the window
    let window =
        match video::Window::new("Platformer Game", video::PosCentered, video::PosCentered,
                WIN_WIDTH, WIN_HEIGHT, [video::OpenGL]) {
            Ok(window) => window,
            Err(err) => fail!(format!("failed to create window: {}", err))
    };

    // Initialise the renderer
    let renderer =
        match render::Renderer::from_window(window, render::DriverAuto, [render::Accelerated]) {
            Ok(renderer) => renderer,
            Err(err) => fail!(format!("failed to create renderer: {}", err))
    };

    // Initialise the game
    let keyboard = Rc::new(RefCell::new(KeyboardState::new()));
    let mut game = game::Game::new(keyboard.clone(), renderer);

    // Initialise timer
    let mut timer = Timer::new();
    let mut log_timer = Timer::new();

    'main: loop {
        let secs = timer.elapsed_seconds();
        timer.reset();
        keyboard.borrow().with_mut(|k| k.update());

        'event: loop {
            match event::poll_event() {
                event::QuitEvent(_) => break 'main,

                /* event::KeyDownEvent(_, _, code, _, _) => {} */
                /* event::KeyUpEvent(_, _, code, _, _) => {} */

                event::NoEvent => break,
                _ => {}
            }
        }

        game.update(secs);

        // Clear the screen
        renderer.set_draw_color(WHITE);
        renderer.clear();

        game.draw(renderer);

        // Refresh the screen
        renderer.present();

        // Print some information about the game
        if log_timer.elapsed_seconds() > 1.0 {
            log_timer.reset();
            println!("PLAYER: velocity {}, position {}, on_ground: {:?}",
                game.player.entity.vel.to_str(),
                game.player.entity.pos.to_str(),
                game.player.entity.on_ground
            );
            println!("");
        }
    }
}
