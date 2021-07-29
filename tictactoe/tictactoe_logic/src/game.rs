use std::ops::{Index, IndexMut};

use crate::{BoardIndex, SquareContent};

pub struct Game<T> {
    content: T,
    current_player: u8,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SetError {
    error_kind: ErrorKind,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ErrorKind {
    CannotResetToEmpty,
    SquareAlreadyHasValue,
    WrongPlayerSetOrder,
}

#[cfg_attr(test, mockall::automock)]
pub trait SquareAccessor {
    fn get(&self, ix: BoardIndex) -> SquareContent;
    fn set(&mut self, ix: BoardIndex, value: SquareContent);
}

impl<T> SquareAccessor for T where T: Index<BoardIndex, Output = SquareContent> 
    + IndexMut<BoardIndex, Output = SquareContent> {
    fn get(&self, ix: BoardIndex) -> SquareContent {
        self[ix]
    }

    fn set(&mut self, ix: BoardIndex, value: SquareContent) {
        self[ix] = value;
    }
}

impl<T: SquareAccessor> Game<T> {
    fn is_valid(&self) -> bool {
        let sum = (0..9).map(|ix| {
            match self.content.get(BoardIndex::from_index(ix)) {
                SquareContent::Empty => 0,
                SquareContent::X => 1,
                SquareContent::O => -1
            }
        }).sum();
        matches!(sum, -1..=1)
    }

    pub fn set(&mut self, ix: BoardIndex, value: SquareContent) -> Result<(), SetError> {
        if value == SquareContent::Empty {
            return Err(SetError { error_kind: ErrorKind::CannotResetToEmpty});
        }
        let current_value = self.content.get(ix);
        if current_value != SquareContent::Empty {
            return Err(SetError { error_kind: ErrorKind::SquareAlreadyHasValue});
        }
        self.content.set(ix, value);
        if !self.is_valid() {
            self.content.set(ix, current_value);
            return Err(SetError { error_kind: ErrorKind::WrongPlayerSetOrder});
        }
        self.current_player = (self.current_player + 1) % 2;

        Ok(())
    }

    pub fn who_is_next(&self) -> u8 {
        self.current_player
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::eq;
    use rstest::rstest;

    #[test]
    fn is_valid() {
        let mut mock = MockSquareAccessor::new();
        mock.expect_get().returning(|ix: BoardIndex| { 
            match Into::<usize>::into(ix) % 2 {
                0 => SquareContent::X,
                1 => SquareContent::O,
                _ => panic!("Must never happen")
            }
        });

        let game = Game { content: mock, current_player: 0 };
        assert!(game.is_valid());
    }

    #[rstest]
    #[case(SquareContent::X)]
    #[case(SquareContent::O)]
    fn is_not_valid(#[case] value: SquareContent) {
        let mut mock = MockSquareAccessor::new();
        mock.expect_get().returning(move |ix: BoardIndex| { 
            match Into::<usize>::into(ix) {
                0 ..= 1 => value,
                _ => SquareContent::Empty
            }
        });

        let game = Game { content: mock, current_player: 0 };
        assert!(!game.is_valid());
    }

    #[test]
    fn set() {
        let mut mock = MockSquareAccessor::new();
        mock.expect_get().return_const(SquareContent::Empty);
        mock.expect_set().with(eq(BoardIndex::from_index(0)), eq(SquareContent::X)).return_const(()).times(1);
        let mut game = Game { content: mock, current_player: 0 };
        assert!(game.set(0.into(), SquareContent::X).is_ok());
    }

    #[test]
    fn set_empty_error() {
        let mock = MockSquareAccessor::new();
        let mut game = Game { content: mock, current_player: 0 };
        assert_eq!(SetError { error_kind: ErrorKind::CannotResetToEmpty}, game.set(0.into(), SquareContent::Empty).err().unwrap());
    }

    #[test]
    fn set_already_value_error() {
        let mut mock = MockSquareAccessor::new();
        mock.expect_get().return_const(SquareContent::X);
        let mut game = Game { content: mock, current_player: 0 };
        assert_eq!(SetError { error_kind: ErrorKind::CannotResetToEmpty}, game.set(0.into(), SquareContent::Empty).err().unwrap());
    }
}