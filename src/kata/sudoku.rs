use std::str::FromStr;
use std::string::ToString;

pub mod index;
pub mod iterator;

#[derive(Debug)]
pub struct SudokuClassic {
    fields: Vec<Option<u8>>,
}

impl SudokuClassic {
    const ROWS: u8 = 9;
    const COLS: u8 = 9;

    pub fn row<'a>(&'a self, row: u8) -> Box<dyn Iterator<Item = &'a Option<u8>> + 'a> {
        Box::new(iterator::LineIterator::new(self, row))
    }

    pub fn col<'a>(&'a self, col: u8) -> Box<dyn Iterator<Item = &'a Option<u8>> + 'a> {
        Box::new(iterator::ColumnIterator::new(self, col))
    }

    pub fn field<'a>(&'a self, row: u8, col: u8) -> Box<dyn Iterator<Item = &'a Option<u8>> + 'a> {
        Box::new(iterator::FieldIterator::new(self, row, col))
    }

    pub fn next_free_field(&mut self) -> Option<(u8, u8)> {
        None
    }

    fn index(row: u8, col: u8) -> usize {
        if row >= SudokuClassic::ROWS || col >= SudokuClassic::COLS {
            panic!("invalid index for Sudoku row {}, col {}", row, col);
        }
        (row * SudokuClassic::ROWS + col).into()
    }
}

impl FromStr for SudokuClassic {
    type Err = String;

    fn from_str(data: &str) -> Result<SudokuClassic, String> {
        use csv::{ReaderBuilder, Trim};
        let mut sudoku = vec![];
        let mut reader = ReaderBuilder::new()
            .has_headers(false)
            .trim(Trim::All)
            .from_reader(data.as_bytes());
        for row in reader.records() {
            let mut v: Vec<_> = row
                .unwrap()
                .iter()
                .map(|s| s.parse().unwrap_or(0))
                .map(|v| if v < 1 || v > 9 { None } else { Some(v) })
                .collect();
            if v.len() != SudokuClassic::COLS.into() {
                return Err(format!("expected 9 columns, got {:}", v.len()));
            }
            sudoku.append(&mut v);
        }
        if sudoku.len() != (SudokuClassic::COLS * SudokuClassic::ROWS).into() {
            return Err(format!("invalid sudoku has {} rows", sudoku.len() / 9));
        }
        Ok(SudokuClassic { fields: sudoku })
    }
}

impl ToString for SudokuClassic {
    fn to_string(&self) -> String {
        let get_row_value = |row: &[Option<u8>]| {
            row.iter()
                .map(|v| v.map_or("_".to_string(), |l| l.to_string()))
                .collect::<Vec<_>>()
                .join(",")
        };
        let chunks: Vec<_> = self.fields.chunks(SudokuClassic::COLS.into()).collect();
        chunks
            .iter()
            .map(|row| get_row_value(row))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

impl Default for SudokuClassic {
    fn default() -> Self {
        SudokuClassic {
            fields: vec![None; (SudokuClassic::ROWS * SudokuClassic::COLS).into()],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SudokuClassic;

    #[test]
    fn sudoku() {
        let sudoku = SudokuClassic::default();
        assert_eq!(sudoku[(0, 0)], None);
    }

    #[test]
    #[should_panic(expected = "invalid index for Sudoku row 9, col 0")]
    fn invalid_sudoku_access() {
        let sudoku = SudokuClassic::default();
        assert_eq!(sudoku[(9, 0)], None);
    }

    #[test]
    fn wrong_data() {
        let sudoku = "".parse::<SudokuClassic>();
        assert!(sudoku.is_err())
    }

    #[test]
    fn from_string() {
        let sudoku = " , , , , , , , , 
             , , , , , , , , 
             , , , , , , , , 
             , , , , , , , , 
             , , , , , , , , 
             , , , , , , , , 
             , , , , , , , , 
             , , , , , , , , 
             , , , , , , , , "
            .parse::<SudokuClassic>();
        //assert_eq!(sudoku.unwrap(), super::SudokuClassic::new());
    }

    #[test]
    fn field_iterator() {
        let sudoku: SudokuClassic = " , , , , , , , , 
             , , , , , , , , 
             , , , , , , , , 
             , , ,1,2,3, , , 
             , , ,4,5,6, , , 
             , , ,7,8,9, , , 
             , , , , , , , , 
             , , , , , , , , 
             , , , , , , , , "
            .parse()
            .unwrap();
        assert_eq!(
            sudoku.field(4, 5).map(|v| v.clone()).collect::<Vec<_>>(),
            vec![
                Some(1),
                Some(2),
                Some(3),
                Some(4),
                Some(5),
                Some(6),
                Some(7),
                Some(8),
                Some(9)
            ]
        );
    }
}
