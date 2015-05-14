use sdl;

use sdl::Rect;
use sdl::video::{Surface, Color};
use sdl::event::{Key, Event};

use blocks::grid::Grid;
use blocks::renderer;

pub struct Game {
    finished: bool,
    pub grid: Grid,
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
        renderer::draw(self, screen);
        screen.flip();
    }

    pub fn handle_events(&mut self) {
        loop {
            match sdl::event::poll_event() {
                Event::Quit => self.exit(),
                Event::None => break,
                Event::Key(k, keydown, _, _) => self.key_event(k, keydown),
                _ => {}
            }
        }
    }

    fn exit(&mut self) {
        self.finished = true;
    }

    fn key_event(&mut self, key: Key, keydown: bool) {
        match key {
            Key::Escape => self.exit(),
            Key::Right if keydown => self.grid.move_right(),
            Key::Left if keydown => self.grid.move_left(),
            Key::Down if keydown => self.grid.move_down(),
            _ => {},
        }
    }
}
