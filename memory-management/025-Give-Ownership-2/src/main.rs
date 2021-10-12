#[derive(Debug)]
struct Vector2d {
    x: f64,
    y: f64,
}

impl Drop for Vector2d {
    fn drop(&mut self) {
        println!("Dropping {:?}", self);
    }
}

fn main() {
    let v = Vector2d { x: 1.0, y: 2.0 };
    consume_vector(v);
    println!("After consume_vector");
    let _v = produce_vector();
    println!("After produce_vector");
}

fn consume_vector(_v: Vector2d) {
    println!("In consume_vector");
}

fn produce_vector() -> Vector2d {
    Vector2d { x: 3.0, y: 4.0 }
}
