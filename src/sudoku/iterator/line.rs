use super::super::Classic;
use super::super::IndexType;

pub struct Iterator<'a> {
    sudoku: &'a Classic,
    row: IndexType,
    col: IndexType,
}

impl Iterator<'_> {
    #[must_use]
    pub const fn new(sudoku: &Classic, row: IndexType) -> Iterator {
        Iterator {
            sudoku,
            row,
            col: 0,
        }
    }
}

impl<'t> std::iter::Iterator for Iterator<'t> {
    type Item = &'t Option<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.col >= Classic::COLS {
            None
        } else {
            let c = self.col;
            self.col += 1;
            Some(&self.sudoku[(self.row, c)])
        }
    }
}
