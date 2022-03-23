use std::{f32::consts, fmt::Display, fs, ops::Add};

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

impl Add<Point> for Point {
    type Output = Self;

    fn add(self, other: Point) -> Self {
        Point { x: self.x + other.x, y: self.y + other.y }
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ x: {}, y: {} }}", self.x, self.y)
    }
}

struct Rect {
    left_upper: Point,
    right_lower: Point,
}

impl Rect {
    fn width(&self) -> f32 {
        self.right_lower.x - self.left_upper.x
    }

    fn height(&self) -> f32 {
        self.right_lower.y - self.left_upper.y
    }

    fn enlarge(&mut self, factor: f32) {
        self.right_lower.x = self.left_upper.x + self.width() * factor;
        self.right_lower.y = self.left_upper.y + self.height() * factor;
    }
}

trait Area {
    fn area(&self) -> f32;
}

impl Area for Rect {
    fn area(&self) -> f32 {
        self.width() * self.height()
    }
}

impl Area for Circle {
    fn area(&self) -> f32 {
        consts::PI * self.radius * self.radius
    }
}

struct Circle {
    center: Point,
    radius: f32,
}

impl Circle {
    fn unit_circle() -> Self {
        Circle { center: Point { x: 0f32, y: 0f32 }, radius: 1f32 }
    }
}

struct Raise {
    current_pot: f32,
    raise: f32,
}

enum PokerMove {
    Fold,
    Call(f32),
    Raise(Raise)
}

fn main() {
    // Create an instance of Point on the stack
    let p1 = Point { x: 20f32, y: 20f32 };

    // Create another Point and take some values from p1
    let p2 = Point { x: 21f32, ..p1 };

    // By default, variables are immutable. Let's create a mutable rect.
    let mut r = Rect {
        left_upper: p1,
        right_lower: p2,
    };
    r.right_lower.y = 40f32;

    // Create a Circle on the heap
    let mut c = Box::new(Circle {
        center: Point { x: 10f32, y: 10f32 },
        radius: 42f32,
    });
    c.radius = 10f32;

    r.enlarge(2.0f32);
    println!("The area is {}", c.area());

    let unit = Circle::unit_circle();
    println!("The area is {}", unit.area());

    let shape_with_area: &dyn Area = &r;
    println!("The area is {}", shape_with_area.area());

    let mut shapes = Vec::<&dyn Area>::new();
    shapes.push(&r);
    shapes.push(c.as_ref());
    for s in shapes {
        println!("The area is {}", s.area());
    }

    let origin = Point { x: 20f32, y: 20f32 };
    println!("{}", origin);
    let other = origin + Point { x: 20f32, y: 20f32 };
    println!("{}", other);

    let points = vec![
        Point { x: 20f32, y: 20f32 },
        Point { x: 21f32, y: 21f32 }
    ];
    println!("{:?}", &points);

    let poker_move = PokerMove::Raise(Raise { current_pot: 42f32, raise: 2f32 });
    match poker_move {
        PokerMove::Call(x) => println!("Player calls at pot size {}", x),
        PokerMove::Raise(x) => println!("Player raises, pot size now {}", x.current_pot + x.raise),
        _ => println!("Player folds"),
    };

    let moves = vec![
        PokerMove::Fold,
        PokerMove::Call(2f32),
        PokerMove::Call(2f32),
        PokerMove::Raise(Raise { current_pot: 42f32, raise: 2f32 })
    ];
    let poker_move: Option<&PokerMove> = moves.iter().nth(4);
    match poker_move {
        Some(_) => println!("Found a move"),
        None => println!("This move does not exist"),
    };

    let contents: Result<String, std::io::Error> = fs::read_to_string("/home/rainer/github/rust-samples/shapes/src/main.rs");
    match contents {
        Result::Ok(data) => println!("{}", data),
        Result::Err(err) => println!("Error: {}", err),
    };
}
