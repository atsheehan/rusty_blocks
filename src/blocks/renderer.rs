use sdl;

use blocks::game::Game;
use blocks::grid;

use sdl::video::Surface;

use sdl::video::Color::RGB;

pub fn draw(game: &Game, screen: &Surface) {
    let (width, height) = screen.get_size();
    screen.fill(RGB(0, 255, 0));

    let cell_size = height / (grid::ROWS + 2);

    let grid_width = cell_size * grid::COLUMNS;
    let grid_height = cell_size * grid::ROWS;

    let grid_x = ((width - grid_width) / 2) as i16;
    let grid_y = ((height - grid_height) / 2) as i16;

    let mut bounds = sdl::Rect { x: 0, y: 0, w: cell_size, h: cell_size };

    for row in 0..grid::ROWS {
        for col in 0..grid::COLUMNS {
            bounds.x = grid_x + (col * cell_size) as i16;
            bounds.y = grid_y + (row * cell_size) as i16;

            screen.fill_rect(Some(bounds), RGB(0, 0, 0));
        }
    }
}
