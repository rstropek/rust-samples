use rand::Rng;

// Naive word puzzle generator

pub fn place_words(words: &[String], size: usize) -> Result<Vec<Vec<char>>, Vec<String>> {
    let mut rng = rand::thread_rng();

    // Make sure that all words are shorter than the size of the grid
    if words.iter().any(|word| word.len() > size) {
        return Err(words
            .iter()
            .filter(|word| word.len() > size)
            .map(|word| word.to_string())
            .collect());
    }

    // Create empty grid
    let mut grid = vec![vec!['.'; size]; size];

    // Remember unplaced words
    let mut unplaced_words = Vec::new();

    for word in words {
        let mut placed = false;

        // In this naive implementation, we try to place a word 100 times before giving up.
        for _ in 0..100 {
            let direction = rng.gen_range(0..=2); // 0: horizontal, 1: diagonal, 2: vertical
            placed = match direction {
                0 => place_word_horizontally(word, &mut grid),
                1 => place_word_diagonally(word, &mut grid),
                2 => place_word_vertically(word, &mut grid),
                _ => panic!("Invalid direction"),
            };
            if placed {
                break;
            }
        }

        // If we couldn't place a word, remember it
        if !placed {
            unplaced_words.push(word.to_string());
        }
    }

    if unplaced_words.is_empty() {
        fill_remaining_spots(&mut grid);
        Ok(grid)
    } else {
        Err(unplaced_words)
    }
}

fn place_word_horizontally(word: &str, grid: &mut Vec<Vec<char>>) -> bool {
    let mut rng = rand::thread_rng();
    let max_tries = 10;

    for _ in 0..max_tries {
        let row = rng.gen_range(0..grid.len());
        let col = rng.gen_range(0..=grid.len() - word.len());

        if can_place_word_horizontally(word, row, col, grid) {
            for (i, c) in word.chars().enumerate() {
                grid[row][col + i] = c;
            }
            return true;
        }
    }

    false
}

fn can_place_word_horizontally(word: &str, row: usize, col: usize, grid: &[Vec<char>]) -> bool {
    word.chars()
        .enumerate()
        .all(|(i, c)| grid[row][col + i] == '.' || grid[row][col + i] == c)
}

fn place_word_diagonally(word: &str, grid: &mut Vec<Vec<char>>) -> bool {
    let mut rng = rand::thread_rng();
    let max_tries = 10;

    for _ in 0..max_tries {
        let row = rng.gen_range(0..=grid.len() - word.len());
        let col = rng.gen_range(0..=grid.len() - word.len());

        if can_place_word_diagonally(word, row, col, grid) {
            for (i, c) in word.chars().enumerate() {
                grid[row + i][col + i] = c;
            }
            return true;
        }
    }

    false
}

fn can_place_word_diagonally(word: &str, row: usize, col: usize, grid: &[Vec<char>]) -> bool {
    word.chars()
        .enumerate()
        .all(|(i, c)| grid[row + i][col + i] == '.' || grid[row + i][col + i] == c)
}

fn place_word_vertically(word: &str, grid: &mut Vec<Vec<char>>) -> bool {
    let mut rng = rand::thread_rng();
    let max_tries = 10;

    for _ in 0..max_tries {
        let row = rng.gen_range(0..=grid.len() - word.len());
        let col = rng.gen_range(0..grid.len());

        if can_place_word_vertically(word, row, col, grid) {
            for (i, c) in word.chars().enumerate() {
                grid[row + i][col] = c;
            }
            return true;
        }
    }

    false
}

fn can_place_word_vertically(word: &str, row: usize, col: usize, grid: &[Vec<char>]) -> bool {
    word.chars()
        .enumerate()
        .all(|(i, c)| grid[row + i][col] == '.' || grid[row + i][col] == c)
}

fn fill_remaining_spots(grid: &mut [Vec<char>]) {
    let mut rng = rand::thread_rng();
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

    for row in grid.iter_mut() {
        for cell in row.iter_mut() {
            if *cell == '.' {
                *cell = alphabet
                    .chars()
                    .nth(rng.gen_range(0..alphabet.len()))
                    .unwrap();
            }
        }
    }
}
