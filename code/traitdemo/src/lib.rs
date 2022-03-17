use std::fmt::Display;
use std::fmt::Debug;

pub trait Summary{
    fn summarize(&self)->String;

    //trait中还可以提供方法的默认实现
    //再组合上未被实现的方法可以实现方法的模版化
    fn summarizeAll(&self)->String{
        format!("read from {}", self.summarize())
    }
}

pub struct Article{
    pub username:String,
    pub content:String,
}

impl Summary for Article{
    fn summarize(&self)->String{
        format!("{},{}", self.username,self.content)
    }
}

pub struct Twitte{
    pub username:String,
    pub content:String,
    pub tel:String,
}

impl Summary for Twitte{
    fn summarize(&self)->String{
        format!("{},{},{}", self.username,self.content, self.tel)
    }
}

//将trait作为参数的简单使用
pub fn notify1(item:impl Summary+Display){
    println!("breaking news: {}", item.summarize());
}

//使用trait bound
//当使用trait作为返回值时使用->impl Summary
//并且返回值只能返回一个特定的具体值，要么是Article，要么是Twitte
pub fn notify<T: Summary+Display>(item:T){
    println!("breaking news: {}", item.summarize());
}

pub fn notify2<T:Summary+Display, U:Debug+Clone>(a:T, b:U){
    println!("breaking news: {}", a.summarize());
}

pub fn notify3<T, U>(a:T, b:U)
where
    T:Summary+Display,
    U:Debug+Clone,
{
    println!("breaking news: {}", a.summarize());
}