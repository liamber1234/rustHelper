use CoffeProj::Coffe::{CoffeOrder, CoffeSize, CoffeType};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// checks if all of the fields are set correctly in order without sugar
    fn test_new_order_without_sugar() {
        let order = CoffeOrder::new(CoffeType::Espresso, CoffeSize::Small, false, None);

        assert!(matches!(order.coffee_type, CoffeType::Espresso));
        assert!(matches!(order.coffee_size, CoffeSize::Small));
        assert!(!order.has_sugar);
        assert_eq!(order.sugar_amount, None);
    }

    #[test]
    /// checks if all of the fields are set correctly in order with sugar
    fn test_new_order_with_sugar() {
        let order = CoffeOrder::new(CoffeType::Latte, CoffeSize::Medium, true, Some(2),
        );

        assert!(matches!(order.coffee_type, CoffeType::Latte));
        assert!(matches!(order.coffee_size, CoffeSize::Medium));
        assert!(order.has_sugar);
        assert_eq!(order.sugar_amount, Some(2));
    }

}
