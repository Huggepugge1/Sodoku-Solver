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
                rows[row].push(self.squares[col])
            }
        }

        rows
    }
}
