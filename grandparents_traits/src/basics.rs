//! Basic trait implementation with one required method and one default method.

pub struct ConsultingWork {
    what: String,
    hours: f32,
    rate: f32,
}

impl ConsultingWork {
    pub fn new(what: &str, hours: f32, rate: f32) -> ConsultingWork {
        ConsultingWork {
            what: what.to_string(),
            hours,
            rate,
        }
    }
}

pub trait Billable {
    fn description(&self) -> &str;
    fn total(&self) -> f32;

    fn formatted_total(&self) -> String {
        format!("${:.2}", self.total())
    }
}

impl Billable for ConsultingWork {
    fn description(&self) -> &str {
        &self.what
    }

    fn total(&self) -> f32 {
        self.hours * self.rate
    }
}

fn print_total(item: &impl Billable) {
    println!("{}: {}", item.description(), item.formatted_total());
}

pub fn run() {
    println!("\n== Basic trait implementation and default methods ==");

    let work = ConsultingWork::new("AI consulting", 10.0, 100.0);
    print_total(&work);
}
