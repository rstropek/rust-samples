use std::{iter::Sum, ops::Add};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ConsultingWork {
    pub what: String,
    pub hours: f32,
    pub rate: f32,
}

impl Add for ConsultingWork {
    type Output = ConsultingWork;

    fn add(self, other: ConsultingWork) -> ConsultingWork {
        if self != Default::default() {
            ConsultingWork {
                what: format!("{}, {}", self.what, other.what),
                hours: self.hours + other.hours,
                rate: self.rate + other.rate,
            }
        } else {
            other
        }
    }
}

impl Sum for ConsultingWork {
    fn sum<I: Iterator<Item = ConsultingWork>>(iter: I) -> ConsultingWork {
        iter.fold(ConsultingWork::default(), |a, b| a + b)
    }
}

pub fn print_work(work: &ConsultingWork) {
    println!("Did {} for {} hours at ${:.2}/hour", work.what, work.hours, work.rate);
}