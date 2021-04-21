# 创建和使用变量
在Rust中可以使用`let`关键字将值绑定到变量。如下所示：

```rust
fn main() {
    let a = 10;
    let b = true;
    println!("a is {}, b is {}", a, b);
    //a = 20;
}
```

上面的代码中分别用let声明了a，b两个变量，通过输出函数对其进行输出。在终端中使用命令

```bash
cargo run
```

程序会输出

```
a is 10, b is true
```

但是如果把代码中注释的部分放开，会发现程序报错。

![](https://tva1.sinaimg.cn/large/008eGmZEly1gpr3ekeepsj30ez06xmxn.jpg)

主要是因为在Rust中使用let声明的变量是不可变的，如果想要让其是可变的需要添加mut变量，如下所示：

```Rust
fn main() {
    let mut a = 10;
    let b = true;
    println!("a is {}, b is {}", a, b);
    a = 20;
    println!("a is {}, b is {}", a, b);
}
```

程序会输出

```
a is 10, b is true
a is 20, b is true
```

还有一种操作成为**阴影操作**，用户还可使用与上一个变量相同的名称声明新变量，这会创建一个新绑定。 在 Rust 中，此操作称为“隐藏”，这是由于新变量会隐藏上一个变量。 旧变量仍存在，但无法再于此范围内引用它。如下所示：

```rust
fn main() {
    let number = 5;          // the first binding is created using the name "number"
    let number = number + 5; // a different binding shadows the name "number"
    let number = number * 2; // again, a new binding is created
    println!("The number is: {}", number);
}
```

程序输出

```
The number is: 20
```



