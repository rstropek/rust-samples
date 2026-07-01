mod advanced_bounds;
mod associated_items;
mod basics;
mod blanket_impl;
mod foreign_types;
mod object_safety;
mod std_traits;
mod trait_objects;

fn main() {
    basics::run();
    foreign_types::run();
    blanket_impl::run();
    std_traits::run();
    trait_objects::run();
    associated_items::run();
    advanced_bounds::run();
    object_safety::run();
}
