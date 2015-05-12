pub const ROWS: u16 = 20;
pub const COLUMNS: u16 = 10;
pub const CELL_COUNT: u16 = ROWS * COLUMNS;

pub struct Grid {
    cells: [u16; CELL_COUNT as usize],
}

impl Grid {
    pub fn new() -> Grid {
        Grid { cells: [0; CELL_COUNT as usize] }
    }
}
