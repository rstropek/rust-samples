pub trait Billable {
    fn total(&self) -> f32;
}

pub struct ConsultingWork {
    #[allow(dead_code)]
    what: String,
    hours: f32,
    rate: f32,
}

impl Billable for ConsultingWork {
    fn total(&self) -> f32 {
        self.hours * self.rate
    }
}

impl Billable for f32 {
    fn total(&self) -> f32 {
        *self
    }
}

pub fn create_billable(what: &str, hours: f32, rate: f32) -> impl Billable {
    ConsultingWork {
        what: what.to_string(),
        hours,
        rate,
    }
}


pub fn create_billable2(what: Option<&str>, hours: f32, rate: f32) -> Box<dyn Billable> {
    if let Some(what) = what {
        Box::new(ConsultingWork {
            what: what.to_string(),
            hours,
            rate: rate,
        })
    } else {
        Box::new(hours * rate)
    }
}

pub fn create_billables() -> Vec<Box<dyn Billable>> {
    vec![
        Box::new(ConsultingWork {
            what: "Project 1".to_string(),
            hours: 10.0,
            rate: 100.0,
        }),
        Box::new(75.0),
    ]
}

pub fn print_total(b: &dyn Billable) {
    println!("Total: ${:.2}", b.total());
}