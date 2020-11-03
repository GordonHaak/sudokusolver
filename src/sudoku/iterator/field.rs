use super::super::Classic;
use super::super::IndexType;

pub struct Iterator<'a> {
    sudoku: &'a Classic,
    row: IndexType,
    col: IndexType,
    pos: IndexType,
}

impl Iterator<'_> {
    #[must_use]
    pub const fn new(sudoku: &Classic, row: IndexType, col: IndexType) -> Iterator {
        Iterator {
            sudoku,
            row: row / 3 * 3,
            col: col / 3 * 3,
            pos: 0,
        }
    }
}

impl<'t> std::iter::Iterator for Iterator<'t> {
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
