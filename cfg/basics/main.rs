fn main() {
    // Note that you can combine configuration predicates with all, any, and not.
    #[cfg(not(any(traditional, vegetarian, vegan)))]
    {
        println!("You haven't specified whether the recipe should be traditional, vegetarian, or vegan.");
        panic!();
    }

    #[cfg(any(all(traditional, vegetarian), all(traditional, vegan), all(vegetarian, vegan)))]
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
    #[cfg(any(traditional, vegetarian))]
    println!("- Parmesan cheese");
    #[cfg(any(vegan))]
    println!("- Vegan Parmesan cheese substitute");

    // Note that the cfg attribute can be applied to any item, not just
    // a line of code. In this case, we're using it to conditionally
    // include a block of code. You could also conditionally define
    // a module, struct, trait, function, etc.
    #[cfg(traditional)]
    {
        println!("- Anchovies");
        println!("- Egg yolks");
    }

    #[cfg(vegetarian)]
    {
        println!("- Capers");
        println!("- Egg yolks");
    }

    #[cfg(vegan)]
    {
        println!("- Capers");
        println!("- Vegan mayonnaise");
    }
}

fn print_preparation() {
    println!("\nPreparation:");

    println!("1. Wash and tear the romaine lettuce into bite-size pieces.");
    println!("2. In a bowl, combine lemon juice, olive oil, salt, and pepper.");
    
    // Instead of the config attribute, we can also use the cfg! macro. Note that it
    // does NOT remove any code. The macro only evaluates to true or false at compile time.
    // The following code might not be the best option in real-life, but it's a good
    // example of how to use the cfg! macro.
    match (cfg!(traditional), cfg!(vegetarian), cfg!(vegan)) {
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
