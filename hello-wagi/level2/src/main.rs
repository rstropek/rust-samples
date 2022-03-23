// Based on https://github.com/deislabs/env_wagi/blob/main/src/main.rs

fn main() {
    println!("Content-Type: text/plain");

    println!("Status: 200");
    println!("X-Foo-Header: Bar");
    println!(); // empty line

    println!("### Arguments ###");
    std::env::args().for_each(|a| println!("arg: {}", a));

    println!("\n### Env Vars ###");
    std::env::vars().for_each(|v| {
        println!("{} = {}", v.0, v.1);
    });

    println!("\n### STDIN ###");
    std::io::copy(&mut std::io::stdin(), &mut std::io::stdout()).unwrap();
}
