#[derive(Debug)]
pub struct Rectangle{
    width:u32,
    length:u32,
}

// 为结构体定义方法
impl Rectangle{
    fn area(&self) -> u32{
        self.width * self.length
    }

    pub fn can_hold(&self, other: &Rectangle) -> bool{
        self.width > other.width && self.length > other.length
    }

    // 定义关联函数(类似于静态函数)
    fn square(size :u32) -> Rectangle{
        Rectangle{
            width:size,
            length:size,
        }
    }
}


pub fn add_two(x : i32)->i32{
    x + 2
}

//rust单元测试可以测试priavte函数
fn greeting(name : &str)->String{
    format!("hello {}", name)
}

pub struct Guess{
    value : u32,
}

impl Guess{
    pub fn new(value : u32) -> Guess{
        if value < 1{
            panic!("must be greater than 1, got {}", value);
        }else if value > 100{
            panic!("must be less than 100, got {}", value);
        }
        Guess{value}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_hold_test() {
        let rec1 = Rectangle{
            width:100,
            length:55,
        };
        let rec2 = Rectangle{
            width:30,
            length:50,
        };
        assert!(rec1.can_hold(&rec2));
    }

    #[test]
    fn add_two_test(){
        assert_eq!(add_two(2),4);
    }

    #[test]
    fn add_two_test2(){
        assert_ne!(add_two(2),5);
    }

    #[test]
    fn greeting_test(){
        let result = greeting("aa");
        //传递自定义错误信息
        assert!(result.contains("aa"),
                "dont contain '{}'", "aa");
    }

    #[test]
    //当期待测试函数发生panic时加入下方标识，还可以通过expected来使得更精确的识别panic发生的位置
    #[should_panic(expected="must be less than 100")]
    fn guess_new_test(){
        Guess::new(200);
    }

    #[test]
    //使用Result作为返回值，这时就不需要should_panic进行标注
    //标记了ignore的测试不会自动执行
    #[ignore]
    fn result_test() -> Result<(), String>{
        if 2 + 2 == 4{
            Ok(())
        }else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
