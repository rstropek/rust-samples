#[derive(Debug)]
struct Vector2d {
    x: f32,
    y: f32,
}

impl Drop for Vector2d {
    fn drop(&mut self) {
        println!("Dropping {:?}", self);
    }
}

fn find_longest_vector<'a>(v1: &'a Vector2d, v2: &'a Vector2d) -> &'a Vector2d {
    if (v1.x * v1.y).sqrt() > (v2.x + v2.y).sqrt() {
        v1
    } else {
        v2
    }
}

fn main() {
    let v1 = &Vector2d { x: 1.0, y: 1.0 };
    let res: &Vector2d;
    let v2 = &Vector2d { x: 2.0, y: 2.0 };
    {
        //let v2 = &Vector2d { x: 2.0, y: 2.0 }; // Would not work
        res = find_longest_vector(v1, v2);
    }

    println!("{:?}", res);
}
