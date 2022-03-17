//修改为泛型
//要求T实现PartialOrd+Clone这两个trait
fn largestCommon<T:PartialOrd+Clone>(list: &[T])->T{
    let mut largest = list[0].clone();
    // 如果number前面不加&符号的话，得到的是i32的引用
    for number in list.iter(){
        if number > &largest{ //实现了std::cmp::PartialOrd才可以比较
            largest = number.clone();
        };
    };
    largest
}

fn largest(list: &[i32])->i32{
    let mut largest = list[0];
    // 如果number前面不加&符号的话，得到的是i32的引用
    for &number in list{
        if number > largest{
            largest = number
        };
    };
    largest
}

fn main() {
    let number_list = vec![51,21,53,95,74,84,1,23];
    let result = largest(&number_list);
    println!("largers number: {}", result);

    let number_list = vec![51,21,53,95,74,84,100,23];
    let result = largest(&number_list);
    println!("largers number: {}", result);
}
