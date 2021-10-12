#[derive(Clone, Debug)]
struct Vector2d {
    x: f64,
    y: f64,
}

fn main() {
    // Allocate array on heap
    let numbers = vec![1, 2, 3, 5, 8];
    println!("{:?}", numbers);

    // Move ownership to other_numbers
    let other_numbers = numbers;
    println!("{:?}", other_numbers);

    // Now we cannot access numbers anymore
    // because value was moved.
    // println!("{:?}", numbers); // -> does NOT COMPILE

    // Make a copy -> no move of ownership
    let cloned_numbers = other_numbers.clone();
    println!("clone = {:?}, source = {:?}", cloned_numbers, other_numbers);

    // Let's put a custom struct on the heap and clone it -> deep copy.
    // Try removing Clone trait from Vector2d -> does not compile.
    let number_on_heap = Box::new(Vector2d { x: 1.0, y: 2.0 });
    let another_number_on_heap = Clone::clone(&number_on_heap);
    println!(
        "{:?} at {:p}",
        another_number_on_heap, another_number_on_heap
    );
    println!("{:?} at {:p}", number_on_heap, number_on_heap);
}
