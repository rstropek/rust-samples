use std::{fmt::Display, ops::{Add, Sub}, str::FromStr};

// Learning: Tuple structs are just a type wrapped around other types
#[derive(Debug, Copy, Clone, PartialEq, Eq, Default, Ord, PartialOrd)]
pub struct BoardIndex(usize);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Direction {
    Horizontal,
    Vertical,
}

// Note: Implementation of BoardIndex is not relevant for the demo of traits. We just
//       need some code for a meaningful demo.
impl BoardIndex {
    pub fn new() -> BoardIndex {
        BoardIndex(0)
    }

    pub fn from_index(index: usize) -> BoardIndex {
        if index >= 3 * 3 {
            panic!("Index out of bounds");
        }

        BoardIndex(index)
    }

    pub fn from_col_row(col: usize, row: usize) -> BoardIndex {
        if col >= 3 {
            panic!("Column out of bounds");
        }

        if row >= 3 {
            panic!("Row out of bounds");
        }

        BoardIndex(row * 3 + col)
    }

    pub fn column(&self) -> usize {
        self.0 % 3
    }

    pub fn row(&self) -> usize {
        self.0 / 3
    }

    pub fn try_next(&self, direction: Direction) -> Option<BoardIndex> {
        match direction {
            Direction::Horizontal if self.column() < 2 => Some(BoardIndex::from_index(self.0 + 1)),
            Direction::Vertical if self.row() < 2 => Some(BoardIndex::from_index(self.0 + 3)),
            _ => None
        }
    }

    pub fn try_previous(&self, direction: Direction) -> Option<BoardIndex> {
        match direction {
            Direction::Horizontal if self.column() > 0 => Some(BoardIndex::from_index(self.0 - 1)),
            Direction::Vertical if self.row() > 0 => Some(BoardIndex::from_index(self.0 - 3)),
            _ => None
        }
    }

    pub fn next_column(&self) -> BoardIndex {
        if self.column() >= 2 { panic!("Already at last column"); }
        BoardIndex::from_index(self.0 + 1)
    }

    pub fn next_row(&self) -> BoardIndex {
        if self.row() >= 2 { panic!("Already at last row"); }
        BoardIndex::from_index(self.0 + 3)
    }

    pub fn previous_column(&self) -> BoardIndex {
        if self.column() == 0 { panic!("Already at first column"); }
        BoardIndex::from_index(self.0 - 1)
    }

    pub fn previous_row(&self) -> BoardIndex {
        if self.row() == 0 { panic!("Already at first row"); }
        BoardIndex::from_index(self.0 - 3)
    }
}

// Note: Some type casting traits, similar to what we did in square_content
impl From<BoardIndex> for usize {
    fn from(ix: BoardIndex) -> Self {
        ix.0
    }
}

impl From<BoardIndex> for String {
    fn from(ix: BoardIndex) -> Self {
        format!("{}", ix)
    }
}

impl From<usize> for BoardIndex {
    fn from(ix: usize) -> Self {
        BoardIndex::from_index(ix)
    }
}

impl From<&str> for BoardIndex {
    fn from(ix: &str) -> Self {
        ix.parse().unwrap()
    }
}

// Learning: FromStr trait for supporting the parse method
impl FromStr for BoardIndex {
    type Err = &'static str;

    fn from_str(location: &str) -> Result<Self, Self::Err> {
        let location = location.as_bytes(); // Note shadowing

        // Check if length of location is ok (A1..C3).
        if location.len() != 2 {
            return Err("Invalid length");
        }

        // Parse column letter (A..C, a..c)
        let col = match location[0] {
            r if matches!(r, b'A'..=b'C') => (r - b'A') as usize, // Check experimental `if let` syntax
            r if matches!(r, b'a'..=b'c') => (r - b'a') as usize,
            _ => return Err("Invalid column"),
        };

        // Parse the row letter(s) (1..3)
        let row = match location[1] {
            c if matches!(c, b'1'..=b'3') => (c - b'1') as usize,
            _ => return Err("Invalid row"),
        };

        Ok(BoardIndex::from_col_row(col, row))
    }
}

impl Display for BoardIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            char::from_u32((b'A' as usize + self.column()) as u32).unwrap(),
            self.row() + 1
        )
    }
}

// Learning: Add and Sub traits for supporting + and - operators
impl Add<usize> for BoardIndex {
    type Output = Self;

    fn add(self, other: usize) -> Self {
        (self.0 + other).into()
    }
}

impl Sub<usize> for BoardIndex {
    type Output = Self;

    fn sub(self, other: usize) -> Self {
        (self.0 - other).into()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn new() {
        assert_eq!(0usize, BoardIndex::new().into());
    }

    #[test]
    fn from_index() {
        assert_eq!(3usize, BoardIndex::from_index(3).into());
    }

    #[test]
    fn from_usize() {
        assert_eq!(BoardIndex::from_index(3), 3.into());
    }

    #[test]
    #[should_panic]
    fn from_index_out_of_bounds() {
        BoardIndex::from_index(3 * 3);
    }

    #[test]
    fn from_row_col() {
        assert_eq!(3usize, BoardIndex::from_col_row(0, 1).into());
    }

    #[test]
    #[should_panic]
    fn from_row_col_row_out_of_bounds() {
        BoardIndex::from_col_row(0, 3);
    }

    #[test]
    #[should_panic]
    fn from_row_col_col_out_of_bounds() {
        BoardIndex::from_col_row(3, 0);
    }

    #[test]
    fn from_str_lowest() {
        // Note: BoardIndex implements FromStr -> we can use the parse method
        let ix: BoardIndex = "A1".parse().unwrap();
        assert_eq!(0usize, ix.into());
    }

    #[test]
    fn from_str_highest() {
        // Learning: We can use parse, but we can also call from_str directly
        let ix = BoardIndex::from_str("C3").unwrap();
        assert_eq!(3usize * 3 - 1, ix.into());
    }

    #[test]
    fn from_str_lowercase() {
        assert_eq!(0usize, BoardIndex::from_str("a1").unwrap().into());
    }

    #[test]
    fn into_str() {
        // Learning: "Into::<String>" is called "Turbofish" syntax.
        //           We cann call a trait method directly on a given type.
        // Note: We have not implemented Into, just From. Why is turbofish
        //       with Into still possible? The solution lies in the generic 
        //       implementation of Into for types implementing from. See
        //       https://github.com/rust-lang/rust/blob/eba3228b2a9875d268ff3990903d04e19f6cdb0c/library/core/src/convert/mod.rs#L533
        // Learning: You can implement traits based on other traits 🤯
        assert_eq!("A1", Into::<String>::into(BoardIndex::new()));
    }

    #[test]
    fn into_index_from_str() {
        assert_eq!(BoardIndex::new(), "A1".parse().unwrap());
    }

    #[rstest]
    #[case("B01")] // too long
    #[case("B4")] // too high column
    #[case("B0")] // too low column
    #[case("B")] // missing column
    #[case("3")] // missing row
    #[case("99")] // missing row
    fn try_parse_errors(#[case] location: &'static str) {
        assert!(BoardIndex::from_str(location).is_err());
    }

    #[rstest]
    #[case(0, 0, 0)]
    #[case(2, 1, 2)]
    #[case(1, 1, 1)]
    fn get_column(#[case] col: usize, #[case] row: usize, #[case] expected_col: usize) {
        assert_eq!(expected_col, BoardIndex::from_col_row(col, row).column());
    }

    #[rstest]
    #[case(0, 0, 0)]
    #[case(2, 1, 1)]
    #[case(1, 2, 2)]
    fn get_row(#[case] col: usize, #[case] row: usize, #[case] expected_row: usize) {
        assert_eq!(expected_row, BoardIndex::from_col_row(col, row).row());
    }

    #[rstest]
    #[case(0, 0, "A1")]
    #[case(1, 1, "B2")]
    #[case(2, 2, "C3")]
    fn try_display(#[case] col: usize, #[case] row: usize, #[case] location: &'static str) {
        assert_eq!(location, format!("{}", BoardIndex::from_col_row(col, row)));
    }

    #[test]
    fn add() {
        assert_eq!(BoardIndex::from_index(2), BoardIndex::from_index(1) + 1);
    }

    #[test]
    fn sub() {
        assert_eq!(BoardIndex::from_index(0), BoardIndex::from_index(1) - 1);
    }

    #[test]
    #[should_panic]
    fn add_overflow() {
        let _ = BoardIndex::from_index(3 * 3 - 1) + 1;
    }

    #[test]
    #[should_panic]
    fn sub_overflow() {
        let _ = BoardIndex::from_index(0) - 1;
    }

    #[rstest]
    #[case(BoardIndex::from_index(0), Direction::Horizontal, Some(BoardIndex::from_index(1)))]
    #[case(BoardIndex::from_index(0), Direction::Vertical, Some(BoardIndex::from_index(3)))]
    #[case(BoardIndex::from_index(2), Direction::Horizontal, None)]
    #[case(BoardIndex::from_col_row(0, 2), Direction::Vertical, None)]
    fn try_get_next(#[case] ix: BoardIndex, #[case] direction: Direction, #[case] expected_ix: Option<BoardIndex>) {
        assert_eq!(expected_ix, ix.try_next(direction));
    }

    #[rstest]
    #[case(BoardIndex::from_index(1), Direction::Horizontal, Some(BoardIndex::from_index(0)))]
    #[case(BoardIndex::from_col_row(0, 1), Direction::Vertical, Some(BoardIndex::from_index(0)))]
    #[case(BoardIndex::from_index(0), Direction::Horizontal, None)]
    #[case(BoardIndex::from_col_row(2, 0), Direction::Vertical, None)]
    fn try_get_prev(#[case] ix: BoardIndex, #[case] direction: Direction, #[case] expected_ix: Option<BoardIndex>) {
        assert_eq!(expected_ix, ix.try_previous(direction));
    }

    #[test]
    #[should_panic]
    fn next_row_invalid() {
        BoardIndex::from_str("A3").unwrap().next_row();
    }

    #[test]
    #[should_panic]
    fn previous_row_invalid() {
        BoardIndex::from_str("A1").unwrap().previous_row();
    }

    #[test]
    #[should_panic]
    fn next_column_invalid() {
        BoardIndex::from_str("C1").unwrap().next_column();
    }

    #[test]
    #[should_panic]
    fn previous_column_invalid() {
        BoardIndex::from_str("A1").unwrap().previous_column();
    }

    #[test]
    fn next_row() {
        assert_eq!(2, BoardIndex::from_str("A2").unwrap().next_row().row());
    }

    #[test]
    fn previous_row() {
        assert_eq!(1, BoardIndex::from_str("A3").unwrap().previous_row().row());
    }

    #[test]
    fn next_column() {
        assert_eq!(2, BoardIndex::from_str("B1").unwrap().next_column().column());
    }

    #[test]
    fn previous_column() {
        assert_eq!(0, BoardIndex::from_str("B3").unwrap().previous_column().column());
    }
}
