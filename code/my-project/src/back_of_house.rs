pub struct Breakfast{
    // 加pub关键字将使得字段可被外部访问
    pub toast: String,
    fruit: String,
}

impl Breakfast{
    pub fn summer(toast: &str) -> Breakfast{
        Breakfast{
            toast: String::from(toast),
            fruit: String::from("apple"),
        }
    }
}