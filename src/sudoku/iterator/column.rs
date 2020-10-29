use super::super::IndexType;
use super::super::SudokuClassic;

pub struct ColumnIterator<'a> {
    sudoku: &'a SudokuClassic,
    row: IndexType,
    col: IndexType,
}

impl ColumnIterator<'_> {
    pub fn new(sudoku: &SudokuClassic, col: IndexType) -> ColumnIterator {
        ColumnIterator {
            sudoku: &sudoku,
            row: 0,
            col,
        }
    }
}

impl<'t> Iterator for ColumnIterator<'t> {
    type Item = &'t Option<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.row >= SudokuClassic::ROWS {
            None
        } else {
            self.row += 1;
            Some(&self.sudoku[(self.row - 1, self.col)])
        }
    }
}
