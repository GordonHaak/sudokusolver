use crate::kata::sudoku::SudokuClassic;

pub struct FieldIterator<'a> {
    data: &'a Vec<Option<u8>>,
    row: u8,
    col: u8,
    pos: u8,
}

impl FieldIterator<'_> {
    pub fn new(sudoku: &SudokuClassic, row: u8, col: u8) -> FieldIterator {
        FieldIterator {
            data: &sudoku.fields,
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
            self.data
                .get(SudokuClassic::index(self.row + (p / 3), self.col + (p % 3)))
        }
    }
}
