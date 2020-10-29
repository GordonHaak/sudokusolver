use super::super::IndexType;
use super::super::SudokuClassic;

pub struct ColumnIterator<'a> {
    data: &'a Vec<Option<u8>>,
    row: IndexType,
    col: IndexType,
}

impl ColumnIterator<'_> {
    pub fn new(sudoku: &SudokuClassic, col: IndexType) -> ColumnIterator {
        ColumnIterator {
            data: &sudoku.fields,
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
            self.data
                .get(SudokuClassic::index((self.row - 1, self.col)))
        }
    }
}
