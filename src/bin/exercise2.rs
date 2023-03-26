// 1. create a struct Unicorn with fields:
    // name of type String
    // magic_powers of type u32
struct Unicorn {
  name: String,
  magic_powers: u32
}

// 2. create a struct Griffin with fields:
    // name of type String
    // magic_powers of type u32
struct Griffin {
  name: String,
  magic_powers: u32
}

// 5. create an enum Creature with Variants:
    // Unicorn with data of (Unicorn)
    // Griffin with data of (Griffin)
enum Creature {
  Unicorn(Unicorn),
  Griffin(Griffin)
}

// 8. implement onto Creature
    // 9. define magic_power function that takes in a referenced self and outputs unsigned 32bits integer
        // matches on itself
            // if its of variant Unicorn, return referenced unicorn's magic power
            // if its of variant Griffin, return referenced griffin's magic power
impl Creature {
  // 12. define name function that takes in a referenced self and outputs a string slice
    // matches on itself
        // if its of variant Unicorn, return referenced unicorn's name
        // if its of variant Griffin, return referenced griffin's name

  // TODO: Return "Result" wrapper here in case its not any of these enums
  fn magic_power(&self) -> u32 {
    match &self {
      Creature::Unicorn(unicorn) => unicorn.magic_powers,
      Creature::Griffin(griffin) => griffin.magic_powers,
      _ => 0
    }
  }

  fn name(&self) -> &str {
    match &self {
      Creature::Unicorn(unicorn) => &unicorn.name,
      Creature::Griffin(griffin) => &griffin.name,
      _ => ""
    }
  }

  fn clone(&self) -> Self {
    match &self {
      Creature::Unicorn(_) => Creature::Unicorn(Unicorn {
          name: self.name().to_string(),
          magic_powers: self.magic_power()
        }),
      Creature::Griffin(_) => Creature::Griffin(Griffin 
          { 
            name: self.name().to_string(), 
            magic_powers: self.magic_power() 
          }
        )
    }
  }
}


// 13. define clone function which takes in a referenced self and returns Self/Creature (interchangeable)
// matches on itself
  // if its of variant Unicorn, return a Unicorn variant
      // name should be assigned to the output of the previously defined name function above
      // magic_power should be assigned to the output of the previously defined magic_power function above
  // if its of variant Griffin, return a Griffin variant
      // name should be assigned to the output of the previously defined name function above
      // magic_power should be assigned to the output of the previously defined magic_power function above

// 10. define compare_magic function, takes in 2 referenced creatures and return 1 referenced creature (lifetime!!)
    // if first creature has larger output derived from magic_power function
        // return first creature
    // else
        // return second creature
fn compare_magic<'a>(creature1: &'a Creature, creature2: &'a Creature) -> &'a Creature {
  if creature1.magic_power() > creature2.magic_power() {
    creature1
  } else {
    creature2
  }
}

// 14. define creature_box function, takes in 1 referenced creature and returns box with an owned creature (lifetime!!)
    // return a box containing an owned creature by calling the clone function within  the box
fn creature_box(creature: &Creature) -> Box<Creature> {
  return Box::new(creature.clone())
}

    fn main() {
        // 3. create a variable unicorn of Unicorn instance
        let unicorn = Unicorn {
          name: "Unicon".to_owned(),
          magic_powers: 50
        };

        // 4. create a variable griffin of Griffin instance
        let griffin: Griffin = Griffin { 
          name: "Griffin".to_owned(), 
          magic_powers: 40
        };

        // 6. create variable unicorn_creature, a Creature enum with Unicorn variant
        let unicorn_creature = Creature::Unicorn(unicorn);

        // 7. create variable griffin_creature, a Creature enum with Griffin variant
        let griffin_creature: Creature = Creature::Griffin(griffin);

        // 11. create stronger_creature, the output of compare_magic function, passing in unicorn and griffin creature
        let stronger_creature = compare_magic(&unicorn_creature, &griffin_creature);

        // 15. create boxed_creature, the output of creature_box function, passing in stronger creature
        let boxed_creature = creature_box(stronger_creature);

        // 16. print "The stronger creature is <creature name> with a magic power of <creature power>."
        println!("The stronger creature is {} with a magic power of {}", boxed_creature.name(), boxed_creature.magic_power());
    }