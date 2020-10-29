use super::IndexTuple;
use super::SudokuClassic;
use std::ops::{Index, IndexMut};

fn to_pos((row, col): IndexTuple) -> usize {
    if row >= SudokuClassic::ROWS || col >= SudokuClassic::COLS {
        panic!("invalid index for Sudoku row {}, col {}", row, col);
    }
    row * SudokuClassic::ROWS + col
}

impl Index<IndexTuple> for SudokuClassic {
    type Output = Option<u8>;

    fn index(&self, index: IndexTuple) -> &Self::Output {
        &self.fields[to_pos(index)]
    }
}

impl IndexMut<IndexTuple> for SudokuClassic {
    fn index_mut(&mut self, index: IndexTuple) -> &mut Self::Output {
        &mut self.fields[to_pos(index)]
    }
}
