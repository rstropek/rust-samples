//! Object-safe traits can be used behind `dyn`; some useful traits are generic-only.

#[derive(Clone, Debug)]
struct WorkTemplate {
    name: &'static str,
}

trait Duplicate {
    fn duplicate(&self) -> Self;
}

impl Duplicate for WorkTemplate {
    fn duplicate(&self) -> Self {
        self.clone()
    }
}

trait Runnable {
    fn run(&self) -> &'static str;
}

impl Runnable for WorkTemplate {
    fn run(&self) -> &'static str {
        self.name
    }
}

fn duplicate_static<T: Duplicate>(item: &T) -> T {
    item.duplicate()
}

fn run_dynamic(item: &dyn Runnable) {
    println!("object-safe dyn Trait: {}", item.run());
}

pub fn run() {
    println!("\n== Object safety ==");

    let template = WorkTemplate {
        name: "repeatable project setup",
    };
    let duplicated = duplicate_static(&template);
    println!("generic trait returning Self: {duplicated:?}");

    run_dynamic(&template);
    println!("Duplicate cannot be used as dyn Duplicate because duplicate returns Self");
}
