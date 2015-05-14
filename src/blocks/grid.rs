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
        self.current_shape = self.current_shape.take().map(|shape| {
            if !self.is_colliding(shape.move_right()) {
                shape.move_right()
            } else {
                shape
            }
        });
    }

    pub fn move_left(&mut self) {
        self.current_shape = self.current_shape.take().map(|shape| {
            if !self.is_colliding(shape.move_left()) {
                shape.move_left()
            } else {
                shape
            }
        });
    }

    pub fn move_down(&mut self) {
        self.current_shape = self.current_shape.take().map(|shape| {
            if !self.is_colliding(shape.move_down()) {
                shape.move_down()
            } else {
                shape
            }
        });
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

    fn is_colliding(&self, shape: Shape) -> bool {
        shape.blocks().iter().any(|&(row, col)| {
            out_of_bounds(row, col) || self.cell_value(row, col) > 0
        })
    }

    fn cell_value(&self, row: i16, col: i16) -> i16 {
        let i = ((row * COLUMNS) + col) as usize;
        self.cells[i]
    }
}

fn out_of_bounds(row: i16, col: i16) -> bool {
    row >= ROWS || row < 0 || col >= COLUMNS || col < 0
}
