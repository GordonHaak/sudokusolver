use kata::kata::sudoku::SudokuClassic;

fn main() -> Result<(), String> {
    let mut s = " ,1, , ,3,8, ,6,
          , , , , ,1, ,4,5
         5,9, , , , , , , 
          , , ,3,9, ,1, , 
         6,5, , , , , , , 
          , , ,1,6, , ,2, 
          , , ,6,1,4, , , 
          , ,7, , , , , , 
          , , , , , ,8, ,9"
        .parse::<SudokuClassic>()
        .unwrap();
    s.solve();
    println!("one result is : \n{}", s.to_string());
    Ok(())
}
