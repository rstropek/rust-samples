//! Trait bounds, supertraits, fully qualified syntax, and extension traits.

use std::fmt::{Debug, Display, Formatter, Result};

trait Billable {
    fn total(&self) -> f32;
}

trait PrintableBillable: Billable + Display {}

impl<T> PrintableBillable for T where T: Billable + Display {}

#[derive(Debug)]
struct SupportPlan {
    name: String,
    monthly_fee: f32,
}

impl SupportPlan {
    fn new(name: &str, monthly_fee: f32) -> Self {
        SupportPlan {
            name: name.to_string(),
            monthly_fee,
        }
    }
}

impl Billable for SupportPlan {
    fn total(&self) -> f32 {
        self.monthly_fee
    }
}

impl Display for SupportPlan {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        write!(formatter, "{}", self.name)
    }
}

trait CustomerFacingLabel {
    fn label(&self) -> String;
}

trait InternalLabel {
    fn label(&self) -> String;
}

impl CustomerFacingLabel for SupportPlan {
    fn label(&self) -> String {
        format!("{} plan", self.name)
    }
}

impl InternalLabel for SupportPlan {
    fn label(&self) -> String {
        format!("sku:{}", self.name.to_lowercase().replace(' ', "-"))
    }
}

trait CurrencyFormatting {
    fn usd(self) -> String;
}

impl CurrencyFormatting for f32 {
    fn usd(self) -> String {
        format!("${self:.2}")
    }
}

fn print_summary<T>(item: &T)
where
    T: PrintableBillable + Debug,
{
    println!(
        "where clause and supertrait: {item} = {}",
        item.total().usd()
    );
    println!("Debug is also available because of the bound: {item:?}");
}

pub fn run() {
    println!("\n== Bounds, supertraits, and extension traits ==");

    let plan = SupportPlan::new("support retainer", 300.0);
    print_summary(&plan);

    println!(
        "fully qualified syntax: {}",
        <SupportPlan as CustomerFacingLabel>::label(&plan)
    );
    println!(
        "fully qualified syntax: {}",
        <SupportPlan as InternalLabel>::label(&plan)
    );
    println!("extension trait on f32: {}", plan.total().usd());
}
