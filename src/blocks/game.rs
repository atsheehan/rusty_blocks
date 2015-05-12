use sdl;

use sdl::Rect;
use sdl::video::{Surface, Color};
use sdl::event::{Key, Event};

use blocks::grid::Grid;

pub struct Game {
    finished: bool,
    grid: Grid,
}

impl Game {
    pub fn new() -> Game {
        Game {
            grid: Grid::new(),
            finished: false,
        }
    }

    pub fn update(&self) {}

    pub fn over(&self) -> bool {
        self.finished
    }


    pub fn draw(&self, screen: &Surface) {
        screen.clear();
        screen.flip();
    }

    pub fn handle_events(&mut self) {
        loop {
            match sdl::event::poll_event() {
                Event::Quit => self.exit(),
                Event::None => break,
                Event::Key(k, _, _, _) => self.key_event(k),
                _ => {}
            }
        }
    }

    fn exit(&mut self) {
        self.finished = true;
    }

    fn key_event(&mut self, key: Key) {


        match key {
            Key::Escape => self.exit(),
            _ => {},
        }
    }
}
