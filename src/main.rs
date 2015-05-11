extern crate sdl;
extern crate rand;

use sdl::video::{SurfaceFlag, VideoFlag};
use sdl::event::Event;

mod game;
use game::Game;

fn main() {
    sdl::init(&[sdl::InitFlag::Video]);
    sdl::wm::set_caption("rust-sdl demo - video", "rust-sdl");

    let screen = match sdl::video::set_video_mode(800, 600, 32,
                                                  &[SurfaceFlag::HWSurface],
                                                  &[VideoFlag::DoubleBuf]) {
        Ok(screen) => screen,
        Err(err) => panic!("failed to set video mode: {}", err)
    };

    let mut game = Game::new();

    while !game.over() {
        screen.clear();
        game.draw(&screen);
        screen.flip();

        loop {
            match sdl::event::poll_event() {
                Event::Quit => game.exit(),
                Event::None => break,
                Event::Key(k, _, _, _) => game.key_event(k),
                _ => {}
            }
        }
    }

    sdl::quit();
}
