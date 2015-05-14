extern crate sdl;

mod blocks;

use sdl::video::{Surface, SurfaceFlag, VideoFlag};
use blocks::game::Game;

const SCREEN_WIDTH: isize = 1280;
const SCREEN_HEIGHT: isize = 720;

fn initialize_sdl(caption: &str) -> Surface {
    sdl::init(&[sdl::InitFlag::Video]);
    sdl::wm::set_caption(caption, caption);

    match sdl::video::set_video_mode(SCREEN_WIDTH, SCREEN_HEIGHT, 32,
                                     &[SurfaceFlag::HWSurface],
                                     &[VideoFlag::DoubleBuf]) {
        Ok(screen) => screen,
        Err(err) => panic!("failed to set video mode: {}", err)
    }
}

fn main() {
    let screen = initialize_sdl("rusty blocks");
    let mut game = Game::new();

    while !game.over() {
        game.update();
        game.handle_events();
        game.draw(&screen);
    }

    sdl::quit();
}
