use super::super::IndexType;
use super::super::SudokuClassic;

pub struct LineIterator<'a> {
    sudoku: &'a SudokuClassic,
    row: IndexType,
    col: IndexType,
}

impl LineIterator<'_> {
    pub fn new(sudoku: &SudokuClassic, row: IndexType) -> LineIterator {
        LineIterator {
            sudoku: &sudoku,
            row,
            col: 0,
        }
    }
}

impl<'t> Iterator for LineIterator<'t> {
    type Item = &'t Option<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.col >= SudokuClassic::COLS {
            None
        } else {
            let c = self.col;
            self.col += 1;
            Some(&self.sudoku[(self.row, c)])
        }
    }
}
