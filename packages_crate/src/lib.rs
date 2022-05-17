mod front_of_house;
//pub use crate::front_of_house::hosting::add_to_waitlist;
use crate::front_of_house::hosting::add_to_waitlist;

fn oder_serve() {}

mod back_of_house {
    pub struct Breakfast{
        pub toast : String,
        seasonal_fruit :String, //private by default
    }

    impl Breakfast {
        pub fn summer(tost : &str) -> Breakfast{
            Breakfast{
                toast: String::from(tost),
                seasonal_fruit: String::from("peaches")
            }
        }
    }
    //In contrast(to struct), if we make an enum public, all of its variants are then public. We only need the pub before the enum keyword
    pub enum Appetizer {
        Soup,  // default of enum variant is public
        Salad,
    }

    fn fix_incorrect_order(){
        cook_order();
        super::oder_serve(); // one level up namespace/mod
    }
    fn cook_order() {}
}

pub fn full_breakfast_eat() {
    let order1 = back_of_house::Appetizer::Salad;
    let order2 = back_of_house::Appetizer::Soup;
}


//use self::front_of_house::hosting::add_to_waitlist //also ok as in the same namespace/file
//The front_of_house module isn’t public, but because the eat_at_restaurant function is defined in the same module as front_of_house (that is, eat_at_restaurant and front_of_house are siblings), we can refer to front_of_house from eat_at_restaurant
pub fn eat_at_restaurant() {
    //absoulte path
    //crate::front_of_house::hosting::add_to_waitlist();

    //relative path
    //front_of_house::hosting::add_to_waitlist();

    //Order a brkfst with Rye Toast in summer
    let mut breakfast_meal = back_of_house::Breakfast::summer("Rye");
    //change mind for the toast choice
    breakfast_meal.toast = String::from("Wheat");
    println!("I'd like {} toast please " , breakfast_meal.toast);
    //breakfast_meal.seasonal_fruit = String::from("Apple") //ERROR for private val
    add_to_waitlist();
}