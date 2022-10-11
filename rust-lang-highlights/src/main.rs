#![allow(unused_variables, dead_code, unused_mut)]

use std::{
    collections::HashMap,
    ops::{Add, Index, IndexMut},
    str::FromStr,
};

fn main() {
    // Constants
    {
        const NEARLY_THE_ANSWER: i32 = 41;
        const ANSWER: i32 = NEARLY_THE_ANSWER + 1;
        const UTF8_STR: &str = "Hello, world!";
        const BYTES: &[u8] = b"Hello, world!";
        const NUMBERS: [i32; 5] = [1, 2, 3, 4, 5];

        struct Customer<'a> {
            name: &'a str,
            age: u8,
        }
        const CUSTOMER: Customer = Customer {
            name: UTF8_STR,
            age: 42,
        };

        trait HasNumbers {
            const NUMBERS: [i32; 5];
            const LAST_NUMBER: i32 = 5;
        }
        struct IHaveNumbers {}
        impl HasNumbers for IHaveNumbers {
            const NUMBERS: [i32; 5] = [1, 2, 3, 4, IHaveNumbers::LAST_NUMBER];
        }
        println!("The numbers are {:?}", IHaveNumbers::NUMBERS);

        const fn numbers<const N: usize>() -> [i32; N] {
            let mut numbers = [0i32; N];

            let mut i = 0;
            while i < N {
                numbers[i] = i as i32 + 1;
                i += 1;
            }

            numbers
        }
        const TEN_NUMBERS: [i32; 10] = numbers();
        println!("The ten numbers are {:?}", TEN_NUMBERS);
    }

    // Variables
    {
        let x = 42;
        let mut y = 42i32;
        let z: i32 = 42;

        let /*mut*/ numbers = vec![1u8, 2, 3, 4, 5];
        //numbers.push(6);

        let number = "42";
        let number: i32 = number.parse().unwrap_or(0); // Note shadowing
        println!("The number is {}", number);
    }

    // Type inference
    {
        let mut numbers = Vec::new();
        numbers.push(1);

        fn get_values() -> Vec<i64> {
            let numbers = vec![1, 2, 3, 4, 5];
            numbers
        }

        let number: i32 = "42".parse().unwrap();
        let number = "42".parse::<i32>().unwrap();

        fn get_numbers_with_index() -> HashMap<usize, i32> {
            let numbers = vec![1, 2, 3, 4, 5];
            numbers
                .into_iter()
                .enumerate()
                .map(|item| (item.0, item.1 * 2))
                .collect()
        }
    }

    // Statements vs. Expressions
    {
        let x = 42;
        if x == 42 {
            println!("The answer to life, the universe, and everything");
        }

        let msg = if x == 42 { "The answer to life, the universe, and everything" } else { "Not the answer" };

        let x = 'outer: loop {
            for i in 0..100 {
                if i == 42 {
                    break 'outer i;
                }
            }
        };
        println!("The answer is {}", x);
    }

    // Everything traits
    {
        #[derive(Debug)]
        struct Vector2d {
            x: f32,
            y: f32,
        }
        impl Vector2d {
            fn new(x: f32, y: f32) -> Self {
                Self { x, y }
            }
            fn from_slice(vals: &[f32]) -> Self {
                if vals.len() != 2 {
                    panic!("Invalid length");
                }

                Self {
                    x: vals[0],
                    y: vals[1],
                }
            }
        }

        trait HasLength {
            fn length(&self) -> f32;
        }
        impl HasLength for Vector2d {
            fn length(&self) -> f32 {
                (self.x * self.x + self.y * self.y).sqrt()
            }
        }

        impl Default for Vector2d {
            fn default() -> Self {
                Self::new(Default::default(), Default::default())
            }
        }
        let v: Vector2d = Default::default();

        impl Add for Vector2d {
            type Output = Self;
            fn add(self, other: Self) -> Self {
                Self::new(self.x + other.x, self.y + other.y)
            }
        }
        let v = v + Vector2d::new(1f32, 2f32);

        #[derive(Debug)]
        enum ParseErr {
            TooFewElements,
            CannotParse,
            TooManyElements,
        }
        impl FromStr for Vector2d {
            type Err = ParseErr;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let mut parts = s.split(',');
                let mut vals = [0f32; 2];
                for i in 0..2 {
                    vals[i] = parts
                        .next()
                        .ok_or(ParseErr::TooFewElements)?
                        .parse()
                        .map_err(|_| ParseErr::CannotParse)?;
                }
                if parts.next().is_some() {
                    return Err(ParseErr::TooManyElements);
                }
                Ok(Self::from_slice(&vals))
            }
        }
        let v: Vector2d = "1,2".parse().unwrap();
        // let v: Vector2d = "12".parse().unwrap();
        // let v: Vector2d = "1,a".parse().unwrap();
        // let v: Vector2d = "1,2,3".parse().unwrap();
        println!("The vector is {v:?}");
    }

    // Traits on traits, macros
    {
        #[derive(Clone, Copy, PartialEq, Eq)]
        enum SquareContent {
            Empty,
            Occupied(char),
        }
        struct Board([SquareContent; 9]);
        #[derive(PartialEq, Eq)]
        struct BoardIndex {
            x: usize,
            y: usize,
        }
        impl BoardIndex {
            fn new(x: usize, y: usize) -> Self {
                if x >= 3 || y >= 3 {
                    panic!("Invalid index");
                }

                Self { x, y }
            }
        }
        impl From<BoardIndex> for usize {
            fn from(index: BoardIndex) -> Self {
                index.x + index.y * 3
            }
        }
        impl From<&str> for BoardIndex {
            fn from(location: &str) -> Self {
                let location = location.as_bytes(); // Note shadowing
        
                if location.len() != 2 {
                    panic!("Invalid length");
                }
        
                let x = match location[0] {
                    r if matches!(r, b'A'..=b'C') => (r - b'A') as usize,
                    r if matches!(r, b'a'..=b'c') => (r - b'a') as usize,
                    _ => panic!("Invalid column"),
                };
        
                let y = match location[1] {
                        c if matches!(c, b'1'..=b'9') => (c - b'1') as usize,
                        _ => panic!("Invalid row"),
                };
        
                BoardIndex::new(x, y)
            }
        }

        impl Index<BoardIndex> for Board {
            type Output = SquareContent;
            fn index(&self, index: BoardIndex) -> &Self::Output {
                &self.0[Into::<usize>::into(index)]
            }
        }
        impl IndexMut<BoardIndex> for Board {
            fn index_mut(&mut self, index: BoardIndex) -> &mut Self::Output {
                &mut self.0[Into::<usize>::into(index)]
            }
        }

        trait HasWinner {
            fn winner(&self) -> Option<char>;
        }
        impl<T> HasWinner for T
        where
            T: Index<BoardIndex, Output = SquareContent>,
        {
            fn winner(&self) -> Option<char> {
                for y in 0..2 {
                    if let SquareContent::Occupied(winner) = self[BoardIndex::new(0, y)] {
                        if self[BoardIndex::new(0, y)] == self[BoardIndex::new(1, y)]
                            && self[BoardIndex::new(1, y)] == self[BoardIndex::new(2, y)]
                        {
                            return Some(winner);
                        }
                    }
                }

                for x in 0..2 {
                    if let SquareContent::Occupied(winner) = self[BoardIndex::new(x, 0)] {
                        if self[BoardIndex::new(x, 0)] == self[BoardIndex::new(x, 1)]
                            && self[BoardIndex::new(x, 1)] == self[BoardIndex::new(x, 2)]
                        {
                            return Some(winner);
                        }
                    }
                }

                if let SquareContent::Occupied(winner) = self[BoardIndex::new(1, 1)] {
                    if (self[BoardIndex::new(0, 0)] == self[BoardIndex::new(1, 1)]
                        && self[BoardIndex::new(1, 1)] == self[BoardIndex::new(2, 2)])
                        || (self[BoardIndex::new(0, 2)] == self[BoardIndex::new(1, 1)]
                            && self[BoardIndex::new(1, 1)] == self[BoardIndex::new(2, 0)])
                    {
                        return Some(winner);
                    }
                }

                None
            }
        }

        let mut board = Board([SquareContent::Empty; 9]);
        board["A1".into()] = SquareContent::Occupied('X');
        board["A2".into()] = SquareContent::Occupied('X');
        board["A3".into()] = SquareContent::Occupied('X');
        println!("The winner is {:?}", board.winner());

        macro_rules! board {
            (+---+---+---+
             |$a1:literal|$b1:literal|$c1:literal|
             +---+---+---+
             |$a2:literal|$b2:literal|$c2:literal|
             +---+---+---+
             |$a3:literal|$b3:literal|$c3:literal|
             +---+---+---+
            ) => {
                Board([SquareContent::Occupied($a1), SquareContent::Occupied($b1), SquareContent::Occupied($c1),
                       SquareContent::Occupied($a2), SquareContent::Occupied($b2), SquareContent::Occupied($c2),
                       SquareContent::Occupied($a3), SquareContent::Occupied($b3), SquareContent::Occupied($c3)])
            };
        }
        let mut board = board!(
            +---+---+---+
            |'O'|'X'|'X'|
            +---+---+---+
            |' '|'O'|' '|
            +---+---+---+
            |' '|' '|'O'|
            +---+---+---+
        );
        println!("The winner is {:?}", board.winner());
    }
}
