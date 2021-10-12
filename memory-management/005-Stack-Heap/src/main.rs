fn main() {
    // Let's put something on the stack
    let on_stack = 42;
    println!("{:?} at {:p}", on_stack, &on_stack);
    
    // Now let's put something on the heap
    let on_heap = Box::new(42);
    println!("{:?} at {:p} pointed to by {:p}", on_heap, on_heap, &on_heap);

    // Let's put something more complex on the heap
    let mut vec_on_heap = vec![1, 2, 3, 4, 5, 6];
    println!("{:?} at {:p}", vec_on_heap, &vec_on_heap);
    println!("{:?} at {:p}", vec_on_heap[0], &vec_on_heap[0]);

    // Let's put anything on the heap just to have at least one 
    // allocation before the next experiment.
    let _something_on_heap = Box::new(42);

    // Now let's append something to our vector. What happened
    // to our pointers?
    // Learning: Sometimes, Rust moves your data around in memory.
    vec_on_heap.push(7);
    println!("{:?} at {:p}", vec_on_heap, &vec_on_heap);
    println!("{:?} at {:p}", vec_on_heap[0], &vec_on_heap[0]);

    // And finally: A string on the heap.
    let string_on_heap = String::from("FooBar");
    println!("{:?} at {:p} point to by {:p}", string_on_heap, string_on_heap.as_ptr(), &string_on_heap);

    // Is there something else than stack and heap?
    // Yes: Data segment of your program (e.g. &'static str)
    let str_on_heap: &'static str = "FooBar";
    println!("{:?} at {:p} point to by {:p}", str_on_heap, str_on_heap.as_ptr(), &str_on_heap);
}
