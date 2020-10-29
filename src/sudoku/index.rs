use super::IndexTuple;
use super::SudokuClassic;
use std::ops::{Index, IndexMut};

impl Index<IndexTuple> for SudokuClassic {
    type Output = Option<u8>;

    fn index(&self, index: IndexTuple) -> &Self::Output {
        let index = SudokuClassic::index(index);
        &self.fields[index]
    }
}

impl IndexMut<IndexTuple> for SudokuClassic {
    fn index_mut(&mut self, index: IndexTuple) -> &mut Self::Output {
        let index = SudokuClassic::index(index);
        &mut self.fields[index]
    }
}
