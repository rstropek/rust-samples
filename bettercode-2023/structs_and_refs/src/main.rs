#![allow(dead_code, unused_variables)]

// Structs hold multiple, related values
struct Hero {
    // You name each piece of data and give it a type
    name: String,
    superpowers: String,
    can_fly: bool,
}

// You can also define structs that don't have named fields.
// They are called tuple structs.
struct HeroTuple(String, String, bool);

// You can also define structs that don't have any fields.
// They are called unit structs (because of the unit type `()`).
struct HeroUnit;

fn main() {
    let answer = 42; // Ignore that for now

    // This is how you instantiate a struct. Instances
    // (the entire instance, not selected fields)
    // can be immutable or mutable. 
    let mut superman = Hero {
        name: String::from("Superman"),
        superpowers: String::from("flight"),
        can_fly: true,
    };

    // You can access the fields of a struct with dot notation
    superman.superpowers.push_str(", super strength");
    
    // You can use field init shortcuts to make code more readable
    let name = String::from("Batman");
    let superpowers = String::from("martial arts");
    let can_fly = false;
    let batman = Hero {
        name, // Works because parameters have same name as struct fields
        superpowers,
        can_fly,
    };

    // Struct update syntax lets you create a new instance based on another.
    let catwoman = Hero {
        name: String::from("Catwoman"),
        ..batman // Use the rest of the fields from batman
    };
    
    // Note that data IS MOVED from batman to catwoman. So the
    // following line does NOT compile.
    //println!("{}", batman.superpowers);

    // You can also create tuple structs.
    let wonderwoman = HeroTuple(String::from("Wonderwoman"), String::from("super strength"), true);

    // You can also create unit structs.
    let empty_hero = HeroUnit;

    // Structs live on the stack, but their data can live on the heap.
    // Let's analyze the memory layout of our structs.
    println!("&answer:\t\t{:p}", &answer);
    println!("&superman:\t\t{:p}\t(4 bytes after answer)", &superman);
    println!("&superman.name:\t\t{:p}", &superman.name);
    println!("&superman.superpowers:\t{:p}\t(24 bytes after superman)", &superman.superpowers);
    println!("\t\t\t\t\t(8 bytes pointer to buffer, 8 bytes capacity, 8 bytes length)");
    println!("&superman.can_fly:\t{:p}\t(24 bytes after superpowers)", &superman.can_fly);
    println!("superman.name.as_str():\t{:p}\t(on the heap)", superman.name.as_str());

    // Excursus: Do you want to learn more about Rust strings?
    // Good read: https://blog.thoughtram.io/string-vs-str-in-rust/

    // Use Box if you want to allocate a struct on the heap.
    let spiderman = Box::new(Hero {
        name: String::from("Spiderman"),
        superpowers: String::from("spider sense"),
        can_fly: false,
    });
    println!("&spiderman:\t\t{:p}\t(on the heap)", &spiderman);

    // If you put immutable borrows in a vector, nothing happens to the instances.
    let heroes = vec![&superman, &catwoman];
    let superman_borrow = heroes[0];
    println!("\nBorrows in Vector:");
    println!("&superman:\t\t{:p}", superman_borrow);
    println!("superman.name.as_str():\t{:p}", superman_borrow.name.as_str());

    // If you put a hero into e.g. a vector, it will be moved to the heap.
    let heroes = vec![superman, catwoman, *spiderman];
    let superman = &heroes[0];
    println!("\nMoved in Vector:");
    println!("&superman:\t\t{:p}\t(now on the heap)", superman);
    println!("superman.name.as_str():\t{:p}\t(did NOT move)", superman.name.as_str());
    
}

// Structs do NOT...
// ... have (multiple) inheritance
// ... have default values
// ... have null
// ... have properties
// ... have constructors
// ... have destructors
// ... have events
// ... have method overloading
// ... have operator overloading
