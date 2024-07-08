use std::ops::Not;

#[derive(Debug)]
struct Person<PetType, PetType1>
where
    PetType: Animal + NotDangerous,
    PetType1: Dangerous + Animal,
{
    name: String,
    pet: PetType,
    pet2: PetType1,
}

trait Animal {
    fn make_sound(&self) -> ();
}

trait NotDangerous {}

trait Dangerous {}
struct Cat {}

impl NotDangerous for Cat {}
impl Animal for Cat {
    fn make_sound(&self) -> () {
        println!("Cat meowed!");
    }
}

#[derive(Debug)]
struct Dog {}
impl NotDangerous for Dog {}
impl Animal for Dog {
    fn make_sound(&self) -> () {
        println!("Dog barked!");
    }
}
struct Tiger {}

impl Dangerous for Tiger {}

impl Animal for Tiger {
    fn make_sound(&self) -> () {
        println!("Tiger Roared!");
    }
}
struct Bear {}
impl Dangerous for Bear {}
impl Animal for Bear {
    fn make_sound(&self) -> () {
        println!("Bear roared!");
    }
}

pub fn create_person() {
    let pet1 = Dog {};
    let pet2 = Cat {};
    let pet3 = Bear {};
    let pet4 = Tiger {};

    let p1 = Person {
        name: String::from("Trevor"),
        pet: pet2,
        pet2: pet3,
    };
    p1.pet.make_sound();
    p1.pet2.make_sound();
}

fn main() {
    create_person()
}
