use crate::cell::{Cell, CellPosition};

#[derive(Debug, Clone)]
struct BackProp {
    position: usize,
    numbers: Vec<u8>,
}

impl BackProp {
    fn new() -> Self {
        Self {
            position: 0,
            numbers: Vec::new(),
        }
    }

    fn start(&mut self) {
        self.numbers.push(0);
    }

    fn next(&mut self) -> Result<(u8, usize), &str> {
        if self.numbers.len() == 0 {
            return Err("Back propogation has not been started yet");
        }
        if self.numbers[0] == 0 {
            self.numbers[0] += 1;
        } else {
            self.position += 1;
            self.numbers.push(1);
        }
        Ok((self.numbers[self.position], self.position))
    }

    fn fail(&mut self) -> Result<(u8, usize, usize), &str> {
        if self.numbers.len() == 0 {
            return Err("Back propogation has not been started yet!");
        }
        if self.position == 0 {
            return Err("Back propogation failed!");
        }
        let mut amount = 0;
        if self.numbers[self.position] < 9 {
            self.numbers[self.position] += 1;
        } else {
            self.position -= 1;
            self.numbers.pop();
            self.numbers[self.position] += 1;
            amount += 1;
            while self.numbers[self.position] > 9 {
                self.position -= 1;
                self.numbers.pop();
                self.numbers[self.position] += 1;
                amount += 1;
            }
        }
        Ok((self.numbers[self.position], self.position, amount))
    }
}

#[derive(Debug, Clone)]
pub struct Sudoku {
    pub cells: Vec<u8>,
    back_prop: BackProp,
    pub visualizer: bool,
}

impl std::fmt::Display for Sudoku {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut grid = String::new();
        for row in 0..9 {
            for col in 0..9 {
                grid += &format!("{} ", self.cells[row * 9 + col]);
            }
            grid += "\n";
        }
        write!(f, "{}", grid)
    }
}

impl Sudoku {
    pub fn new(cells: Vec<u8>) -> Self {
        Self {
            cells,
            back_prop: BackProp::new(),
            visualizer: false,
        }
    }

    pub fn get_rows(&self) -> Vec<Vec<u8>> {
        let mut rows = Vec::new();
        for _ in 0..9 {
            rows.push(Vec::new());
        }
        for cell in 0..81 {
            rows[cell.get_row()].push(self.cells[cell]);
        }

        rows
    }

    pub fn get_cols(&self) -> Vec<Vec<u8>> {
        let mut cols = Vec::new();
        for _ in 0..9 {
            cols.push(Vec::new());
        }
        for cell in 0..81 {
            cols[cell.get_col()].push(self.cells[cell]);
        }

        cols
    }

    pub fn get_squares(&self) -> Vec<Vec<u8>> {
        let mut squares = Vec::new();
        for _ in 0..9 {
            squares.push(Vec::new());
        }
        for cell in 0..81 {
            squares[cell.get_square()].push(self.cells[cell]);
        }
        squares
    }

    pub fn get_position(square: CellPosition, inner_coords: CellPosition) -> usize {
        square.get_position() * 3 + inner_coords.get_position()
    }

    pub fn check(&self) -> bool {
        for cell in 0..81 {
            if self.cells[cell] != 0
                && (self.get_rows()[cell.get_row()]
                    .iter()
                    .filter(|x| **x == self.cells[cell])
                    .count()
                    > 1
                    || self.get_cols()[cell.get_col()]
                        .iter()
                        .filter(|x| **x == self.cells[cell])
                        .count()
                        > 1
                    || self.get_squares()[cell.get_square()]
                        .iter()
                        .filter(|x| **x == self.cells[cell])
                        .count()
                        > 1)
            {
                return false;
            }
        }
        true
    }

    fn back_prop_helper(&mut self, position: usize) -> Result<(), ()> {
        if position >= 81 {
            return Ok(());
        }
        if self.cells[position] != 0 {
            return self.back_prop_helper(position + 1);
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
                self.cells[position] = number;
                if self.back_prop_helper(position + 1).is_ok() {
                    return Ok(());
                }
                self.cells[position] = 0;
            }
        }
        self.cells[position] = 0;
        Err(())
    }

    pub fn solve(&self) -> Result<Self, &str> {
        if !self.check() {
            return Err("Sudoku is not solvable!");
        }
        let mut new_sudoku = self.clone();
        match new_sudoku.back_prop_helper(0) {
            Ok(()) => Ok(new_sudoku),
            Err(()) => Err("Sudoku is not solvable!"),
        }
    }

    pub fn back_prop_next_step(&mut self) -> Result<(), &str> {
        if self.back_prop.numbers.len() == 0 {
            self.back_prop.start();
        }
        if self.check() {
            if self.back_prop.numbers.len() == 81 {
                self.visualizer = false;
            } else {
                match self.back_prop.next() {
                    Ok((number, position)) => {
                        self.cells[position] = number;
                    }
                    Err(e) => return Err(e),
                }
            }
        } else {
            match self.back_prop.fail() {
                Ok((number, position, amount)) => {
                    self.cells[position] = number;
                    for pos in (position + 1)..(position + 1 + amount) {
                        self.cells[pos] = 0;
                    }
                }
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }
}
