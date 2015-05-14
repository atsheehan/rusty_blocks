use blocks::shape::Shape;

pub const ROWS: i16 = 20;
pub const COLUMNS: i16 = 10;
pub const CELL_COUNT: i16 = ROWS * COLUMNS;

pub struct Grid {
    cells: [i16; CELL_COUNT as usize],
    current_shape: Option<Shape>,
}

impl Grid {
    pub fn new() -> Grid {
        Grid {
            cells: [0; CELL_COUNT as usize],
            current_shape: Some(Shape::random()),
        }
    }

    pub fn move_right(&mut self) {
        match self.current_shape {
            Some(ref mut shape) => shape.x += 1,
            None => {}
        };
    }

    pub fn move_left(&mut self) {
        match self.current_shape {
            Some(ref mut shape) => shape.x -= 1,
            None => {}
        };
    }

    pub fn move_down(&mut self) {
        match self.current_shape {
            Some(ref mut shape) => shape.y += 1,
            None => {}
        };
    }

    pub fn cell(&self, row: i16, col: i16) -> i16 {
        match self.current_shape {
            Some(ref shape) => match shape.cell(row, col) {
                Some(value) => value,
                None => self.cell_value(row, col),
            },
            None => self.cell_value(row, col)
        }
    }

    fn cell_value(&self, row: i16, col: i16) -> i16 {
        let i = ((row * COLUMNS) + col) as usize;
        self.cells[i]
    }
}
