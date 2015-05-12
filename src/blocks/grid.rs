const ROWS: usize = 20;
const COLUMNS: usize = 10;
const CELL_COUNT: usize = ROWS * COLUMNS;

pub struct Grid {
    cells: [u16; CELL_COUNT],
}

impl Grid {
    pub fn new() -> Grid {
        Grid { cells: [0; CELL_COUNT] }
    }
}
