use super::super::IndexType;
use super::super::SudokuClassic;

pub struct FieldIterator<'a> {
    sudoku: &'a SudokuClassic,
    row: IndexType,
    col: IndexType,
    pos: IndexType,
}

impl FieldIterator<'_> {
    pub fn new(sudoku: &SudokuClassic, row: IndexType, col: IndexType) -> FieldIterator {
        FieldIterator {
            sudoku: &sudoku,
            row: row / 3 * 3,
            col: col / 3 * 3,
            pos: 0,
        }
    }
}

impl<'t> Iterator for FieldIterator<'t> {
    type Item = &'t Option<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= 9 {
            None
        } else {
            let p = self.pos;
            self.pos += 1;
            Some(&self.sudoku[(self.row + (p / 3), self.col + (p % 3))])
        }
    }
}
