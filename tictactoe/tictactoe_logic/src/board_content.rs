use std::{convert::TryFrom, fmt, ops::{Index, IndexMut}};

use crate::SquareContent;

#[derive(Debug, Clone, Copy)]
pub struct BoardContent {
    board_content: [SquareContent; 3 * 3],
}

// Note: Implementation of BoardContent is not relevant for the demo of traits. We just
//       need some code for a meaningful demo.
impl BoardContent {
    pub fn new() -> Self {
        BoardContent::new_initialized(Default::default())
    }

    pub fn new_initialized(initial_content: SquareContent) -> BoardContent {
        BoardContent {
            board_content: [initial_content; 3 * 3],
        }
    }

    pub fn as_slice(&self) -> &[SquareContent] {
        &self.board_content
    }
}

// Learning: Create our own trait and implement it for a struct
pub trait ToCompactString {
    fn to_compact_str(&self) -> String;
}

impl ToCompactString for BoardContent {
    fn to_compact_str(&self) -> String {
        let mut result = String::new();
        result.reserve_exact(3 * 3);

        // Note: Iterating over array, brand new in Rust 1.53
        for c in self.board_content {
            result.push(c.into());
        }

        result
    }
}

// Question: Can we implement the to_compact_str function in a more abstract way?
//           Can we make it applicable for any array of proper size of items that
//           support conversion to char?
// Learning: Yes, we can. We can implement our own trait for the array's IntoIter trait.
pub trait BoardIterToCompactString {
    fn to_compact_str(&mut self) -> String;
}

impl<T: Into<char>> BoardIterToCompactString for core::array::IntoIter<T, 9> {
    fn to_compact_str(&mut self) -> String {
        let mut result = String::new();
        result.reserve_exact(3 * 3);

        // Note: Iterating over array, brand new in Rust 1.53
        for c in self {
            result.push(c.into());
        }

        result
    }
}

// Question: What's missing? We need to make our BoardContent an iterable.
// Learning: How to implement IntoIterator for our own struct
impl IntoIterator for BoardContent {
    type Item = SquareContent;

    type IntoIter = core::array::IntoIter<Self::Item, 9>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(self.board_content)
    }
}

impl Default for BoardContent {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct InvalidBoardSize; // Note unit struct (field-less struct)

// Learning: Apply trait to an error type
impl fmt::Display for InvalidBoardSize {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Board data is of invalid size. Has to contain {size} elements",
            size = 3 * 3
        )
    }
}

impl TryFrom<&[u8]> for BoardContent {
    // Note: We need the error type for implementing TryFrom.
    type Error = InvalidBoardSize;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        match bytes.len() {
            9 => {
                let mut content = BoardContent::new();
                for (ix, square_byte) in bytes.iter().copied().enumerate() {
                    content.board_content[ix] = square_byte.into();
                }

                Ok(content)
            }
            _ => Err(InvalidBoardSize),
        }
    }
}

impl From<BoardContent> for [u8; 3 * 3] {
    fn from(c: BoardContent) -> Self {
        let mut content: [u8; 3 * 3] = [SquareContent::default().into(); 3 * 3];
        for (ix, square) in c.board_content.iter().copied().enumerate() {
            content[ix] = square.into();
        }

        content
    }
}

// Learning: We can implement the Index trait to allow read-only indexing
impl Index<usize> for BoardContent {
    type Output = SquareContent;

    fn index(&self, ix: usize) -> &Self::Output {
        if ix >= 3 * 3 {
            panic!("Index out of bounds");
        }

        &self.board_content[ix]
    }
}

// Learning: We can implement the IndexMut trait to allow write indexing
impl IndexMut<usize> for BoardContent {
    fn index_mut(&mut self, ix: usize) -> &mut Self::Output {
        if ix >= 3 * 3 {
            panic!("Index out of bounds");
        }

        &mut self.board_content[ix]
    }
}

#[cfg(test)]
mod tests {
    use crate::SquareContent;

    use super::*;

    #[test]
    fn new() {
        let b = BoardContent::new();

        // Note: Here we make use of our IntoIterator implementation
        assert_eq!(b.into_iter().count(), 3 * 3);
        assert!(b.into_iter().all(|v| v == Default::default()));
    }

    #[test]
    fn new_initialized() {
        let square_content = SquareContent::X;
        let b = BoardContent::new_initialized(square_content);
        assert_eq!(b.into_iter().count(), 3 * 3);
        assert!(b.into_iter().all(|v| v == square_content));
    }

    #[test]
    fn clone() {
        let b = BoardContent::new_initialized(SquareContent::X);
        let b2 = b.clone();
        assert_eq!(SquareContent::X, b2[0]);
    }

    #[test]
    fn from_bytes() {
        let square_content = SquareContent::X;
        let content: &[u8] = &[square_content.into(); 3 * 3];
        let board = BoardContent::try_from(content).unwrap();
        assert_eq!(board.into_iter().count(), 3 * 3);
        assert!(board.into_iter().all(|v| v == square_content));
    }

    #[test]
    fn from_invalid_bytes() {
        let content: &[u8] = &[0; 99];
        let board = BoardContent::try_from(content);
        assert_eq!(Some(InvalidBoardSize), board.err());
    }

    #[test]
    fn into_bytes() {
        let board = BoardContent::new();
        let bytes: [u8; 3 * 3] = board.into();
        assert_eq!(bytes.iter().count(), 9);
        assert!(bytes.iter().all(|v| *v == SquareContent::default().into()));
    }

    #[test]
    #[should_panic]
    fn indexing_out_of_bounds() {
        BoardContent::new()[100];
    }

    #[test]
    #[should_panic]
    fn indexing_mut_out_of_bounds() {
        BoardContent::new()[100] = SquareContent::X;
    }

    #[test]
    fn indexing() {
        let mut b = BoardContent::new();
        let square_content = SquareContent::X;

        // Note: Here we make use of our implementation of read/write indexing
        b[8] = square_content;
        assert_eq!(square_content, b[8]);
    }

    #[test]
    fn slice_data() {
        let b = BoardContent::new();
        let bs = b.as_slice();
        assert_eq!(bs.len(), 3 * 3);
    }

    #[test]
    fn into_short_str() {
        let b = BoardContent::new();

        // Note: Here we call the to_compact_str implemented on BoardContent
        assert_eq!(format!("{: >9}", ""), b.to_compact_str());

        // Note: Here we call the to_compact_str implemented on an iterator
        assert_eq!(format!("{: >9}", ""), b.into_iter().to_compact_str());
    }
}
