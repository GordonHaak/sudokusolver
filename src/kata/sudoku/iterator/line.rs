use crate::kata::sudoku::SudokuClassic;

pub struct LineIterator<'a> {
    data: &'a Vec<Option<u8>>,
    row: u8,
    col: u8,
}

impl LineIterator<'_> {
    pub fn new(sudoku: &SudokuClassic, row: u8) -> LineIterator {
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
        if self.col >= SudokuClassic::COLS - 1 {
            None
        } else {
            self.col += 1;
            self.data.get(SudokuClassic::index(self.row, self.col - 1))
        }
    }
}
