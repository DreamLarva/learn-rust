#![allow(unused_variables)] // 不对 未使用的变量 warning

pub fn ch03_02_data_types() {
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
    println!("guess : {}", guess);

    // 布尔类型
    let t = true;
    let f: bool = false; // 显式指定类型注解
    println!("{},{}", t, f);

    // 字符 单引号
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("{},{},{}", c, z, heart_eyed_cat);

    // 字符串
    let string = "123abc";
    println!("{}", string);

    // 元组类型
    let _tup1: (i32, f64, u8) = (500, 6.4, 1); // 可以设置多种类型
    let _tup2 = (500, 6.4, 1, '1'); // 可以推断
    let (_x, y, _z) = _tup1;


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

    // println!("{}",months[99]); // 数组越界会 不能编译
    // println!("{}", months[1 * 2 * 4 * 5]); // 数组越界会 不能编译
}

pub fn ch03_03_how_functions_work() {
    let a = 1;
    // let b = a = 1; // 不允许
    let c = a + 1;

    let x = 5;

    let y = { // { } 块 宏调用
        let x = 3;
        x + 1 // 这行 没有 ; 表示这是一个返回值
    };

    println!("The value of y is: {}", y);

    let z = five();
    let d = six(5);

    // 如果返回值被用来赋值 那么就一定要 显示注解类型
    fn five() -> i32 {
        5
    }

    fn six(x: i32) -> i32 {
        x + 1
    }
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
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}