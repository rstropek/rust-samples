use std::{fmt, ops::Index};

use crate::{BoardContent, SquareContent};

// Represents a single row in BoardContent
pub struct Row<'a> {
    // Note: Lifetime annotation here means that the struct `Row`
    // cannot outlive the reference it hold in the `board` field.
    board: &'a BoardContent,
    pub row_index: usize,
}

impl<'a> Row<'a> {
    pub fn new(board: &'a BoardContent, row: usize) -> Self {
        Row { board, row_index: row }
    }

    pub fn as_slice(&self) -> &[SquareContent] {
        &self.board.as_slice()[(self.row_index * 3)..((self.row_index + 1) * 3)]
    }

    pub fn has_next(&self) -> bool {
        self.row_index < 2
    }
}

// Nothing new here, just another indexer (read-only)
impl<'a> Index<usize> for Row<'a> {
    type Output = SquareContent;

    fn index(&self, ix: usize) -> &Self::Output {
        if ix >= 3 {
            panic!("Index out of bounds");
        }

        &self.board[self.row_index * 3 + ix]
    }
}

pub struct RowsIterator<'a> {
    board: &'a BoardContent,
    next_row: usize,
}

impl<'a> RowsIterator<'a> {
    pub fn new(board: &'a BoardContent) -> Self {
        RowsIterator { board, next_row: 0 }
    }
}

// Learning: How to implement a customer iterator
impl<'a> Iterator for RowsIterator<'a> {
    type Item = Row<'a>;

    fn next(&mut self) -> Option<Row<'a>> {
        if self.next_row >= 3 {
            return None;
        }

        let current = self.next_row;
        self.next_row += 1;
        Some(Row { board: self.board, row_index: current })
    }
}

// Learning: Implement a trait with a lifetime annotation.
// Note: Lifetime annotation here means that the iterator
//       cannot outlive the struct implementing the iterator.
trait Rows<'a> {
    fn row(&self, row: usize) -> Row;

    // Learn: Dynamic binding instead of static binding
    fn rows(&'a self) -> Box<dyn Iterator<Item = Row> + 'a>;
}

// Learning: Implement our trait for BoardContent
impl<'a> Rows<'a> for BoardContent {
    fn row(&self, row: usize) -> Row {
        if row >= 3 {
            panic!("Index out of bounds");
        }

        Row::new(self, row)
    }

    fn rows(&'a self) -> Box<dyn Iterator<Item = Row> + 'a> {
        Box::new(RowsIterator::new(self))
    }
}

impl fmt::Display for BoardContent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fn build_separator(chars: &[char]) -> String {
            debug_assert!(chars.len() == 4);

            let mut result = String::new();
            result.reserve_exact(
                chars[0].len_utf8()
                    + 3 * 2 * chars[1].len_utf8()
                    + (3 - 1) * chars[2].len_utf8()
                    + chars[3].len_utf8()
                    + '\n'.len_utf8(),
            );
            result.push(chars[0]);
            for _ in 0..3 - 1 {
                result.push(chars[1]);
                result.push(chars[1]);
                result.push(chars[2]);
            }

            result.push(chars[1]);
            result.push(chars[1]);
            result.push(chars[3]);

            result.push('\n');

            result
        }

        let top = build_separator(&['┏', '━', '┯', '┓']);
        let middle = build_separator(&['┠', '─', '┼', '┨']);
        let bottom = build_separator(&['┗', '━', '┷', '┛']);

        let mut result = String::new();
        result.reserve_exact(
            top.len()
                + middle.len() * (3 - 1)
                + bottom.len()
                + (2 * '┃'.len_utf8()
                    + (3 - 1) * '|'.len_utf8()
                    + 3 * 2 * ' '.len_utf8()
                    + '\n'.len_utf8())
                    * 3,
        );

        result.push_str(&top);

        // Note: Here we use our rows iterator
        for row in self.rows() {
            result.push('┃');
            for col in 0..3 {
                let char = row[col].into();
                result.push(char);
                result.push(char);
                if col < 2 {
                    result.push('|');
                }
            }

            result.push('┃');
            result.push('\n');
            if row.row_index < 2 {
                result.push_str(&middle);
            }
        }

        result.push_str(&bottom);

        write!(f, "{}", result)
    }
}

#[cfg(test)]
mod tests {
    use crate::{BoardContent, SquareContent};

    use super::*;

    #[test]
    fn slice_row() {
        let mut b = BoardContent::new();
        let square_content = SquareContent::X;
        b[3] = square_content;

        // Note that we need the following variable to extend the lifetime of 
        // the `Row` instance. Otherwise, we could not call `data()` as our row
        // would have been clean up before the call. So the following line would NOT work:
        // let rs = b.row(1).data(); // <<< DOES NOT COMPILE
        let row = b.row(1);
        let rs = row.as_slice();
        assert_eq!(rs.len(), 3);
        assert_eq!(rs[0], square_content);
    }

    #[test]
    fn row_iterator() {
        let mut b = BoardContent::new();
        let square_content = SquareContent::X;
        b[3] = square_content;
        assert_eq!(3, b.rows().count());
        assert_eq!(b.rows().nth(1).unwrap().as_slice()[0], square_content);
    }

    #[test]
    fn into_string() {
        let mut b = BoardContent::new();
        b[4] = SquareContent::X;
        let str = format!("{}", b);

        // This is how it should look like:
        // ┏━━┯━━┯━━┓
        // ┃  |  |  ┃
        // ┠──┼──┼──┨
        // ┃  |XX|  ┃
        // ┠──┼──┼──┨
        // ┃  |  |  ┃
        // ┗━━┷━━┷━━┛

        assert_eq!("┏━━┯━━┯━━┓\n┃  |  |  ┃\n┠──┼──┼──┨\n┃  |XX|  ┃\n┠──┼──┼──┨\n┃  |  |  ┃\n┗━━┷━━┷━━┛\n", str);
    }
}
