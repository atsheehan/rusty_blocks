const ROWS: i16 = 4;
const COLUMNS: i16 = 4;
const CELL_COUNT: usize = (ROWS * COLUMNS) as usize;
const ROTATIONS: usize = 4;

const SHAPES: [[i16; CELL_COUNT]; ROTATIONS] = [
    [0, 0, 0, 0,
     0, 1, 1, 0,
     0, 1, 1, 0,
     0, 0, 0, 0],
    [0, 0, 0, 0,
     0, 1, 1, 0,
     0, 1, 1, 0,
     0, 0, 0, 0],
    [0, 0, 0, 0,
     0, 1, 1, 0,
     0, 1, 1, 0,
     0, 0, 0, 0],
    [0, 0, 0, 0,
     0, 1, 1, 0,
     0, 1, 1, 0,
     0, 0, 0, 0],
    ];

pub struct Shape {
    pub x: i16,
    pub y: i16,
    rotation: usize,
}

impl Shape {
    pub fn random() -> Shape {
        Shape { x: 0, y: 0, rotation: 0 }
    }

    pub fn cell(&self, row: i16, col: i16) -> Option<i16> {
        if row >= self.y && row < self.y + ROWS && col >= self.x && col < self.x + COLUMNS {
            let i = (((row - self.y) * COLUMNS) + (col - self.x)) as usize;
            Some(SHAPES[self.rotation][i])
        } else {
            None
        }
    }

    pub fn blocks(&self) -> Vec<(i16, i16)> {
        let shape = SHAPES[self.rotation];
        let mut occupied_cells: Vec<(i16, i16)> = Vec::with_capacity(4);

        for row in 0..ROWS {
            for col in 0..COLUMNS {
                let i: usize = ((row * COLUMNS) + col) as usize;
                if shape[i] > 0 {
                    occupied_cells.push((row + self.y, col + self.x));
                }
            }
        }

        occupied_cells
    }

    pub fn move_left(&self) -> Shape {
        Shape { x: self.x - 1, y: self.y, rotation: self.rotation }
    }

    pub fn move_right(&self) -> Shape {
        Shape { x: self.x + 1, y: self.y, rotation: self.rotation }
    }

    pub fn move_down(&self) -> Shape {
        Shape { x: self.x, y: self.y + 1, rotation: self.rotation }
    }
}
