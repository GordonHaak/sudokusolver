use super::super::Classic;
use super::super::IndexType;

pub struct Iterator<'a> {
    sudoku: &'a Classic,
    row: IndexType,
    col: IndexType,
}

impl Iterator<'_> {
    #[must_use]
    pub const fn new(sudoku: &Classic, col: IndexType) -> Iterator {
        Iterator {
            sudoku,
            row: 0,
            col,
        }
    }
}

impl<'t> std::iter::Iterator for Iterator<'t> {
    type Item = &'t Option<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.row >= Classic::ROWS {
            None
        } else {
            self.row += 1;
            Some(&self.sudoku[(self.row - 1, self.col)])
        }
    }
}
