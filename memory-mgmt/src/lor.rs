#[derive(Debug)]
pub struct MyPreciousRing {
    pub engraving: String
}

impl MyPreciousRing {
    pub fn forge() -> Self {
        Self {
            engraving: "One Ring to rule them all".to_string()
        }
    }
}

mod we_are_all_friends_here {
    #[derive(Debug, Copy, Clone)]
    pub struct MyPreciousRing {
        engraving: &'static str
    }

    impl MyPreciousRing {
        pub fn forge() -> Self {
            Self {
                engraving: "One Ring to rule them all"
            }
        }
    }
}

pub fn ownership_ring() {
    let saurons_ring = MyPreciousRing::forge();
    //let saurons_ring = forge();
    println!("Sauron's ring says: {saurons_ring:?}");
    
    let gollums_ring = saurons_ring;
    println!("My precious... ({gollums_ring:?})");
    
    let bilbos_ring = gollums_ring;
    // println!("My precious... ({gollums_ring:?})");
    
    let frodos_ring = bilbos_ring;
    println!("Have to destroy it... ({frodos_ring:?})");
    
    let samwises_ring = &frodos_ring;
    println!("Keep it safe... ({samwises_ring:?})");
    println!("Have to destry it... ({frodos_ring:?})");
    
    let mut frodos_ring = frodos_ring;
    heat(&mut frodos_ring);

    // let mut frodos_ring_mut = &mut frodos_ring;
    // let mut samwises_ring_mut = &mut frodos_ring;
    // heat(&mut frodos_ring_mut);
    // heat(&mut samwises_ring_mut);

    destroy(frodos_ring);
    //println!("Have to destroy it... ({frodos_ring:?})");

}

fn forge() -> MyPreciousRing {
    MyPreciousRing::forge()
}

pub fn heat(ring: &mut MyPreciousRing) {
    // The inscription becomes readable...
    ring.engraving = "Ash nazg durbatulûk, ash nazg gimbatul, ash nazg thrakatulûk agh burzum-ishi krimpatul.".to_string();
}

fn destroy(_ring: MyPreciousRing) {
    // Source: https://emojicombos.com/one-ring-ascii-art
    println!(r"
    ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣠⣤⣶⣶⣶⣶⣄⠀⢠⣄⡀⠀⠀⠀⠀
    ⠀⠀⠀⠀⠀⠀⠀⠀⢀⣠⣾⣿⣿⡿⠛⢻⣿⣿⣿⠀⢀⣿⣿⣦⡀⠀⠀
    ⠀⠀⠀⠀⠀⠀⣠⣴⣿⣿⣿⠋⠉⠁⠀⣸⣿⣿⡏⠀⢸⣿⣿⣿⣷⡄⠀
    ⠀⠀⠀⠀⢀⣾⣿⣿⠋⠁⠉⠀⣰⣶⣾⣿⡿⠟⠀⢠⣿⣿⣿⣿⣿⣿⡄
    ⠀⠀⠀⣴⣿⣿⠟⠛⠀⠀⣿⣿⣿⡿⠛⠉⠀⠀⢠⣾⣿⣿⣿⣿⣿⣿⡇
    ⠀⢀⣾⣿⣿⠿⠀⠀⣶⣾⣿⡿⠋⠀⠀⠀⠀⣰⣿⣿⡟⠉⢻⣿⣿⣿⠇
    ⠀⣾⣿⡏⠀⢀⣀⣴⣿⡿⠋⠀⠀⠀⠀⣠⣾⣿⣿⠋⠁⠀⢀⣿⣿⡟⠀
    ⢸⣿⣿⣧⣀⣼⣿⣿⡟⠁⠀⠀⠀⣠⣾⣿⣿⠛⠛⠀⠀⣾⣿⣿⡟⠀⠀
    ⠸⣿⣿⣿⣿⣿⡿⠏⠀⠀⢀⣠⣾⣿⡿⠿⠿⠀⢠⣤⣾⣿⣿⠟⠀⠀⠀
    ⠀⠈⠉⠉⠁⠀⢀⣀⣤⣾⣿⣿⠿⠿⠃⠀⣀⣠⣾⣿⣿⡿⠃⠀⠀⠀⠀
    ⠀⠳⣶⣶⣶⣿⣿⣿⣿⣿⣿⣏⠀⢀⣀⣠⣿⣿⣿⡿⠋⠀⠀⠀⠀⠀⠀
    ⠀⠀⠙⢿⣿⣿⣿⣿⣿⣿⣿⣿⣶⣾⣿⣿⣿⠟⠁⠀⠀⠀⠀⠀⠀⠀⠀
    ⠀⠀⠀⠀⠙⠻⢿⣿⣿⣿⣿⣿⣿⠿⠛⠉⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
    ⠀⠀⠀⠀⠀⠀⠀⠈⠉⠉⠉⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀");
}

pub fn happy_lor() {
    let saurons_ring = we_are_all_friends_here::MyPreciousRing::forge();
    //let saurons_ring = forge();
    println!("Sauron's ring says: {saurons_ring:?}");
    
    let gollums_ring = saurons_ring;
    println!("My precious... ({gollums_ring:?})");
    
    let bilbos_ring = gollums_ring;
    println!("My precious... ({gollums_ring:?})");
    
    let frodos_ring = bilbos_ring;
    println!("Have to destroy it... ({frodos_ring:?})");
    
    // Everyone is happy and can do whatever they want with their (copy of the) ring ;-)
}
