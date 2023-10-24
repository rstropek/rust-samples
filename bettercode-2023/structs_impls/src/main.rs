#![allow(dead_code, unused_variables)]

struct Rectangle {
    width: f32,
    height: f32,
}

// Associate methods to the struct
impl Rectangle {
    // Method that is not associated with a specific instance of the struct.
    // Note that "new" is just a name, it could be anything. No constructors in Rust.
    fn new() -> Self {
        Rectangle::from_width_height(0f32, 0f32)
    }

    // Note Self is the type of the struct to which the methods are associated.
    fn from_width_height(width: f32, height: f32) -> Self {
        Rectangle { width, height }
    }

    // Methods that are associated with a specific instance of the struct.
    // It also has a return type. Note immutable borrow.
    fn area(&self) -> f32 {
        self.width * self.height
    }
    
    fn diagonal_length(&self) -> f32 {
        (self.width.powi(2) + self.height.powi(2)).sqrt()
    }

    fn with_width(&self, width: f32) -> Self {
        Rectangle {
            width,
            height: self.height,
        }
    }
    
    fn with_height(&self, height: f32) -> Self {
        Rectangle {
            width: self.width,
            height,
        }
    }
    
    // Note mutable borrow.
    fn set_width(&mut self, width: f32) {
        self.width = width;
    }

    fn set_height(&mut self, height: f32) {
        self.height = height;
    }
}

fn main() {
    // Call a static function
    let rect = Rectangle::new();
    let rect = Rectangle::from_width_height(10f32, 20f32);

    // Builder pattern (call a method that returns a new instance of the struct)
    let rect = Rectangle::new().with_width(10f32).with_height(20f32);

    // Call a method
    println!("Area: {}, diagonal: {}", rect.area(), rect.diagonal_length());

    // Call a method that mutates the struct
    let mut rect = Rectangle::new();
    rect.set_width(10f32);
    rect.set_height(20f32);

    // Local method inside main
    fn get_bounding_rect(rect1: &Rectangle, rect2: &Rectangle) -> Rectangle {
        let width = rect1.width.max(rect2.width);
        let height = rect1.height.max(rect2.height);
        Rectangle::from_width_height(width, height)
    }

    // Call a local method
    let rect1 = Rectangle::from_width_height(10f32, 20f32);
    let rect2 = Rectangle::from_width_height(20f32, 10f32);
    let bounding_rect = get_bounding_rect(&rect1, &rect2);
    println!("Bounding rect width = {}, height = {}", bounding_rect.width, bounding_rect.height);
}
