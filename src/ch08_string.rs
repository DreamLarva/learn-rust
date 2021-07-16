#![allow(unused_variables)] // 不对 未使用的变量 warning

use std::collections::*;
use std::fs::{self, File};
use std::io;
use std::io::ErrorKind;
use std::io::Read;
use std::ops::Add;

pub fn ch08_02_strings() {
    // 使用字符串存储 UTF-8编码的文本
    // Rust 的核心语言中只有一种字符串类型：str，字符串 slice，它通常以被借用的形式出现，&str
    // String 是一个 Vec<u8> 的封装

    // 称作 String 的类型是由标准库提供的，而没有写进核心语言部分，
    // 它是可增长的、可变的、有所有权的、UTF-8 编码的字符串类型。

    // 新建字符串
    {
        let mut a = String::new(); // 使用类似Vector 的方法 新建一个空的字符串

        let data = "initial contents";
        let data: &str = "initial contents";

        let s = data.to_string();
        let s: String = data.to_string();

        // 该方法也可直接用于字符串字面值：
        let s = "initial contents".to_string();
        let s: String = "initial contents".to_string();

        // String::from 和 .to_string 最终做了完全相同的工作
        // 可以按照个人喜好选择使用
    }

    // 更新字符串
    {
        // 使用push 换个 push_str 更新字符串
        // 可以通过 push_str 方法来附加**字符串 slice**，从而使 String 变长
        let mut s1 = String::from("foo");
        let s2 = "bar";
        s1.push_str(s2);
        println!("s2 is {}", s2);

        // push 方法被定义为获取一个单独的字符作为参数，并附加到 String 中。
        let mut s3 = String::from("lo");
        s3.push('l');
        println!("s3 is {}", s3);
    }

    // 使用 + 运算符 或者 format!宏拼接字符串
    {
        let s1 = String::from("Hello, ");
        let s7 = String::from("Hello, ");
        let s2 = String::from("world");
        let s4 = String::from("!");
        // &s2 是 &String 可以被 强转（coerced）成 &str。
        // Rust 使用了一个被称为 解引用强制多态（deref coercion）的技术，你可以将其理解为它把 &s2 变成了 &s2[..]。
        let mut s3 = s1 + &s2 + &s4; // 注意 s1 被移动了，不能继续使用 s2 s3 可以继续使用
        let mut t3 = String::from("") + &s2 + &s3;
        let s3 = s3.add(" 哈 ");
        // let s5 = &s2 + &s4; // error 不能全是 &str 类型
        let s6 = String::from("1");
        let s7 = String::from("2");
        // let s8 = s6 + s7; // error 第二个开始 就必须是 &str 类型
        // 其实就是 String 上 add 方法调用了2次
        // println!("{}",s1);
        println!("{}", s2);
        println!("{}", &s3);

        // 类似模块字符串
        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");

        // 并不会 获取所有权
        let s = format!("{}-{}-{}", s1, s2, s3);
        println!("{} {}", s, s1); // s1 s2 s3 依然有效

        // 之所以能够在 add 调用中使用 &s2 是因为 &String 可以被 强转（coerced）成 &str。
        // 当add函数被调用时，Rust 使用了一个被称为 解引用强制多态（deref coercion）的技术，
        // 你可以将其理解为它把 &s2 变成了 &s2[..]。
        let test = String::from("123");
        let test = &test[..];
    }
    // 索引字符串
    // rust 中是不能使用索引访问字符串的
    // 当使用 UTF-8 访问的时候 不能判断是 多少个字节是一个字符
    {
        // let s1 = String::from("hello");
        // let h = s1[0]; // error
    }
    // 字符串 slice
    // 可以使用 [] 和一个 range 来创建含特定字节的字符串 slice
    {
        let hello = "Здравствуйте";

        let s = &hello[0..4]; // 每个 Unicode 标量值需要两个字节存储
                              // let s = &hello[0..1]; // error
                              // thread 'main' panicked at 'byte index 1 is not a char boundary; it is inside 'З' (bytes 0..2) of `Здравствуйте`'
                              // rust 自动判断了 一个字符的边界

        println!("字符串 slice {}", s)
    }
    // 遍历字符串
    {
        // 返回六个 char 类型的值
        for c in "नमस्ते".chars() {
            println!("{}", c);
            // न
            // म
            // स
            // ्
            // त
            // े
        }

        // bytes 方法返回每一个原始字节
        for b in "नमस्ते".bytes() {
            println!("{}", b);
            // 打印18个字节
        }
    }

    // String 实现了 Deref<Target=str> , &String 可以自动解构成 &str
    {
        fn takes_str(s: &str) {}

        let s = String::from("Hello");

        takes_str(&s);
    }
    // In the following example a string slice &'a str implements the trait TraitExample,
    // and the function example_func takes anything that implements the trait.
    // In this case Rust would need to make two implicit conversions,
    // which Rust doesn't have the means to do. For that reason,
    // the following example will not compile.
    {
        trait TraitExample {}

        impl<'a> TraitExample for &'a str {}

        fn example_func<A: TraitExample>(example_arg: A) {}

        let example_string = String::from("example_string");

        // example_func(&example_string);
        //              ^^^^^^^^^^^^^^^
        //              |
        //              the trait `TraitExample` is not implemented for `&String`
        //              help: consider adding dereference here: `&*example_string`

        // In this case we are dereferencing a String to a str,
        // then referencing the str back to &str
        // 先解引用 再 引用
        example_func(&*example_string);
        example_func(example_string.as_str());
    }
}
