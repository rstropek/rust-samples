# Storybook

## What's New?

### Rust Before 1.53

```bash
docker run -it --rm rust:1.52.0
mkdir rust-demo
cd rust-demo
cargo new iter
```

* Connect VSCode to container
* Open folder */rust-demo/iter/*
* Install *Rust Analyzer*

```rust
fn main() {
    let numbers = [1, 2, 3, 4, 5];
    for n in /*&*/numbers { // <<< CHANGE HERE
        println!("{}", n);
    }
}
```

* `for n in numbers {` does not work
  * Iteration by value does not work
* `for n in &numbers {` works
  * Iteration by reference does work
  * Let's check if it is really an iteration by reference

```rust
fn main() {
    let mut numbers = [1, 2, 3, 4, 5];
    println!("{:p}", &numbers[0]); // (1) <<< ADD THIS
    for n in &mut numbers { *n *= 2; } // (3) <<< ADD THIS
    for n in &numbers {
        println!("{:p}: {}", n, n); // (2) <<< CHANGE THIS
    }
}
```

### Rust After 1.53

```bash
cd /tmp/
mkdir rust-demo
cd rust-demo
cargo new iter
```

* Open */tmp/rust-demo/iter* in VSCode (`code .`)

```rust
fn main() {
    // This worked already in Rust < 1.53
    for &number in &[1, 1, 2, 3, 5, 8, 13] {
        println!("The next value is {}", number);
    }

    // This works in Rust >= 1.53
    for number in [1, 1, 2, 3, 5, 8, 13] {
        println!("The next value is {}", number);
    }
}
```

## Deep Dive Into Iterators

### Basic Abstractions

* What is an iterator?
  * Abstraction that allows code to iterate through a collection of values read-only, forward-only.
* Like many languages, Rust has abstractions (=traits) for iterators
* Two core traits:
  * `std::iter::IntoIterator`
  * `std::iter::Iterator`
* `std::iter::IntoIterator`
  * [docs](https://doc.rust-lang.org/std/iter/trait.IntoIterator.html)
  * [src](https://github.com/rust-lang/rust/blob/f66e825f73d2bd7f8a763b723983850f891985b0/library/core/src/iter/traits/collect.rs#L204)
  * Enables `for` loop

```rust
fn main() {
    let numbers = vec![1, 1, 2, 3, 5, 8, 13];
    
    // std::vec::Vec implements IntoIterator,
    // therefore we can use it in a for loop.
    for number in numbers {
        println!("The next value is {}", number);
    } 
}
```

* `std::iter::Iterator`
  * [docs](https://doc.rust-lang.org/std/iter/trait.Iterator.html)
  * [src](https://github.com/rust-lang/rust/blob/f66e825f73d2bd7f8a763b723983850f891985b0/library/core/src/iter/traits/iterator.rs#L55)
  * Note: Default implementation for mapping, filtering, aggregating, etc. -> Power of abstraction
    * Want more? Check out [IterTools](https://github.com/rust-itertools/itertools)

### Let's Practice By Implementing a Generator

```rust
use rand::Rng;

struct Passwords {
    length: usize,
}

impl Passwords {
    fn new() -> Self {
        Self::with_length(10)
    }

    fn with_length(length: usize) -> Self {
        Passwords { length: length }
    }
}

impl IntoIterator for Passwords {
    type Item = String;
    type IntoIter = PasswordsIterator;
    fn into_iter(self) -> Self::IntoIter {
        PasswordsIterator {
            length: self.length,
        }
    }
}

struct PasswordsIterator {
    length: usize,
}

impl Iterator for PasswordsIterator {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        let mut result = String::with_capacity(self.length);
        for _ in 0..self.length {
            // For demo purposes, we generate a password
            // consisting only of letters a..z
            result.push((b'a' + rand::thread_rng().gen_range(0..=(b'z' - b'a'))) as char);
        }

        Some(result)
    }
}

fn main() {
    // Generate three passwords and print them in a for loop
    for p in Passwords::new().into_iter().take(3) {
        println!("The next password is {}", p);
    }

    // Generate three passwords and print them with for_each
    Passwords::with_length(5)
        .into_iter()
        .take(3)
        .for_each(|p| println!("The next password is {}", p));
}
```

### Consuming Iterators

* Let's look at some aspects of iterators that might not be obvious for Rust beginners.
* Note: `into_iter` takes ownership of underlying data structure.

```rust
fn main() {
    let numbers = vec![1, 1, 2, 3, 5, 8, 13];
    println!("The min is {}", numbers.into_iter().min().unwrap());
    // The following line does NOT work because into_iter 
    // CONSUMES the vector (i.e. takes ownership of it). Once
    // you call into_iter, you cannot access the underlying vector
    // anymore.
    println!("The sum is {}", numbers.into_iter().sum::<i32>());
}
```

* We solve that by borrowing a reference to the vector

```rust
fn main() {
    // The following example DOES work because numbers is a 
    // borrowed reference to the vector, not the vector itself.
    let numbers = &vec![1, 1, 2, 3, 5, 8, 13]; // <<< CHANGE TO REFERENCE
    println!("The min is {}", numbers.into_iter().min().unwrap());
    println!("The sum is {}", numbers.into_iter().sum::<i32>());
}
```

* Note: We do not need to change call to `into_iter()` because there is no `->` in Rust
* You can also solve the problem by using `iter()` instead of `into_iter()` because by convention, `iter()` borrows and iterates by reference
  * `iter_mut()` if you need mutable reference
* If you use `into_iter()`, the compiler tries to figure out what you want.

```rust
struct Point {
    x: f64,
    y: f64,
}

fn main() {
    // Iterate over items in a vector by value.
    let points = vec![Point{x: 1.0, y: 1.0}, Point{x: 2.0, y: 2.0}];
    let _first_point: Point = points.into_iter().next().unwrap();

    // Iterate over items in a vector by reference
    let points = vec![Point{x: 1.0, y: 1.0}, Point{x: 2.0, y: 2.0}];
    let mut iter = points.iter();    // points.iter() is equivalent
                                     // to (&points).into_iter()
    let _first_point: &Point = iter.next().unwrap();

    // Iterate over items in a vector by mutable reference
    let mut points = vec![Point{x: 1.0, y: 1.0}, Point{x: 2.0, y: 2.0}];
    let mut iter = points.iter_mut();  // points.iter_mut() is 
                                       // equivalent to (&mut points).into_iter()
    let first_point: &mut Point = iter.next().unwrap();
    first_point.x = 3.0; // As we have a mutable reference, we can change the content
    first_point.y = 4.0;
}
```

### Zero-Cost Abstractions

Try this code in Rust Playground and look at assembly code (debug vs. release).

```rust
fn main() {
    let mut sum = 0;
    for i in 0..10 {
        sum += i;
    }
    
    println!("{}", sum);
}
```

## Why Did It Take So Long?

### Const Generics

Let's write a short, naive sample with a generic trait.

```bash
docker run -it --rm rust:1.50.0
mkdir rust-demo
cd rust-demo
cargo new iter
```

* Connect VSCode to container
* Open folder */rust-demo/iter/*
* Install *Rust Analyzer*

```rust
trait IsOrdered {
    fn is_ordered(&self) -> bool;
}

impl<T> IsOrdered for [T; 4] where T: Ord + Copy {
// impl<T, const N: usize> IsOrdered for [T; N] where T: Ord + Copy { // <<< CHANGE THIS TO GENERIC
    fn is_ordered(&self) -> bool {
        if matches!(self.len(), 0..=1) {
            return true;
        }

        let mut prev = None;
        for item in self {
            if Some(item) < prev {
                return false;
            }

            prev = Some(item);
        }

        true
    }
}

fn main() {
    let ordered = [1, 2, 3, 4];
    println!("{}", ordered.is_ordered());
    let unordered = [1, 3, 2, 4];
    println!("{}", unordered.is_ordered())
}
```

* Takeaway: We needed const generics to be able to implement `IntoIterator` for arrays.
* Came with [Rust 1.51 (March 2021)](https://blog.rust-lang.org/2021/03/25/Rust-1.51.0.html#const-generics-mvp)

### Breaking Change

Let's re-implement iterator traits in a small sample. First, let's implement it only by reference.

```rust
use std::marker::PhantomData;

struct MyArray<T> {
    phantom: PhantomData<T>,
}

// See also https://github.com/rust-lang/rust/blob/f66e825f73d2bd7f8a763b723983850f891985b0/library/core/src/iter/traits/collect.rs#L204
trait MyIntoIterator {
    type Item;
    type IntoIter: MyIterator<Item = Self::Item>;
    fn into_iter(self) -> Self::IntoIter;
}

// See also https://github.com/rust-lang/rust/blob/f66e825f73d2bd7f8a763b723983850f891985b0/library/core/src/iter/traits/iterator.rs#L55
trait MyIterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

// See also https://github.com/rust-lang/rust/blob/f66e825f73d2bd7f8a763b723983850f891985b0/library/core/src/array/mod.rs#L191
impl<'a, T> MyIntoIterator for &'a MyArray<T> {
    type Item = &'a T;
    type IntoIter = MyIter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        MyIter::<'a, T>{ _phantom: &self.phantom }
    }
}

// See also https://github.com/rust-lang/rust/blob/f66e825f73d2bd7f8a763b723983850f891985b0/library/core/src/slice/iter.rs#L65
struct MyIter<'a, T: 'a> {
    _phantom: &'a PhantomData<T>,
}

// See also https://github.com/rust-lang/rust/blob/f66e825f73d2bd7f8a763b723983850f891985b0/library/core/src/slice/iter.rs#L133
// and https://github.com/rust-lang/rust/blob/f66e825f73d2bd7f8a763b723983850f891985b0/library/core/src/slice/iter/macros.rs#L130
impl<'a, T: 'a> MyIterator for MyIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

fn main() {
    let _arr = MyArray::<i32>{ phantom: PhantomData::<i32>{} };

    let mut iter: MyIter<i32> = _arr.into_iter();
    let _first_item: Option<&i32> = iter.next();
}
```

Now let's add by value:

```rust
// See also https://github.com/rust-lang/rust/blob/f66e825f73d2bd7f8a763b723983850f891985b0/library/core/src/array/mod.rs#L173
impl<T> MyIntoIterator for MyArray<T> {
    type Item = T;
    type IntoIter = MyIntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        MyIntoIter::<T>{ _phantom: self.phantom }
    }
}

// See https://github.com/rust-lang/rust/blob/f66e825f73d2bd7f8a763b723983850f891985b0/library/core/src/array/iter.rs#L13
struct MyIntoIter<T> {
    _phantom: PhantomData<T>,
}

// See https://github.com/rust-lang/rust/blob/f66e825f73d2bd7f8a763b723983850f891985b0/library/core/src/array/iter.rs#L102
impl<T> MyIterator for MyIntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}
```

This breaks main. But we can now iterate by value:

```rust
fn main() {
    let _arr = MyArray::<i32>{ phantom: PhantomData::<i32>{} };

    // let mut iter: MyIter<i32> = _arr.into_iter();
    // let _first_item: Option<&i32> = iter.next();

    let mut iter: MyIntoIter<i32> = MyArray::<i32>::into_iter(_arr);
    let _first_item: Option<i32> = iter.next();
}
```

This problem was solved with a trick:

```rust
use std::{vec::IntoIter};

fn main() {
    // Iterate over vector by value
    let numbers: Vec<i32> = vec![1, 1, 2, 3, 5, 8, 13];
    let mut num_iter: IntoIter<i32> = numbers.into_iter();
    let item: i32 = num_iter.next().unwrap();
    println!("{}", item);

    // Iterate over vector by reference
    let numbers: &Vec<i32> = &vec![1, 1, 2, 3, 5, 8, 13];
    let mut num_iter: std::slice::Iter<i32> = numbers.into_iter();
    let item: &i32 = num_iter.next().unwrap();
    println!("{}", item);

    // In contrast to vector, iterating over an array means
    // iterating BY REFERENCE, not by value. Note that this
    // code leads to a warning because this behavior will
    // change in future Rust versions.
    let numbers: [i32; 7] = [1, 1, 2, 3, 5, 8, 13];
    let mut num_iter: std::slice::Iter<i32> = numbers.into_iter();
    let item: &i32 = num_iter.next().unwrap();
    println!("{}", item);

    // Since Rust 1.53, we can explicitly call IntoIterator::into_iter
    // as it is now implemented for arrays. It will lead to an iteration
    // by value, NO LONGER by reference.
    let numbers: [i32; 7] = [1, 1, 2, 3, 5, 8, 13];
    let mut num_iter: std::array::IntoIter<i32, 7> = IntoIterator::into_iter(numbers);
    let item: i32 = num_iter.next().unwrap();
    println!("{}", item);

    // As IntoIterator has been implemented in Rust 1.53, the following
    // syntax is now possible. It iterates over the given array BY VALUE
    // using a for loop.
    for i in [1, 1, 2, 3, 5] {
        println!("{}", i)
    }
}
```

## So?

So much fuzz just to make this work ;-)

```rust
fn main() {
    for n in [1, 2, 3, 4, 5] {
        println!("{}", n);
    }
}
```
