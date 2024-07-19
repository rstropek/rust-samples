pub trait Billable {
    fn total(&self) -> f32;
}

pub trait Pointworthy {
    fn points(&self) -> i32;
}

impl<T: Billable> Pointworthy for T {
    fn points(&self) -> i32 {
        (self.total() / 10.0) as i32
    }
}

#[allow(dead_code)]
pub enum ShirtSize {
    Small,
    Medium,
    Large,
    ExtraLarge,
    Epic,
}

pub struct CodingWork {
    size: ShirtSize,
}

impl CodingWork {
    pub fn new(size: ShirtSize) -> CodingWork {
        CodingWork { size }
    }
}

// Implement Billable for all slices of Billables by adding up totals
impl<T: Billable, const C: usize> Billable for [T; C] {
    fn total(&self) -> f32 {
        self.iter().map(|b| b.total()).sum()
    }
}

impl Billable for CodingWork {
    fn total(&self) -> f32 {
        match self.size {
            ShirtSize::Small => 100.0,
            ShirtSize::Medium => 200.0,
            ShirtSize::Large => 1000.0,
            ShirtSize::ExtraLarge => 5000.0,
            ShirtSize::Epic => 100_000.0,
        }
    }
}

pub fn print_points(b: &impl Pointworthy) {
    println!("Points: {}", b.points());
}
