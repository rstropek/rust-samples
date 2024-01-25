#[derive(Clone)]
pub struct Character {
    name: &'static str,
    power: u32,
}

const BALROG: Character = Character { name: "Balrog", power: 1000 };
const GANDALF: Character = Character { name: "Gandalf", power: 1001 };

pub fn battle<'a>(c1: &'a Character, c2: &'a Character) -> &'a Character {
    if c1.power > c2.power {
        c1
    } else {
        c2
    }
}

pub fn battle_story<'a, /*'b, 'c */>(_c1: &/*'a*/'_ Character, _c2: &/*'a*/'_ Character) -> &'a Character {
    // Gandalf always wins ;-)
    &GANDALF
}

pub struct Item {
    name: &'static str,
}

const STING: Item = Item { name: "Sting" };
const ANDURIL: Item = Item { name: "Andúril" };
const PALANTIRI: Item = Item { name: "Palantíri" };

pub struct CharacterWithStuff<'a> {
    name: &'static str,
    stuff: &'a Item,
}

pub fn characters_with_stuff() {
    let mut stuff = vec![STING, ANDURIL, PALANTIRI];

    #[allow(unused_variables)]
    let frodo = CharacterWithStuff {
        name: "Frodo",
        stuff: &stuff[0],
    };

    stuff.remove(0);
    // println!("{}", frodo.stuff.name);
}