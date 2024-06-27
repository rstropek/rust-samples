/// Iterator generating fibonacci numbers
struct Fibonacci {
    curr: u64,
    next: u64,
}

impl Fibonacci {
    fn new() -> Self {
        Fibonacci { curr: 0, next: 1 }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let old_curr = self.curr;
        self.curr = self.next;
        self.next = old_curr + self.next;
        Some(old_curr)
    }
}

fn main() {
    let fib = Fibonacci::new();
    for i in fib.take(10) {
        println!("{i}");
    }
}
