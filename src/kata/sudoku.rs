use std::fmt;

pub mod index;
pub mod iterator;

#[derive(Debug)]
pub struct SudokuClassic {
    fields: Vec<Option<u8>>,
}

impl SudokuClassic {
    const ROWS: u8 = 9;
    const COLS: u8 = 9;

    pub fn new() -> SudokuClassic {
        SudokuClassic {
            fields: vec![None; (SudokuClassic::ROWS * SudokuClassic::COLS).into()],
        }
    }

    pub fn row(&self, row: u8) -> iterator::LineIterator {
        iterator::LineIterator::new(self, row)
    }

    fn index(row: u8, col: u8) -> usize {
        if row >= SudokuClassic::ROWS || col >= SudokuClassic::COLS {
            panic!("invalid index for Sudoku row {}, col {}", row, col);
        }
        (row * SudokuClassic::ROWS + col).into()
    }
}

impl Default for SudokuClassic {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for SudokuClassic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for r in 0..SudokuClassic::ROWS - 1 {
            for i in self.row(r) {
                write!(f, "{:?}", i)?;
            }
            writeln!(f, "");
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn sudoku() {
        let sudoku = super::SudokuClassic::new();
        assert_eq!(sudoku[(0, 0)], None);
    }

    #[test]
    #[should_panic(expected = "invalid index for Sudoku row 9, col 0")]
    fn invalid_sudoku_access() {
        let sudoku = super::SudokuClassic::new();
        assert_eq!(sudoku[(9, 0)], None);
    }
}
