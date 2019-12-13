const RADIUS: f32 = 20.0;

fn main() {
    print_line(RADIUS);
}

fn print_line(l: f32) {
    if l < 1.0 {
        return;
    }

    let b: f32 = l as f32;
    let a: f32 = (b / (b / RADIUS).asin().tan()).round();

    print!("{}**{}**\n", " ".repeat((RADIUS - a) as usize * 2), " ".repeat(a as usize * 4));
    print_line(l - 1.0);
    print!("{}**{}**\n", " ".repeat((RADIUS - a) as usize * 2), " ".repeat(a as usize * 4));
}
