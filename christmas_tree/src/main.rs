fn main() {
    for leafs in [(1, 3), (2, 5), (4, 9)].iter() {
        for stars in (leafs.0 - 1)..leafs.1 {
            print_line(stars);
        }
    }

    for _ in 0..2 {
        print_line(2)
    }
}

fn print_line(stars: i32) {
    for _ in 0..(30 - stars * 2) {
        print!(" ");
    }
    for _ in 0..(stars * 4 + 1) {
        print!("*");
    }
    print!("\n");
}
