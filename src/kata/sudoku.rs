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

    pub fn row<'a>(&'a self, row: u8) -> Box<dyn Iterator<Item = &'a Option<u8>> + 'a> {
        Box::new(iterator::LineIterator::new(self, row))
    }

    pub fn col<'a>(&'a self, col: u8) -> Box<dyn Iterator<Item = &'a Option<u8>> + 'a> {
        Box::new(iterator::ColumnIterator::new(self, col))
    }

    pub fn field<'a>(&'a self, row: u8, col: u8) -> Box<dyn Iterator<Item = &'a Option<u8>> + 'a> {
        Box::new(iterator::FieldIterator::new(self, row, col))
    }

    pub fn next_free_field(&mut self) -> Option<(u8, u8)> {
        None
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
            let row = self.row(r);
            writeln!(f, "{:?}", row.collect::<Vec<_>>())?;
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

    #[test]
    fn field_iter() {
        let mut sudoku = super::SudokuClassic::new();
        sudoku[(3, 3)] = Some(1);
        sudoku[(3, 4)] = Some(2);
        sudoku[(3, 5)] = Some(3);
        sudoku[(4, 3)] = Some(4);
        sudoku[(4, 4)] = Some(5);
        sudoku[(4, 5)] = Some(6);
        sudoku[(5, 3)] = Some(7);
        sudoku[(5, 4)] = Some(8);
        sudoku[(5, 5)] = Some(9);
        assert_eq!(
            sudoku.field(4, 5).map(|v| v.clone()).collect::<Vec<_>>(),
            vec![
                Some(1),
                Some(2),
                Some(3),
                Some(4),
                Some(5),
                Some(6),
                Some(7),
                Some(8),
                Some(9)
            ]
        );
        println!("Sudoku is : \n{}", sudoku);
    }
}
