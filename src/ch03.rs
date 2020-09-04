#![allow(unused_variables)] // 不对 未使用的变量 warning

use std::num::Wrapping;

pub fn ch03_02_data_types() {
    /** 基本类型 */
    // 直接的赋值能够直接推断出值的类型
    let a = 1; // 默认为 i32
    println!("a : {}", a);
    let b = 2.0; // 默认为 f64
    println!("b : {}", b);

    // 或者显示指定类型注解
    //    let c:u32 = 123;
    //    let d:f32 = 123.0;


    // let guess= "42".parse().expect("Not a number!");  // 这句会报错 编译器没法推断出类型
    let guess: u32 = "42".parse().expect("Not a number!");
    // let guess: u8 = "-42".parse().expect("Not a number!"); // 运行时报错
    // let guess: u8 = "4222222222222222".parse().expect("Not a number!"); // 运行时报错
    println!("guess : {}", guess);

    // 数字类型
    // Length	  Signed	Unsigned
    // 8-bit	  i8	    u8
    // 16-bit	  i16	    u16
    // 32-bit	  i32	    u32
    // 64-bit	  i64	    u64
    // 128-bit	  i128	    u128
    // arch	      isize	    usize
    let decimal = 98_222; // 10进制
    let hex = 0xff; // 16进制
    let octal = 0o77; // 8进制
    let binary = 0b1111_0000; // 2进制
    let byte = b'A'; // 字节

    // 无 release 的 会报panic
    // --release 编译出来的代码 数字越界默认是不会报错的而是,从最小值开始重新计数
    // 如果希望 无 release 也不报错 使用 Wrapping 方法
    let zero = Wrapping(0u32);
    let one = Wrapping(1u32);
    println!("zero - one {}", 100u32);
    assert_eq!(u32::MAX, (zero - one).0);

    // 浮点类型
    // f32 单精度 和 f64(默认) 双精度
    let f64 = 2.0; // f64

    let f32: f32 = 3.0; // f32
    // 支持使用 数值运算
    // addition
    let sum = 5 + 10;
    // subtraction
    let difference = 95.5 - 4.3;
    // multiplication
    let product = 4 * 30;
    // division
    let quotient = 56.7 / 32.2;
    // remainder
    let remainder = 43 % 5;


    // 布尔类型
    let t = true;
    let f: bool = false; // 显式指定类型注解
    println!("{},{}", t, f);

    // 字符 单引号
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("{},{},{}", c, z, heart_eyed_cat);

    // 字符串 4个字节 所以不能显示 8个字节的中文 emoji 等
    let string = "123abc";
    println!("{}", string);

    /** 符合类型 */
    // 元组类型
    // 一旦初始化 大小就固定了 不能添加或者删除
    let _tup1: (i32, f64, u8) = (500, 6.4, 1); // 可以设置多种类型
    let _tup2 = (500, 6.4, 1, '1'); // 可以推断
    let (_x, y, _z) = _tup1; // 解构
    let (_, _, _z) = _tup1; // 解构 可以用 _ 替代不要的位置


    // 通过索引取值
    let five_hundred = _tup1.0;

    let six_point_four = _tup1.1;

    let one = _tup1.2;

    println!("one : {}", one);
    println!("The value of y is: {}", y);

    // 数组类型 所有元素的类型必须相同 且长度固定
    // 数组中的内容可以修改 但是数组的长度不能修改
    // 当你想要在栈（stack）而不是在堆（heap）上为数据分配空间（第四章将讨论栈与堆的更多内容），或者是想要确保总是有固定数量的元素时，数组非常有用。
    let g = [1, 2, 3, 4, 5];
    let months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "Dec"];

    println!("{}", months[0]);


    // 声明一个数组
    let a: [i32; 5]; // 只声明
    a = [1, 2, 3, 4, 5];
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    // println!("{}",months[99]); // 数组越界会 不能编译 nb
    // println!("{}", months[1 * 2 * 4 * 5]); // 数组越界会 不能编译
    let a = [1, 2, 3, 4, 5];
    let index = 10;

    let element = a[index]; // 不报错 但是运行时报错
}

// function 的名字 必须是 snake case
pub fn ch03_03_how_functions_work() {
    let a = 1;
    // let b = a = 1; // 不允许
    let c = a + 1;

    let x = 5;

    // 有返回值的才是表达式 才能成为赋值语句 的右值
    let y = { // { } 块 宏调用
        let x = 3;
        x + 1 // 这行 没有 ; 表示这是一个返回值
    };

    println!("The value of y is: {}", y);

    let z = five();
    let d = six(5);


    /// 方法只要在作用域中声明 就能够调用(不用先声明在前)
    fun_1();
    fn fun_1() {
        println!("Another function.");
    }
    fun_2();
    fun_1();
    another_function(5, 6);
    // 声明参数类型
    // 没有返回的话 返回的类型是 ()
    fn another_function(x: i32, y: i32) -> () {
        println!("The value of x is: {}", x);
        println!("The value of y is: {}", y);
    }


    // 如果返回值被用来赋值 那么就一定要 显示注解类型
    fn five() -> i32 {
        5
    }

    fn six(x: i32) -> i32 {
        x + 1
    }

    // rust 是基于表达式的
}

pub fn ch03_05_control_flow() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // rust 并不能自动转换 下面语句不能通过编译
    //    if number {
    //        println!("number was three");
    //    }

    // 在初始化变量时使用if
    let condition = true;
    let number = if condition { 5 } else { 6 };

    // 下列语句 不能通过编译
    // 变量 赋值只能出现一种类型的可能
    //    let condition = true;
    //
    //    let number = if condition {
    //        5
    //    } else {
    //        "six"
    //    };


    // 一直循环到 有明确的退出为止
    let mut a = 1;
    loop {
        if a == 5 { break; }
        println!("again!");
        a = a + 1
    }


    // 从 loop 中返回数值
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // break 关键字 之后的是 返回的数值
        }
    };
    println!("The result is {}", result);

    // while 循环同 js
    let mut number = 3;

    while number != 0 {
        // 如果在这里使用 校验的值作为 数组的下标
        // 编译器是不能判断 会不会越界 而添加更多的 代码来校验并抛出可能的错误
        println!("{}!", number);

        number = number - 1;
    }

    // 最推荐的 循环 for 有效避免 数据越界
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    // 使用 range 来不用声明新的变量 使用循环的次数
    // .rev 翻转 可迭代的范围数据
    for number in (1..4).rev() {≤
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}

fn fun_2() {}