//! Standard library traits make custom types work with familiar operators and adapters.

use std::{iter::Sum, ops::Add};

#[derive(Debug, Clone, Default)]
pub struct InvoiceTotal {
    description: String,
    amount: f32,
}

impl InvoiceTotal {
    fn new(description: &str, amount: f32) -> Self {
        InvoiceTotal {
            description: description.to_string(),
            amount,
        }
    }
}

impl Add for InvoiceTotal {
    type Output = InvoiceTotal;

    fn add(self, other: InvoiceTotal) -> InvoiceTotal {
        match (self.description.is_empty(), other.description.is_empty()) {
            (true, _) => other,
            (_, true) => self,
            _ => InvoiceTotal {
                description: format!("{}, {}", self.description, other.description),
                amount: self.amount + other.amount,
            },
        }
    }
}

impl Sum for InvoiceTotal {
    fn sum<I: Iterator<Item = InvoiceTotal>>(iter: I) -> InvoiceTotal {
        iter.fold(InvoiceTotal::default(), |total, item| total + item)
    }
}

fn print_invoice_total(label: &str, total: &InvoiceTotal) {
    println!("{label}: {} = ${:.2}", total.description, total.amount);
}

pub fn run() {
    println!("\n== Standard library traits: Add and Sum ==");

    let consulting = InvoiceTotal::new("AI consulting", 1_000.0);
    let coding = InvoiceTotal::new("Rust coding", 1_500.0);
    print_invoice_total("operator overloading with Add", &(consulting + coding));

    let work_items = [
        InvoiceTotal::new("discovery", 250.0),
        InvoiceTotal::new("implementation", 1_500.0),
        InvoiceTotal::new("handover", 250.0),
    ];
    print_invoice_total(
        "iterator aggregation with Sum",
        &work_items.into_iter().sum(),
    );
}
