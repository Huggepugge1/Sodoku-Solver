#[derive(Debug)]
pub struct Sudoku {
    pub squares: Vec<u8>,
}

impl std::fmt::Display for Sudoku {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut grid = String::new();
        for row in 0..9 {
            for col in 0..9 {
                grid += &format!("{} ", self.squares[row * 9 + col]);
            }
            grid += "\n";
        }
        write!(f, "{}", grid)
    }
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

    fn get_square_position(position: usize) -> usize {
        let square_row = position / 9 / 3;
        let square_col = position % 9 / 3;
        return square_row * 3 + square_col;
    }

    fn back_prop_helper(&mut self, position: usize) -> Result<(), ()> {
        if position >= 81 || self.squares[position] != 0 {
            return Ok(());
        }
        for number in 1..=9 {
            if self.get_rows()[position / 9]
                .iter()
                .find(|x| **x == number)
                .is_none()
                && self.get_cols()[position % 9]
                    .iter()
                    .find(|x| **x == number)
                    .is_none()
                && self.get_square()[Self::get_square_position(position)]
                    .iter()
                    .find(|x| **x == number)
                    .is_none()
            {
                self.squares[position] = number;
                if self.back_prop_helper(position + 1).is_ok() {
                    return Ok(());
                }
            }
        }
        self.squares[position] = 0;
        Err(())
    }

    pub fn back_prop(&self) -> Self {
        let mut new_sudoku = Sudoku {
            squares: self.squares.clone(),
        };
        match new_sudoku.back_prop_helper(0) {
            Ok(()) => (),
            Err(()) => eprintln!("Sudoku is not solvable!"),
        }
        new_sudoku
    }
}
