use std::fmt::Display;

pub struct Pair<T>{
    pub x:T,
    pub y:T,
}

//不管T为什么类型，都有new方法
pub impl<T> Pair<T>{
    fn new(x:T, y:T)->Self{
        Self{x,y}
    }
}

//只有实现了Display+PartialOrd这两个trait的泛型拥有cmpDisplay方法
pub impl<T:Display+PartialOrd> Pair<T>{
    pub fn cmpDisplay(&self){
        if self.x >= self.Y{
            println!("aaaa");
        }else {
            println!("bbbb");
        }
    }
}