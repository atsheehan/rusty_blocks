extern crate sdl;
extern crate rand;

use rand::Rng;

use sdl::video::{SurfaceFlag, VideoFlag};
use sdl::event::{Event, Key};

fn main() {
    sdl::init(&[sdl::InitFlag::Video]);
    sdl::wm::set_caption("rust-sdl demo - video", "rust-sdl");

    let mut rng = rand::thread_rng();
    let screen = match sdl::video::set_video_mode(800, 600, 32,
                                                  &[SurfaceFlag::HWSurface],
                                                  &[VideoFlag::DoubleBuf]) {
        Ok(screen) => screen,
        Err(err) => panic!("failed to set video mode: {}", err)
    };

    'main : loop {
        for i in 0usize..10 {
            for j in 0usize..10 {
                screen.fill_rect(Some(sdl::Rect {
                    x: (i as i16) * 800 / 10,
                    y: (j as i16) * 600 / 10,
                    w: 800 / 10,
                    h: 600 / 10
                }), rng.gen::<sdl::video::Color>());
            }
        }

        screen.flip();

        'event : loop {
            match sdl::event::poll_event() {
                Event::Quit => break 'main,
                Event::None => break 'event,
                Event::Key(k, _, _, _)
                    if k == Key::Escape
                    => break 'main,
                _ => {}
            }
        }
    }

    sdl::quit();
}
