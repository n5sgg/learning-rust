#[allow(dead_code)]
#[derive(Debug)]
enum PizzaSize {
    Personal,
    Family,
}

#[allow(dead_code)]
#[derive(Debug)]
enum Veggies {
    Mushrooms,
    GreenPeppers,
    Onions,
    Tomatoes,
    Pineapples,
}

#[allow(dead_code)]
#[derive(Debug)]
enum Meats {
    Sausage,
    Chicken,
    Pepperoni,
    Bacon,
}

#[derive(Debug)]
enum PizzaToppings {
    Veggies(Veggies),
    Meats(Meats),
}

#[derive(Debug)]
struct Pizza {
    size: PizzaSize,
    topping: Vec<PizzaToppings>,
}

impl Pizza {
    pub fn price(&self) -> f32 {
        self.topping.iter().fold(
            match self.size {
                PizzaSize::Personal => 9.,
                PizzaSize::Family => 18.,
            },
            |total, topping| match topping {
                PizzaToppings::Veggies(_) => total * 1.05,
                PizzaToppings::Meats(_) => total * 1.1,
            },
        )
    }

    pub fn with(mut self, topping: PizzaToppings) -> Self {
        self.topping.push(topping);
        self
    }

    pub fn new(size: PizzaSize) -> Self {
        Self {
            size: size,
            topping: vec![],
        }
    }
}

fn main() {
    let pizza = Pizza::new(PizzaSize::Personal)
        .with(PizzaToppings::Meats(Meats::Chicken))
        .with(PizzaToppings::Veggies(Veggies::GreenPeppers));

    println!("Finished pizza: {:?} for ${}", pizza, pizza.price());
}
