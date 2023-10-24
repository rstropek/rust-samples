#![allow(dead_code, unused_variables)]

struct Rectangle { width: f32, height: f32 }

// Define custom trait
trait HasArea {
    fn area(&self) -> f32;
}

// Implement trait for custom type
impl HasArea for Rectangle {
    fn area(&self) -> f32 {
        self.width * self.height
    }
}

// Auto-generate trait implementation (system trait)
#[derive(Debug)]
struct Circle { radius: f32 }

impl HasArea for  Circle {
    fn area(&self) -> f32 {
        std::f32::consts::PI * self.radius * self.radius
    }
}

// Explicitely implement system trait
impl Default for Circle {
    fn default() -> Circle {
        Circle { radius: 1.0 }
    }
}

use std::fmt::{Display, Formatter, Result};
impl Display for Circle {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Circle with radius {} and area {}", self.radius, self.area())
    }
}

struct Arc { radius: f32, angle_radiant: f32 }

impl HasArea for Arc {
    fn area(&self) -> f32 {
        self.radius * self.radius * self.angle_radiant / 2.0
    }
}

// Implement generic system trait
impl From<&Circle> for Arc {
    fn from(circle: &Circle) -> Arc {
        Arc { radius: circle.radius, angle_radiant: 2.0 * std::f32::consts::PI }
    }
}

// Copy and clone traits change how values are passed around
#[derive(Clone, Copy, Debug)]
struct Point { x: f32, y: f32 }

fn main() {
    let rectangle = Rectangle { width: 3.0, height: 4.0 };
    let circle = &Circle { radius: 5.0 };

    println!("Rectangle area: {}", rectangle.area());
    println!("Circle: {:?}", circle); // Uses Debug trait
    println!("Circle: {}", circle); // Uses Display trait

    // Using the From and Into traits
    let arc: Arc = circle.into();
    let arc = Arc::from(circle);
    println!("Arc (from circle) has area {}", arc.area());

    // Applying copy and clone traits
    let p1 = Point { x: 1.0, y: 2.0 };
    let p2 = p1;
    println!("p1: {:?}, p2: {:?}", p1, p2);

    let p3 = p1.clone();
    println!("p1: {:?}, p3: {:?}", p1, p3);
}
