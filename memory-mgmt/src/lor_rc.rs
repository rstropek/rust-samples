use std::{cell::RefCell, rc::Rc};

#[allow(unused_imports)]
use crate::lor::{heat, MyPreciousRing};

pub fn share_the_ring() {
    let saurons_ring = MyPreciousRing::forge();
    println!("Sauron's ring says: {saurons_ring:?}");

    // Somehow, the ring gets to Frodo...

    let frodos_ring = Rc::new(saurons_ring);
    //println!("Sauron's ring says: {saurons_ring:?}");
    println!("The ring now has {} owner", Rc::strong_count(&frodos_ring));
    println!("Have to destroy it... ({frodos_ring:?})");

    let samwises_ring = Clone::clone(&frodos_ring);
    println!("Keep it safe... ({samwises_ring:?})");

    println!("The ring now has {} owner", Rc::strong_count(&frodos_ring));
    println!(
        "\t{:p}\n\t{:p}",
        frodos_ring.as_ref(),
        samwises_ring.as_ref()
    );

    //let mut frodos_ring = frodos_ring;
    //heat(&mut frodos_ring);
}

pub fn share_and_alter() {
    let saurons_ring = MyPreciousRing::forge();
    // Somehow, the ring gets to Frodo...

    let frodos_ring = Rc::new(RefCell::new(saurons_ring));
    println!("Have to destroy it... ({:?})", frodos_ring.borrow());

    let samwises_ring = Clone::clone(&frodos_ring);
    println!("The ring now has {} owner", Rc::strong_count(&frodos_ring));

    heat(&mut frodos_ring.borrow_mut());
    println!("The ring says: {:?}", samwises_ring.borrow());

    // let mut frodos_ring_mut = frodos_ring.borrow_mut();
    // let mut samwises_ring_mut = samwises_ring.borrow_mut();
    // heat(&mut frodos_ring_mut);
    // heat(&mut samwises_ring_mut);
}
