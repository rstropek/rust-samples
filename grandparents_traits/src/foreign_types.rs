use std::ops::Deref;

pub trait Billable {
    fn total(&self) -> f32;
}

impl Billable for f32 {
    fn total(&self) -> f32 {
        *self
    }
}

pub fn print_total(b: &impl Billable) {
    println!("Total: ${:.2}", b.total());
}

// The following code does not work because f32 is a foreign type
// and Default is also not included in the current crate.
// impl Default for f32 {
//     fn default() -> Self {
//         -1.0
//     }
// }

pub struct MyF32(pub f32); // Newtype pattern

impl Deref for MyF32 {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for MyF32 {
    fn default() -> Self {
        MyF32(-1.0)
    }
}
