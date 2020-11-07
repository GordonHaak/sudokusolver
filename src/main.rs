use sudoku::sudoku::Classic;

fn main() -> Result<(), String> {
    let mut s = ",1, , ,3,8, ,6,
          , , , , ,1, ,4,5
         5,9, , , , , , , 
          , , ,3,9, ,1, , 
         6,5, , , , , , , 
          , , ,1,6, , ,2, 
          , , ,6,1,4, , , 
          , ,7, , , , , , 
          , , , , , ,8, ,9"
        .parse::<Classic>()
        .unwrap();
    println!("{}", s.solve().get(0).unwrap().to_string());
    println!("results  are : {:?}", s.solve_all());
    Ok(())
}
