use std::{any::Any, fmt};

struct Cat(String);

impl Cat {
    fn feed(&mut self) {
        self.0 = format!("{} (purring)", self.0);
    }
}

struct CatFeeder<'a> {
    cat: &'a mut Cat,
}

impl<'a> CatFeeder<'a> {
    fn feed(&mut self) {
        self.cat.feed();
    }
}

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Point")
            .field("x", &self.x)
            .field("y", &self.y)
            .finish()
    }
}

trait Animal {
    fn speak(&self);
}

struct Dog;

impl Animal for Dog {
    fn speak(&self) {
        println!("Wang!!!");
    }
}

struct Duck;

impl Animal for Duck {
    fn speak(&self) {
        println!("Gagaga!!!");
    }
}

fn speak_twince(animal: &impl Animal) {
    for _ in 0..2 {
        animal.speak();
    }
}

fn speak_twince2(animal: Box<dyn Animal>) {
    animal.speak();
}

fn make_animal() -> impl Animal {
    Dog
}

trait DowncastableAnimal {
    fn speak(&self) {
        println!("No idea")
    }

    fn as_any(&self) -> &dyn Any;
}

struct Toroise;

impl DowncastableAnimal for Toroise {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

fn main() {
    let mut cats = vec![Cat("Frodo".to_string()), Cat("Bilbo".to_string())];

    let mut feeders = Vec::new();
    for cat in cats.iter_mut() {
        feeders.push(CatFeeder { cat })
    }

    feeders.iter_mut().for_each(|f| f.feed());

    let p = Point { x: 1, y: 2 };

    println!("{:#?}", p);

    let dog = Dog;
    dog.speak();

    let duck = Duck;
    duck.speak();

    speak_twince(&dog);
    speak_twince(&duck);

    let animal = make_animal();
    animal.speak();

    let dog2 = Box::new(Dog);
    let mut animals: Vec<Box<dyn Animal>> = Vec::new();
    animals.push(dog2);
    animals.push(Box::new(Duck));

    animals.iter().for_each(|a| a.speak());

    let more_animals: Vec<Box<dyn DowncastableAnimal>> = vec![Box::new(Toroise)];
    for animal in more_animals.iter() {
        if let Some(a) = animal.as_any().downcast_ref::<Toroise>() {
            println!("I'm a tortoise");
        }
    }
}
