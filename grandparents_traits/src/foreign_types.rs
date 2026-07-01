//! Local traits can be implemented for foreign types. Foreign traits require a local type.

pub trait Billable {
    fn total(&self) -> f32;
}

impl Billable for f32 {
    fn total(&self) -> f32 {
        *self
    }
}

#[derive(Debug, PartialEq)]
struct FlatFee(pub f32);

// `Default` is a foreign trait, so we need the local `FlatFee` newtype.
impl Default for FlatFee {
    fn default() -> Self {
        FlatFee(0.0)
    }
}

impl Billable for FlatFee {
    fn total(&self) -> f32 {
        self.0
    }
}

fn print_total(label: &str, item: &impl Billable) {
    println!("{label}: ${:.2}", item.total());
}

pub fn run() {
    println!("\n== Foreign types, local traits, and the newtype pattern ==");

    let flat_amount = 500.0_f32;
    print_total(
        "local Billable trait implemented for foreign f32",
        &flat_amount,
    );

    let default_fee = FlatFee::default();
    print_total(
        "foreign Default trait implemented for local FlatFee",
        &default_fee,
    );
}
