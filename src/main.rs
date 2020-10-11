use kata::kata::sudoku::SudokuClassic;

fn main() -> Result<(), String> {
    let s = " ,9,5, , , , , ,
          , , , , , , , , 
          , , , , , , , , 
          , , , , , , , , 
          , , , , , , , , 
          , , , , , , , , 
          , , , , , , , , 
          , , , , , , , , 
          , , , , , , , ,"
        .parse::<SudokuClassic>()
        .unwrap();
    println!("Sudoku is : \n{}", s.to_string());
    Ok(())
}
