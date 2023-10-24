use std::fmt::{Display, Formatter};

use chrono::{Utc, Datelike};

trait Animal {
    fn is_mammal(&self) -> bool;
    fn get_species(&self) -> String;
    fn get_sound(&self) -> String;
    fn number_of_legs(&self) -> i32;
}

struct Cat;

struct Spider;

impl Animal for Cat {
    fn is_mammal(&self) -> bool {
        true
    }

    fn get_species(&self) -> String {
        "Cat".to_string()
    }

    fn get_sound(&self) -> String {
        "Meow".to_string()
    }

    fn number_of_legs(&self) -> i32 {
        4
    }
}

impl Display for Cat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_species())
    }
}

impl Animal for Spider {
    fn is_mammal(&self) -> bool {
        false
    }

    fn get_species(&self) -> String {
        "Spider".to_string()
    }

    fn get_sound(&self) -> String {
        "Hiss".to_string()
    }

    fn number_of_legs(&self) -> i32 {
        8
    }
}

fn get_a_cat() -> impl Animal {
    Cat { }
}

fn get_an_animal() -> Box<dyn Animal> {
    if Utc::now().naive_utc().day() % 2 == 0 { Box::new(Spider {}) } else { Box::new(Cat {}) }
}

fn do_something_with_animal(animal: &impl Animal) {
    println!("{}", animal.get_species());
}

fn display_animal<T: Animal + Display>(animal: &T) {
    println!("{}", animal);
}

fn main() {
    let animals: Vec<Box<dyn Animal>> = vec![
        Box::new(Cat {}),
        Box::new(Spider {}),
    ];

    for animal in animals {
        println!("{}", animal.get_species());
    }

    let cat = get_a_cat();
    do_something_with_animal(&cat);
    do_something_with_animal(&Cat {});
    do_something_with_animal(&Spider {});

    let animal = get_an_animal();
    println!("{}", animal.get_species());

    display_animal(&Cat {});
    // display_animal(&Spider {}); // -> doesn't work
}