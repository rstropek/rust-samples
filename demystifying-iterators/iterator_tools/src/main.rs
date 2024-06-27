/// An iterator that finds palindromes in a sequence of elements.
struct Palindrome<I>
where
    I: Iterator,
{
    iter: I,
}

/// Implement the Iterator trait for the Palindrome struct.
impl<I> Iterator for Palindrome<I>
where
    I: Iterator,
    I::Item: ToString,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        // Find the next palindrome in the sequence.
        self.iter.find(|item| {
            let string = item.to_string();
            let reversed = string.chars().rev().collect::<String>();
            string == reversed
        })
    }
}

/// A trait that extends the Iterator trait with a method to find palindromes.
trait PalindromeIterator: Iterator {
    fn palindromes(self) -> Palindrome<Self>
    where
        Self: Sized,
        Self::Item: ToString,
    {
        Palindrome { iter: self }
    }
}

impl<I: Iterator> PalindromeIterator for I {}

fn main() {
    for palindrome in (0..1000u32).palindromes() {
        println!("{}", palindrome);
    }
}
