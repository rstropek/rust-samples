//! `impl Trait` keeps a concrete return type hidden; `dyn Trait` enables runtime dispatch.

pub trait Billable {
    fn label(&self) -> &str;
    fn total(&self) -> f32;
}

struct ConsultingWork {
    what: String,
    hours: f32,
    rate: f32,
}

impl Billable for ConsultingWork {
    fn label(&self) -> &str {
        &self.what
    }

    fn total(&self) -> f32 {
        self.hours * self.rate
    }
}

impl Billable for f32 {
    fn label(&self) -> &str {
        "flat amount"
    }

    fn total(&self) -> f32 {
        *self
    }
}

fn create_billable(what: &str, hours: f32, rate: f32) -> impl Billable {
    ConsultingWork {
        what: what.to_string(),
        hours,
        rate,
    }
}

fn create_billable_dyn(what: Option<&str>, hours: f32, rate: f32) -> Box<dyn Billable> {
    if let Some(what) = what {
        Box::new(ConsultingWork {
            what: what.to_string(),
            hours,
            rate,
        })
    } else {
        Box::new(hours * rate)
    }
}

fn create_billables() -> Vec<Box<dyn Billable>> {
    vec![
        Box::new(ConsultingWork {
            what: "project work".to_string(),
            hours: 10.0,
            rate: 100.0,
        }),
        Box::new(75.0),
    ]
}

fn print_static_dispatch(item: &impl Billable) {
    println!("static dispatch: {} = ${:.2}", item.label(), item.total());
}

fn print_dynamic_dispatch(item: &dyn Billable) {
    println!("dynamic dispatch: {} = ${:.2}", item.label(), item.total());
}

pub fn run() {
    println!("\n== Static dispatch and dynamic dispatch ==");

    let concrete_billable = create_billable("AI consulting", 10.0, 100.0);
    print_static_dispatch(&concrete_billable);

    let maybe_concrete = create_billable_dyn(None, 10.0, 100.0);
    print_dynamic_dispatch(maybe_concrete.as_ref());

    for item in create_billables() {
        print_dynamic_dispatch(item.as_ref());
    }
}
