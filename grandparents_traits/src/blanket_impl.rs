//! Blanket implementations let one trait implementation cover every matching type.

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

fn print_points(label: &str, item: &impl Pointworthy) {
    println!("{label}: {} points", item.points());
}

pub fn run() {
    println!("\n== Blanket implementations ==");

    let single_item = CodingWork::new(ShirtSize::Epic);
    print_points("single billable item", &single_item);

    let sprint = [
        CodingWork::new(ShirtSize::Small),
        CodingWork::new(ShirtSize::Medium),
        CodingWork::new(ShirtSize::Large),
        CodingWork::new(ShirtSize::ExtraLarge),
    ];
    print_points("array of billable items", &sprint);
}
