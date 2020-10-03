use super::SudokuClassic;
use std::ops::{Index, IndexMut};

impl Index<(u8, u8)> for SudokuClassic {
    type Output = Option<u8>;

    fn index(&self, index: (u8, u8)) -> &Self::Output {
        let index = SudokuClassic::index(index.0, index.1);
        &self.fields[index]
    }
}

impl IndexMut<(u8, u8)> for SudokuClassic {
    fn index_mut(&mut self, index: (u8, u8)) -> &mut Self::Output {
        let index = SudokuClassic::index(index.0, index.1);
        &mut self.fields[index]
    }
}
