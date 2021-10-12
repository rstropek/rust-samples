fn main() {
    let numbers = vec![1, 2, 3, 5, 8];
    consume(numbers); // Gives ownership to `consume`
    let produced_numbers = produce(); // Takes ownership
    println!("{:?}", produced_numbers);
    // produced_numbers gets of of scope -> free memory
}

fn consume(numbers: Vec<i32>) {
    let sum: i32 = numbers.iter().sum();
    println!("The sum is {}", sum);
    // numbers gets out of scope -> free memory
}

fn produce() -> Vec<i32> {
    let mut numbers: Vec<i32> = Vec::new();
    for i in 0..4 {
        numbers.push(i);
    }
    numbers // Gives ownership to caller
}
