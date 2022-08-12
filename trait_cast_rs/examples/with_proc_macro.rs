#![feature(trait_upcasting, const_type_id)]
#![allow(incomplete_features)]
use std::any::Any;

use trait_cast_rs::{make_trait_castable, trait_cast, TraitcastTarget, Traitcastable};

extern crate trait_cast_rs;

#[make_trait_castable(Dog, Cat)]
struct HybridPet {
  name: String,
}
impl HybridPet {
  fn greet(&self) {
    println!("{}: Hi", self.name)
  }
}

impl Dog for HybridPet {
  fn bark(&self) {
    println!("{}: Woof!", self.name);
  }
}
impl Cat for HybridPet {
  fn meow(&self) {
    println!("{}: Meow!", self.name);
  }
}

trait Dog {
  fn bark(&self);
}
trait Cat {
  fn meow(&self);
}

fn main() {
  // The box is technically not needed but kept for added realism
  let pet = Box::new(HybridPet {
    name: "Kokusnuss".to_string(),
  });
  pet.greet();

  let castable_pet: Box<dyn Traitcastable> = pet;

  // WARNING: YOU MUST USE `as_ref()` otherwise you would cast the Box to an Any!
  let as_dog = trait_cast::<dyn Dog>(castable_pet.as_ref()).unwrap();
  as_dog.bark();

  let as_cat = trait_cast::<dyn Cat>(castable_pet.as_ref()).unwrap();
  as_cat.meow();

  let any_pet = castable_pet as Box<dyn Any>;
  let cast_back: &HybridPet = any_pet.downcast_ref().unwrap();
  cast_back.greet();
}