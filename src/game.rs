use rand;
use rand::Rng;

use sdl::Rect;
use sdl::video::{Surface, Color};
use sdl::event::Key;

pub struct Game {
    finished: bool,
}

impl Game {
    pub fn new() -> Game {
        Game { finished: false }
    }

    pub fn over(&self) -> bool {
        self.finished
    }

    pub fn exit(&mut self) {
        self.finished = true;
    }

    pub fn draw(&self, screen: &Surface) {
        let mut rng = rand::thread_rng();

        for i in 0usize..10 {
            for j in 0usize..10 {
                screen.fill_rect(Some(Rect {
                    x: (i as i16) * 800 / 10,
                    y: (j as i16) * 600 / 10,
                    w: 800 / 10,
                    h: 600 / 10
                }), rng.gen::<Color>());
            }
        }
    }

    pub fn key_event(&mut self, key: Key) {
        match key {
            Key::Escape => self.exit(),
            _ => {},
        }
    }
}
