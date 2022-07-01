// Further readings:
// What's new in Rust 1.61: https://blog.rust-lang.org/2022/05/19/Rust-1.61.0.html
// Tracking issue for const fn: https://github.com/rust-lang/rust/issues/57563
// Constant items in Rust docs: https://doc.rust-lang.org/reference/items/constant-items.html
// Constant evaluation in Rust docs: https://doc.rust-lang.org/reference/const_eval.html
// Const generics in Rust docs: https://doc.rust-lang.org/reference/items/generics.html#const-generics

// A simple constant value
const NEARLY_THE_ANSWER: i32 = 41;

// A constant expression that is evaluated at compile time.
// Note: Not everything is allowed in a constant expression. See
// https://doc.rust-lang.org/reference/const_eval.html#constant-expressions
// for details
const ANSWER: i32 = NEARLY_THE_ANSWER + 1;

// A simple constant string (because of static lifetime elision, we don't need 'static)
const TEXT: &str = "the quick brown fox jumps over the lazy dog!";

// A constant array
const NUMBERS: [i32; 5] = [1, 2, 3, 4, 5];

// Constants in Rust does not need to be basic data types.
// Structs can also be constant.
#[derive(Debug, Clone, Copy)]
struct Customer<'a> {
    name: &'a str,
    age: i32,
}
const CUSTOMER: Customer = Customer {
    name: "John",
    age: 42,
};

// A constant struct.
// Note that constants may refer to the address of other constants.
// The lifetime defaults to 'static (elided).
struct NamedNumbers<'a> {
    name: &'a str,
    numbers: &'a [i32; 5],
}
const NAMED_NUMBERS: NamedNumbers = NamedNumbers {
    name: TEXT,
    numbers: &NUMBERS,
};

// Constants can have destructors
struct WillSayGoodbye<'a>(&'a str);
impl<'a> Drop for WillSayGoodbye<'a> {
    fn drop(&mut self) {
        println!("{}", self.0);
    }
}
// Note that destructor on statics will not run on program/thread exit.
static _GOODBYE_IN_ENGLISH: WillSayGoodbye = WillSayGoodbye("Goodbye");
const GOODBYE_IN_GERMAN: WillSayGoodbye = WillSayGoodbye("Auf Wiedersehen");

// Constants in traits
trait HasNumbers {
    // Note that we have a constant here without a value.
    const NUMBERS: [i32; 5];

    // Constants in traits can have default values
    const LAST_NUMBER: i32 = 5;
}
struct IHaveNumbers {}
impl HasNumbers for IHaveNumbers {
    // This is an *associated constant*, as it is associated with a type.
    const NUMBERS: [i32; 5] = [1, 2, 3, 4, IHaveNumbers::LAST_NUMBER];
}
struct IHaveOtherNumbers {}
impl HasNumbers for IHaveOtherNumbers {
    // Here we override the default value from the trait.
    const LAST_NUMBER: i32 = 6;
    const NUMBERS: [i32; 5] = [1, 2, 3, 4, IHaveOtherNumbers::LAST_NUMBER];
}

// Constants do not need to have a name. They can be unnamed.
// This is useful for running code at compile time.
const _: () = {
    // Code that makes sure that IHaveNumbers implements HasNumbers
    use std::marker::PhantomData;
    struct ImplementsMyTrait<T: HasNumbers>(PhantomData<T>);
    let _ = ImplementsMyTrait(PhantomData::<IHaveNumbers>);
};

// Functions can be constant to be usable in const and static scenarios.
// Note that some limitations apply (see examples below).
const fn five_numbers() -> [i32; 5] {
    let mut numbers = [0i32; 5];

    // Note that for loops can not (yet) be used in constant functions.
    // So we have to go for a while loop.
    let mut i = 0;
    while i < 5 {
        numbers[i] = i as i32 + 1;
        i += 1;
    }

    numbers
}

// Const functions can be generic and can also receive const
// generic parameters.
const fn numbers<const N: usize>() -> [i32; N] {
    let mut numbers = [0i32; N];

    let mut i = 0;
    while i < N {
        numbers[i] = i as i32 + 1;
        i += 1;
    }

    numbers
}

// Slightly more complex samples for concatenation of strings in const fn.
// Inspired by crates like https://docs.rs/const-str/latest/const_str/.

/// Gets total length of all provided strings
const fn len(strs: &[&str]) -> usize {
    let mut result = 0;
    let mut remaining = strs;

    // Note that we cannot use iter() here because
    // it is not a constant expression. Therefore, we
    // have to iterate using slice deconstruction.
    while let [current, tail @ ..] = remaining {
        result += current.len();
        remaining = tail;
    }

    result
}

/// Helper struct for concatenation of strings in const fn.
struct Buf<const N: usize>([u8; N]);

/// Concatenates all provided strings into a single string *at compile time*.
const fn concat<const N: usize>(strs: &[&str]) -> Buf<N> {
    let mut buffer = [0; N];
    let mut position_in_buffer = 0;

    let mut remaining = strs;
    while let [current, tail @ ..] = remaining {
        let x = current.as_bytes();
        let mut i = 0;

        // Note that we cannot use copy_from_slice because mutable
        // references are not allowed in const functions.
        // buffer.copy_from_slice(x);

        // Note that for loop is not (yet) allowed in const functions.
        // We have to use while instead.
        while i < x.len() {
            buffer[position_in_buffer] = x[i];
            position_in_buffer += 1;
            i += 1;
        }

        remaining = tail;
    }

    Buf(buffer)
}

macro_rules! my_concat {
    ($($x: expr),+ $(,)?) => {{
        const STRS: &[&str] = &[$($x),+];
        const LEN: usize = len(STRS);
        const CONCAT_BUF: Buf<LEN> = concat(STRS);
        unsafe { core::str::from_utf8_unchecked(&CONCAT_BUF.0) }
    }}
}

// Brand new (1.61, stable): Trait bounds on generic parameters for
// const functions are now supported.
const fn nth<T: Copy, const N: usize>(items: [T; N], index: usize) -> T {
    items[index]
}

trait Animal {
    fn make_sound<'a>(&self) -> &'a str;
}
struct Cat {}
struct Dog {}
impl Animal for Cat {
    fn make_sound<'a>(&self) -> &'a str {
        "meow"
    }
}
impl Animal for Dog {
    fn make_sound<'a>(&self) -> &'a str {
        "woof"
    }
}

// Brand new (1.61): Arguments and return values for const fns
// now support impl trait.
const fn favorite_animal() -> impl Animal {
    Cat {}
}

// Brand new (1.61): Arguments and return values for const fns
// now support dyn trait.
const fn animal_by_sound<'a>(can_purr: bool) -> &'a dyn Animal {
    match can_purr {
        true => &Cat {},
        false => &Dog {},
    }
}

const fn _can_purr(_animal: &dyn Animal) -> Result<bool, &str> {
    // Note that the following if statement is not possible in const fn
    // because calls to trait methods are not allowed.
    // if (animal.make_sound() == "meow") {
    //     return Ok(true);
    // }

    Err("Sorry, cannot find that out")
}

use std::sync::{Mutex, atomic::AtomicI32};

// Brand new (1.63): Mutex::new and RwLock::new are const functions.
// No need for lazy_static & friends anymore (see also
// https://stackoverflow.com/a/27826181/3548127).
static ARRAY: Mutex<Vec<u8>> = Mutex::new(vec![]);

// Speaking of static: Note that CONSTS are COPYIED everywhere they are referenced.
// If you have a type that is mutable, always use STATIC, not CONST.
// +-- Replace this with static
// v
const CONST_VALUE: AtomicI32 = AtomicI32::new(0);

fn main() {
    // Constants are inlined at compile time wherever they are used.
    println!("The answer is {ANSWER}");
    println!("{TEXT}");
    println!("Numbers: {:?}", NUMBERS);

    // Constants can be declared in any scope, not just global.
    const VERSION: &str = "1.2.3";
    println!("Version: {}", VERSION);

    println!("{TEXT:p}\n{:p}", NAMED_NUMBERS.name);
    println!("{}", NAMED_NUMBERS.numbers.iter().sum::<i32>());

    println!(
        "Customer {} is of age {} ({:?})",
        CUSTOMER.name, CUSTOMER.age, CUSTOMER
    );

    // Note that if you modify a const item, a new temporary
    // item is created. the original const item is not modified.
    CUSTOMER.age += 1;
    
    {
        let _goodbye_sayer = GOODBYE_IN_GERMAN;
        // Destructor will run at appropriate point when const is used. Therefore,
        // this code will print "Auf Wiedersehen" on stdout when var goes out of scope.
    }

    println!("{:?}", IHaveNumbers::NUMBERS);
    println!("{:?}", IHaveOtherNumbers::NUMBERS);

    const FIVE_NUMBERS: [i32; 5] = five_numbers();
    println!("{:?}", FIVE_NUMBERS);

    const TEN_NUMBERS: [i32; 10] = numbers();
    println!("{:?}", TEN_NUMBERS);

    // Const functions can also be called at runtime.
    // They are not limited to constants or static variables.
    let five_numbers = five_numbers();
    println!("{:?}", five_numbers);

    // Define constant strings and concat them at compile time
    const STRS: &[&str] = &["Hello", " ", "World!"];
    const LEN: usize = len(STRS);
    const CONCAT_BUF: Buf<LEN> = concat(STRS);
    const GREETING: &str = unsafe { core::str::from_utf8_unchecked(&CONCAT_BUF.0) };
    println!("{}", GREETING);

    // Const functions can be called at runtime, too
    let strs: &[&str] = &["Hello", " ", "World!"];
    let concat_buf: Buf<LEN> = concat(strs);
    let greeting: &str = unsafe { core::str::from_utf8_unchecked(&concat_buf.0) };
    println!("{}", greeting);

    const CAT: &str = "😺";
    const MOUSE: &str = "🐭";

    // Rust's built-in concat! macro only accepts literals.
    // We cannot concat two string constants.
    //const GREETING3: &str = concat!(CAT, "💗", MOUSE);

    // Our macro makes it easy to call the concat const fns
    // we defined before. It is not limited to literals.
    const GREETING2: &str = my_concat!(CAT, "💗", MOUSE);
    println!("{}", GREETING2);

    const CUSTOMERS: [Customer; 2] = [
        Customer {
            name: "John",
            age: 30,
        },
        Customer {
            name: "Jane",
            age: 25,
        },
    ];
    const NTH_CUSTOMER: Customer = nth(CUSTOMERS, 1);
    println!("{:?}", NTH_CUSTOMER);

    const FAVORITE_ANIMAL: &dyn Animal = &favorite_animal();
    println!("My favorite animal makes {}", FAVORITE_ANIMAL.make_sound());

    const PURRING_ANIMAL: &dyn Animal = animal_by_sound(true);
    println!("Animals that can purr say {}", PURRING_ANIMAL.make_sound());

    {
        let mut arr = ARRAY.lock().unwrap();
        for _ in 0..10 {
            arr.push(1);
        }
    }
    println!("Called push {} times", ARRAY.lock().unwrap().len());

    CONST_VALUE.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    CONST_VALUE.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    println!("Result should be 2, it is {:?}", CONST_VALUE.load(std::sync::atomic::Ordering::Relaxed));

    println!("Program ends now");
}
