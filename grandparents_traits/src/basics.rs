pub struct ConsultingWork {
    #[allow(dead_code)]
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
    fn total(&self) -> f32;
}

impl Billable for ConsultingWork {
    fn total(&self) -> f32 {
        self.hours * self.rate
    }
}

pub fn print_total(b: &impl Billable) {
    println!("Total: ${:.2}", b.total());
}
