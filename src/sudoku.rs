#[derive(Debug)]
pub struct Sudoku {
    pub squares: Vec<u8>,
}

impl Sudoku {
    pub fn get_rows(&self) -> Vec<Vec<u8>> {
        let mut rows = Vec::new();
        for row in 0..9 {
            rows.push(Vec::new());
            for col in 0..9 {
                rows[row].push(self.squares[row * 9 + col])
            }
        }

        rows
    }

    pub fn get_cols(&self) -> Vec<Vec<u8>> {
        let mut cols = Vec::new();
        for col in 0..9 {
            cols.push(Vec::new());
            for row in 0..9 {
                cols[col].push(self.squares[row * 9 + col])
            }
        }

        cols
    }

    pub fn get_square(&self) -> Vec<Vec<u8>> {
        let mut squares = Vec::new();
        for square_row in 0..3 {
            for square_col in 0..3 {
                squares.push(Vec::new());
                for row in 0..3 {
                    for col in 0..3 {
                        squares[square_row * 3 + square_col]
                            .push(self.squares[square_row * 27 + square_col * 3 + row * 9 + col])
                    }
                }
            }
        }
        squares
    }

    pub fn back_prop(&self) -> Self {
        let new_squares = self.squares.clone();
        Sudoku {
            squares: new_squares,
        }
    }
}
