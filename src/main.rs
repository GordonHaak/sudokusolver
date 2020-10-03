use kata::kata::sudoku::SudokuClassic;

fn main() {
    let mut s = SudokuClassic::new();
    s[(0, 0)] = Some(3);
    println!("Sudoku is : \n{}", s);
}
