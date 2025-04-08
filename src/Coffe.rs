use std::fmt;

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
/// CoffeType enum represents different types of coffee.
pub enum CoffeType {
    Espresso = 1,
    Latte = 2,
    Cappuccino = 3,
    Americano = 4,
}

impl fmt::Display for CoffeType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CoffeType::Espresso => write!(f, "Espresso"),
            CoffeType::Latte => write!(f, "Latte"),
            CoffeType::Cappuccino => write!(f, "Cappuccino"),
            CoffeType::Americano => write!(f, "Americano"),
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
/// CoffeSize enum represents different sizes of coffee.
pub enum CoffeSize {
    Small = 1,
    Medium = 2,
    Large = 3,
}

impl fmt::Display for CoffeSize {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CoffeSize::Small => write!(f, "Small"),
            CoffeSize::Medium => write!(f, "Medium"),
            CoffeSize::Large => write!(f, "Large"),
        }
    }
}

/// CoffeOrder struct represents a coffee order.
pub struct CoffeOrder {
    pub coffee_type: CoffeType,
    pub coffee_size: CoffeSize,
    pub sugar_amount: Option<u8>,
}

impl CoffeOrder {
    /// Creates a new `CoffeOrder` instance.
    /// parameters:
    /// - coffe_type : the type of coffe
    /// - coffe_size : the size of coffe
    /// - has_sugar : if the coffe contains sugar
    /// - sugar_amount : the amount of sugar in spoons, if any
    /// returns: 
    /// - a new CoffeOrder instance
    pub fn new(coffee_type: CoffeType, coffee_size: CoffeSize, sugar_amount: Option<u8>) -> Self {
        CoffeOrder {
            coffee_type,
            coffee_size,
            sugar_amount,
        }
    }

    /// Prints the details of the coffee order.
    /// parameters:
    /// - None
    /// returns:
    /// - None
    pub fn print_order(&self) {
        println!("Coffee Type: {}", self.coffee_type);
        println!("Coffee Size: {}", self.coffee_size);
        if let Some(amount) = self.sugar_amount {
            println!("Sugar Amount: {} spoons", amount);
        } else {
            println!("Sugar Amount: None");
        }
    }
}