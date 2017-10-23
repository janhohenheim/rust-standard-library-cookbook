fn main() {
    // We can easily create different configurations
    let normal_burger = BurgerBuilder::new().build();
    let cheese_burger = BurgerBuilder::new()
        .set_cheese(true)
        .set_salad(false)
        .build();
    let veggie_bigmac = BurgerBuilder::new()
        .set_vegetarian(true)
        .number_of_patties(2)
        .build();


    if let Ok(normal_burger) = normal_burger {
        normal_burger.print();
    }
    if let Ok(cheese_burger) = cheese_burger {
        cheese_burger.print();
    }
    if let Ok(veggie_bigmac) = veggie_bigmac {
        veggie_bigmac.print();
    }

    // Our builder can perform a check for
    // invalid configurations
    let invalid_burger = BurgerBuilder::new()
        .set_vegetarian(true)
        .set_bacon(true)
        .build();
    if let Err(error) = invalid_burger {
        println!("Failed to print burger: {}", error);
    }

    // If we omit the last step, we can reuse our builder
    let cheese_burger_builder = BurgerBuilder::new().set_cheese(true);
    for i in 1..10 {
        let cheese_burger = cheese_burger_builder.build();
        if let Ok(cheese_burger) = cheese_burger {
            println!("cheese burger number {} is ready!", i);
            cheese_burger.print();
        }
    }
}


struct Burger {
    number_of_patties: i32,
    is_vegetarian: bool,
    has_cheese: bool,
    has_bacon: bool,
    has_salad: bool,
}
impl Burger {
    // This method is just here for illustrative purposes
    fn print(&self) {
        let pretty_patties = if self.number_of_patties == 1 {
            "patty"
        } else {
            "patties"
        };
        let pretty_bool = |val| if val { "" } else { "no " };
        let pretty_is_vegetarian = if self.is_vegetarian {
            "vegetarian "
        } else {
            ""
        };
        println!(
            "This is a {}burger with {} {}, {}cheese, {}bacon and {}salad",
            pretty_is_vegetarian,
            self.number_of_patties,
            pretty_patties,
            pretty_bool(self.has_cheese),
            pretty_bool(self.has_bacon),
            pretty_bool(self.has_salad)
        )
    }
}


struct BurgerBuilder {
    number_of_patties: i32,
    is_vegetarian: bool,
    has_cheese: bool,
    has_bacon: bool,
    has_salad: bool,
}
impl BurgerBuilder {
    // in the constructor, we can specify
    // the standard values
    fn new() -> Self {
        BurgerBuilder {
            number_of_patties: 1,
            is_vegetarian: false,
            has_cheese: false,
            has_bacon: false,
            has_salad: true,
        }
    }

    // Now we have to define a method for every
    // configurable value
    fn number_of_patties(mut self, val: i32) -> Self {
        self.number_of_patties = val;
        self
    }

    fn set_vegetarian(mut self, val: bool) -> Self {
        self.is_vegetarian = val;
        self
    }
    fn set_cheese(mut self, val: bool) -> Self {
        self.has_cheese = val;
        self
    }
    fn set_bacon(mut self, val: bool) -> Self {
        self.has_bacon = val;
        self
    }
    fn set_salad(mut self, val: bool) -> Self {
        self.has_salad = val;
        self
    }

    // The final method actually constructs our object
    fn build(&self) -> Result<Burger, String> {
        let burger = Burger {
            number_of_patties: self.number_of_patties,
            is_vegetarian: self.is_vegetarian,
            has_cheese: self.has_cheese,
            has_bacon: self.has_bacon,
            has_salad: self.has_salad,
        };
        // Check for invalid configuration
        if burger.is_vegetarian && burger.has_bacon {
            Err(
                "Sorry, but we don't server vegetarian bacon yet".to_string(),
            )
        } else {
            Ok(burger)
        }
    }
}
