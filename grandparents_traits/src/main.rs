use std::ops::Deref;

mod basics;
mod foreign_types;
mod blanket_impl;
mod std_traits;
mod trait_objects;

fn main() {
    let b = basics::ConsultingWork::new("AI Consulting", 10.0, 100.0);
    basics::print_total(&b);

    let b = 500.0;
    foreign_types::print_total(&b);

    use foreign_types;
    let b: foreign_types::MyF32 = Default::default();
    foreign_types::print_total(b.deref());

    let b = blanket_impl::CodingWork::new(blanket_impl::ShirtSize::Epic);
    blanket_impl::print_points(&b);

    
    let b = [
        blanket_impl::CodingWork::new(blanket_impl::ShirtSize::Small),
        blanket_impl::CodingWork::new(blanket_impl::ShirtSize::Medium),
        ];
        blanket_impl::print_points(&b);
        
    use std_traits;
    let b1 = std_traits::ConsultingWork { what: "AI Consulting".to_string(), hours: 10.0, rate: 100.0 };
    let b2 = std_traits::ConsultingWork { what: "Rust Coding".to_string(), hours: 10.0, rate: 150.0 };
    let c = b1 + b2;
    std_traits::print_work(&c);
    
    let b1 = std_traits::ConsultingWork { what: "AI Consulting".to_string(), hours: 10.0, rate: 100.0 };
    let b2 = std_traits::ConsultingWork { what: "Rust Coding".to_string(), hours: 10.0, rate: 150.0 };
    let b = [b1, b2];
    std_traits::print_work(&b.into_iter().sum());

    let b = trait_objects::create_billable("AI Consulting", 10.0, 100.0);
    trait_objects::print_total(&b);
    
    let b = trait_objects::create_billable2(None, 10.0, 100.0);
    trait_objects::print_total(b.as_ref());

    let b = trait_objects::create_billables();
    for b in b.iter() {
        trait_objects::print_total(b.as_ref());
    }
}
