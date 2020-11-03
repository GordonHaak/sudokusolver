#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]

use std::str::FromStr;
use std::string::ToString;

pub mod index;
pub mod iterator;

#[derive(Debug, PartialEq, Clone)]
pub struct Classic {
    fields: Vec<Option<u8>>,
}

use iterator::{ColumnIterator, FieldIterator, LineIterator};

type IndexType = usize;
type IndexTuple = (IndexType, IndexType);
type IterType<'a> = Box<dyn Iterator<Item = &'a Option<u8>> + 'a>;

impl Classic {
    const ROWS: IndexType = 9;
    const COLS: IndexType = 9;

    #[must_use]
    pub fn row(&self, row: IndexType) -> IterType {
        Box::new(LineIterator::new(self, row))
    }

    #[must_use]
    pub fn col(&self, col: IndexType) -> IterType {
        Box::new(ColumnIterator::new(self, col))
    }

    #[must_use]
    pub fn field(&self, row: IndexType, col: IndexType) -> IterType {
        Box::new(FieldIterator::new(self, row, col))
    }

    fn is_valid_entry(&self, r: IndexType, c: IndexType) -> bool {
        let is_unique = |entries: IterType| -> bool {
            use std::collections::HashSet;
            let mut numbers: HashSet<u8> = HashSet::new();
            entries
                .filter_map(Option::as_ref)
                .all(|v| numbers.insert(*v))
        };
        is_unique(self.row(r)) && is_unique(self.col(c)) && is_unique(self.field(r, c))
    }

    #[must_use]
    pub fn solve_all(&self) -> Vec<Self> {
        self.clone().solve()
    }

    fn solve(&mut self) -> Vec<Self> {
        let free_field = self.fields.iter().position(Option::is_none);
        match free_field {
            None => vec![self.clone()],
            Some(position) => {
                let mut result = Vec::new();
                let (r, c) = (position / 9, position % 9);
                for i in 1..=9 {
                    self[(r, c)] = Some(i);
                    if self.is_valid_entry(r, c) {
                        result.append(&mut self.solve());
                    }
                    self[(r, c)] = None; //remove the entry to allow another try
                }
                result
            }
        }
    }
}

impl FromStr for Classic {
    type Err = String;

    fn from_str(data: &str) -> Result<Self, Self::Err> {
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
            if v.len() != Self::COLS {
                return Err(format!("expected 9 columns, got {:}", v.len()));
            }
            sudoku.append(&mut v);
        }
        if sudoku.len() != (Self::COLS * Self::ROWS) {
            return Err(format!("invalid sudoku has {} rows", sudoku.len() / 9));
        }
        Ok(Self { fields: sudoku })
    }
}

impl ToString for Classic {
    fn to_string(&self) -> String {
        let get_row_value = |row: &[Option<u8>]| {
            row.iter()
                .map(|v| v.map_or("_".to_string(), |l| l.to_string()))
                .collect::<Vec<_>>()
                .join(",")
        };
        let chunks: Vec<_> = self.fields.chunks(Self::COLS).collect();
        chunks
            .iter()
            .map(|row| get_row_value(row))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

impl Default for Classic {
    fn default() -> Self {
        Self {
            fields: vec![None; Self::ROWS * Self::COLS],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Classic;

    #[test]
    fn sudoku() {
        let sudoku = Classic::default();
        assert_eq!(sudoku[(0, 0)], None);
    }

    #[test]
    #[should_panic(expected = "invalid index for Sudoku row 9, col 0")]
    fn invalid_sudoku_access() {
        let sudoku = Classic::default();
        assert_eq!(sudoku[(9, 0)], None);
    }

    #[test]
    fn wrong_data() {
        let sudoku = "".parse::<Classic>();
        assert!(sudoku.is_err())
    }

    #[test]
    fn fromto() {
        let sudoku = Classic::default();
        assert_eq!(sudoku.to_string().parse::<Classic>().unwrap(), sudoku);
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
            .parse::<Classic>();
        assert_eq!(sudoku.unwrap(), Classic::default());
    }

    #[test]
    fn fielditer() {
        let sudoku = Classic::default();
        assert_eq!(sudoku.field(4, 4).count(), 9)
    }
    #[test]
    fn coliter() {
        let sudoku = Classic::default();
        assert_eq!(sudoku.col(0).count(), 9)
    }
    #[test]
    fn lineiter() {
        let sudoku = Classic::default();
        assert_eq!(sudoku.row(0).count(), 9)
    }

    #[test]
    fn field_iterator() {
        let sudoku: Classic = " , , , , , , , , 
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
            sudoku.field(4, 5).copied().collect::<Vec<_>>(),
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
