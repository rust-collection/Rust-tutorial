mod back_of_house;

pub fn eat_at_restaurant(){
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("wheat");
    // 因为fruit是私有的，通过这种方式调用会报错
    // meal.fruit = String::from("peaches");
}