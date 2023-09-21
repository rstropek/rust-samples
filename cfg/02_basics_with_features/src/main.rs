fn main() {
    // Note that we are now checking cargo features
    #[cfg(not(any(feature =  "traditional", feature =  "vegetarian", feature =  "vegan")))]
    {
        println!("You haven't specified whether the recipe should be traditional, vegetarian, or vegan.");
        panic!();
    }

    #[cfg(any(
        all(feature =  "traditional", feature =  "vegetarian"), 
        all(feature =  "traditional", feature =  "vegan"), 
        all(feature =  "vegetarian", feature =  "vegan")))]
    {
        println!("You've specified more than one recipe type. Please specify only one.");
        panic!();
    }

    print_ingredients();
    print_preparation();
}

fn print_ingredients() {
    println!("Ingredients for Caesar Salad:");
    
    println!("- Romaine lettuce");
    println!("- Croutons");
    println!("- Lemon juice");
    println!("- Olive oil");
    println!("- Salt");
    println!("- Black pepper");

    // We combine config predicates again
    #[cfg(any(feature =  "traditional", feature =  "vegetarian"))]
    println!("- Parmesan cheese");
    #[cfg(any(feature =  "vegan"))]
    println!("- Vegan Parmesan cheese substitute");

    #[cfg(feature =  "traditional")]
    {
        println!("- Anchovies");
        println!("- Egg yolks");
    }

    #[cfg(feature =  "vegetarian")]
    {
        println!("- Capers");
        println!("- Egg yolks");
    }

    #[cfg(feature =  "vegan")]
    {
        println!("- Capers");
        println!("- Vegan mayonnaise");
    }
}

fn print_preparation() {
    println!("\nPreparation:");

    println!("1. Wash and tear the romaine lettuce into bite-size pieces.");
    println!("2. In a bowl, combine lemon juice, olive oil, salt, and pepper.");
    
    match (cfg!(feature =  "traditional"), cfg!(feature =  "vegetarian"), cfg!(feature =  "vegan")) {
        (true, false, false) => println!("3. Add grated Parmesan cheese, anchovies, and egg yolks to the bowl."),
        (false, true, false) => println!("3. Add grated Parmesan cheese, capers, and egg yolks to the bowl."),
        (false, false, true) => println!("3. Add vegan Parmesan cheese substitute, capers, and vegan mayonnaise to the bowl."),
        _ => panic!("More than one recipe type? This should never happen!"),
    };

    println!("4. Mix well until the ingredients are well-combined.");
    println!("5. Add the croutons and romaine lettuce to the bowl.");
    println!("6. Toss until the lettuce is well-coated with the dressing.");
    println!("7. Serve immediately and enjoy your Caesar Salad!");
}
