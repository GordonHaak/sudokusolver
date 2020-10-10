use kata::kata::sudoku::SudokuClassic;

fn main() -> Result<(), String> {
    let s = SudokuClassic::from_string(
        " ,9,5, , , , , ,
          , , , , , , , , 
          , , , , , , , , 
          , , , , , , , , 
          , , , , , , , , 
          , , , , , , , , 
          , , , , , , , , 
          , , , , , , , , 
          , , , , , , , ,",
    )
    .unwrap();
    println!("Sudoku is : \n{}", s);
    Ok(())
}
