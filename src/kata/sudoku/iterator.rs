use super::SudokuClassic;

trait SudokuInterator<'t> {
    fn is_end(&self) -> bool;
    fn inc(&mut self) -> Option<&'t Option<u8>>;
}

pub struct LineIterator<'a> {
    data: &'a Vec<Option<u8>>,
    row: u8,
    col: u8,
}

impl<'t> LineIterator<'t> {
    pub fn new(sudoku: &'t SudokuClassic, row: u8) -> LineIterator {
        LineIterator {
            data: &sudoku.fields,
            row,
            col: 0,
        }
    }
}

impl<'n> SudokuInterator<'n> for LineIterator<'n> {
    fn is_end(&self) -> bool {
        self.col >= SudokuClassic::COLS - 1
    }

    fn inc(&mut self) -> Option<&'n Option<u8>> {
        self.col += 1;
        self.data.get(SudokuClassic::index(self.row, self.col - 1))
    }
}

impl<'t> Iterator for LineIterator<'t> {
    type Item = &'t Option<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_end() {
            None
        } else {
            self.inc()
        }
    }
}
