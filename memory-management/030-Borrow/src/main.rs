fn main() {
    let mut numbers = vec![1, 2, 3, 5, 8];

    println!("The sum is {}", consume(&numbers)); // Passes reference, keeps ownership
    println!("The sum is {}", add_and_consume(&mut numbers)); // Mutable reference, keeps ownership

    println!("{:?}", numbers);
}

fn consume(numbers: &Vec<i32>) -> i32 {
    // numbers is read-only, cannot be mutated
    //numbers.push(42);  // -> does NOT COMPILE
    let sum: i32 = numbers.iter().sum();
    sum
}

fn add_and_consume(numbers: &mut Vec<i32>) -> i32 {
    numbers.push(42);
    consume(numbers)
}
