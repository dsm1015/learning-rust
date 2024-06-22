mod front_of_house;
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }
    
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
    
    pub enum Appetizer {
        Soup,
        Salad,
    }
    
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }
    fn cook_order() {
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // Absolute path
    //crate::front_of_house::hosting::add_to_waitlist();
    // Relative path
    hosting::add_to_waitlist();
    
    //Order breakfast in summer with rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    //Change our mind about the bread
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please!", meal.toast);
    
    //This line won't compile because the fruit is private
    //meal.seasonal_fruit = String::from("blueberries");
    
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

fn deliver_order() {
    
}