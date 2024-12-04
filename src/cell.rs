pub trait Cell {
    fn get_row(&self) -> usize;
    fn get_col(&self) -> usize;
    fn get_square(&self) -> usize;
}

impl Cell for usize {
    fn get_row(&self) -> usize {
        self / 9
    }

    fn get_col(&self) -> usize {
        self % 9
    }

    fn get_square(&self) -> usize {
        self.get_row() / 3 * 3 + self.get_col() / 3
    }
}

pub struct CellPosition {
    pub row: usize,
    pub col: usize,
    pub size: usize,
}

impl Cell for CellPosition {
    fn get_row(&self) -> usize {
        self.row
    }

    fn get_col(&self) -> usize {
        self.col
    }

    fn get_square(&self) -> usize {
        self.row / 3 * 3 + self.col / 3
    }
}

impl std::fmt::Display for CellPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.row, self.col)
    }
}

impl CellPosition {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col, size: 9 }
    }

    pub fn get_position(&self) -> usize {
        self.row * self.size + self.col
    }
}
