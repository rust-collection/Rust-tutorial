struct Person{
    name: String,
    age: u8,
    likes_oranges: bool,
}

fn main() {
    let person = Person{
        name: String::from("Adam"),
        likes_oranges: true,
        age: 25
    };

    if person.likes_oranges {
        println!("{:?} is {:?} and likes oranges.", person.name, person.age);  
    } else {
        println!("{:?} is {:?} and doesn't like oranges.", person.name, person.age);  
    }
}
