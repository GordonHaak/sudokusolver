use super::super::IndexType;
use super::super::SudokuClassic;

pub struct LineIterator<'a> {
    data: &'a Vec<Option<u8>>,
    row: IndexType,
    col: IndexType,
}

impl LineIterator<'_> {
    pub fn new(sudoku: &SudokuClassic, row: IndexType) -> LineIterator {
        LineIterator {
            data: &sudoku.fields,
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
            self.data.get(SudokuClassic::index((self.row, c)))
        }
    }
}
