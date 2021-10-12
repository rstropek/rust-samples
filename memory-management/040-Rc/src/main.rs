use std::rc::Rc;

#[derive(Debug)]
struct MyPrecious {
    ring: i32,
}

impl Drop for MyPrecious {
    fn drop(&mut self) {
        println!("Dropping {:?}", self);
    }
}

fn main() {
    let mine = Box::new(MyPrecious{ ring: 21 });
    let also_mine = mine;
    //println!("{:?}", mine); // Does not work
    println!("{:?}", also_mine.ring);

    let mine = Rc::new(MyPrecious{ ring: 42 });
    {
        let also_mine = Clone::clone(&mine);
        println!("{:?} {:p}", mine.ring, mine.as_ref());
        println!("{:?} {:p}", also_mine.ring, also_mine.as_ref());
        println!("Ref count inside: {:?}", Rc::strong_count(&also_mine));
    }

    println!("Ref count outside: {:?}", Rc::strong_count(&mine));
}
