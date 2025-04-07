#[derive(Debug, Clone, Copy)]
/// CoffeType enum represents different types of coffee.
pub enum CoffeType {
    Espresso,
    Latte,
    Cappuccino,
    Americano,
}

#[derive(Debug, Clone, Copy)]
/// CoffeSize enum represents different sizes of coffee.
pub enum CoffeSize {
    Small,
    Medium,
    Large,
}

/// CoffeOrder struct represents a coffee order.
pub struct CoffeOrder {
    pub coffee_type: CoffeType,
    pub coffee_size: CoffeSize,
    pub has_sugar: bool,
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
    pub fn new(coffee_type: CoffeType, coffee_size: CoffeSize, has_sugar: bool, sugar_amount: Option<u8>) -> Self {
        CoffeOrder {
            coffee_type,
            coffee_size,
            has_sugar,
            sugar_amount,
        }
    }

    /// Prints the details of the coffee order.
    /// parameters:
    /// - None
    /// returns:
    /// - None
    pub fn print_order(&self) {
        println!("Coffee Type: {:?}", self.coffee_type);
        println!("Coffee Size: {:?}", self.coffee_size);
        println!("Contains Sugar: {:?}", self.has_sugar);
        if let Some(amount) = self.sugar_amount {
            println!("Sugar Amount: {} spoons", amount);
        } else {
            println!("Sugar Amount: None");
        }
    }
}