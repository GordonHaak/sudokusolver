use super::Classic;
use super::IndexTuple;
use std::ops::{Index, IndexMut};

fn to_pos((row, col): IndexTuple) -> usize {
    if row >= Classic::ROWS || col >= Classic::COLS {
        panic!("invalid index for Sudoku row {}, col {}", row, col);
    }
    row * Classic::ROWS + col
}

impl Index<IndexTuple> for Classic {
    type Output = Option<u8>;

    fn index(&self, index: IndexTuple) -> &Self::Output {
        &self.fields[to_pos(index)]
    }
}

impl IndexMut<IndexTuple> for Classic {
    fn index_mut(&mut self, index: IndexTuple) -> &mut Self::Output {
        &mut self.fields[to_pos(index)]
    }
}
