use crate::cell::{Cell, CellPosition};

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
        for _ in 0..9 {
            rows.push(Vec::new());
        }
        for cell in 0..81 {
            rows[cell.get_row()].push(self.squares[cell]);
        }

        rows
    }

    pub fn get_cols(&self) -> Vec<Vec<u8>> {
        let mut cols = Vec::new();
        for _ in 0..9 {
            cols.push(Vec::new());
        }
        for cell in 0..81 {
            cols[cell.get_col()].push(self.squares[cell]);
        }

        cols
    }

    pub fn get_squares(&self) -> Vec<Vec<u8>> {
        let mut squares = Vec::new();
        for _ in 0..9 {
            squares.push(Vec::new());
        }
        for cell in 0..81 {
            squares[cell.get_square()].push(self.squares[cell]);
        }
        squares
    }

    pub fn get_position(square: CellPosition, inner_coords: CellPosition) -> usize {
        square.get_position() * 3 + inner_coords.get_position()
    }

    fn check(&self) -> bool {
        for cell in 0..81 {
            if self.squares[cell] != 0
                && (self.get_rows()[cell.get_row()]
                    .iter()
                    .filter(|x| **x == self.squares[cell])
                    .count()
                    > 1
                    || self.get_cols()[cell.get_col()]
                        .iter()
                        .filter(|x| **x == self.squares[cell])
                        .count()
                        > 1
                    || self.get_squares()[cell.get_square()]
                        .iter()
                        .filter(|x| **x == self.squares[cell])
                        .count()
                        > 1)
            {
                return false;
            }
        }
        true
    }

    fn back_prop_helper(&mut self, position: usize) -> Result<(), ()> {
        if position >= 81 || self.squares[position] != 0 {
            return Ok(());
        }
        for number in 1..=9 {
            if self.get_rows()[position.get_row()]
                .iter()
                .find(|x| **x == number)
                .is_none()
                && self.get_cols()[position.get_col()]
                    .iter()
                    .find(|x| **x == number)
                    .is_none()
                && self.get_squares()[position.get_square()]
                    .iter()
                    .find(|x| **x == number)
                    .is_none()
            {
                self.squares[position] = number;
                if self.back_prop_helper(position + 1).is_ok() {
                    return Ok(());
                }
                self.squares[position] = 0;
            }
        }
        self.squares[position] = 0;
        Err(())
    }

    pub fn solve(&self) -> Result<Self, &str> {
        if !self.check() {
            return Err("Sudoku is not solvable!");
        }
        let mut new_sudoku = Sudoku {
            squares: self.squares.clone(),
        };
        match new_sudoku.back_prop_helper(0) {
            Ok(()) => Ok(new_sudoku),
            Err(()) => Err("Sudoku is not solvable!"),
        }
    }
}
