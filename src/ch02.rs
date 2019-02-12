#![allow(unused_variables)] // 不对 未使用的变量 warning


// 引用 标准库 std 中的 io
use std::io;
use std::cmp::Ordering;
use rand::Rng;


pub fn ch02_00_guessing_game_tutorial() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 100 + 1);

    println!("The secret_number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        // 在 Rust 中，变量默认是不可变的
        // let foo = 5; // 不可变
        // let mut bar = 5; // 可变
        let mut guess = String::new();
        // :: 语法表明 new 是 String 类型的一个 关联函数（associated function）
        // 关联函数是针对类型实现的，在这个例子中是 String ，而不是 String 的某个特定实例。一些语言中把它称为 静态方法（static method）
        // new 函数创建了一个新的空字符串，你会发现很多类型上有 new 函数，因为它是创建类型实例的惯用函数名。

        // read_line 的工作是，无论用户在标准输入中键入什么内容，都将其存入一个字符串中，因
        // 此它需要字符串作为参数。这个字符串参数应该是可变的，以便 read_line 将用户输入附加上去。
        // & 表示这个参数是一个 引用（reference），它允许多处代码访问同一处数据，而无需在内
        // 存中多次拷贝。
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed: {}", guess);

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("To big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}




