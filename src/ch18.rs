use std::cell::Cell;

// 所有可能会用到模式的位置
pub fn ch18_01_all_the_places_for_patterns() {
    // match 分支

    // match Value {
    //  Pattern => EXPRESSION,
    //  Pattern => EXPRESSION,
    //  Pattern => EXPRESSION,
    // }

    // match 表达式必须是穷尽(exhaustive)的, 意为match表达式所有可能的值都必须被考虑到
    // 一个确保覆盖每个可能值的方法是在最后一个分支 使用捕获所有的模式:
    // 比如一匹配任何值的名称永远也不会失败,因此可以匹配所有匹配剩下情况
    // _ 可以匹配所有情况

    // if let 表达式
    // if let 主要用于编写等同于只关心一个情况的match语句.
    // if let 可以对应一个 可选的带有代码的 else 在 if let 中的模式不匹配时运行
    // 可以组合匹配 if let, else if 和 else if let表达式
    {
        let favourite_color: Option<&str> = None;
        let is_tuesday = false;
        let age: Result<u8, _> = "34".parse();

        if let Some(color) = favourite_color {
            println!("Using your favourite color,{},as the background", color);
        } else if is_tuesday {
            println!("Tuesday is green day!");
        } else if let Ok(age) = age {
            if age > 30 {
                println!("Using purple as the background color");
            } else {
                println!("Using orange as the background color");
            }
        } else {
            println!("Using blue as the background color");
        }
    }

    // while let 条件循环
    // 一个与 if let 结构类似的是 while let 条件循环,它允许只要模式匹配一直进行 while 循环.
    {
        let mut stack = Vec::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);
        while let Some(top) = stack.pop() {
            println!("{}", top);
        }
    }

    // for 循环
    {
        let v = vec!['a', 'b', 'c'];
        for (index, value) in v.iter().enumerate() {
            println!("{} is at index {}", value, index);
        }

        // a is at index 0
        // b is at index 1
        // c is at index 2
    }

    // let 语句
    // let PATTERN = EXPRESSION;
    {
        // x 是用模式代表"将匹配到的值绑定到变量x"
        let x = 5;
        let (x, y, z) = (1, 2, 3);

        // let (x, y) = (1, 2, 3); // error
        let (x, y, _) = (1, 2, 3);

        let (x, y) = (y, x);
        assert_eq!(x, 2);
        assert_eq!(y, 1);
    }

    // 函数参数
    {
        fn foo(x: i32) {
            // 代码
        }

        fn print_coordinates(&(x, y): &(i32, i32)) {
            println!("Current location:({},{})", x, y);

            let point = (3, 5);
            print_coordinates(&point);
        }
    }
}

// Refutability（可反驳性）: 模式是否会匹配失效
pub fn ch18_02_refutability() {
    // 模式有两种形态: refutable(可反驳的) 和 irrefutable(不可反驳的).
    // 能匹配任何传递的可能值的模式 被称为不可反驳的(irrefutable).
    // 一个例子就是 let x=5;语句中的x,因为x可以匹配任何值,所以不可能失败.
    // 对某些可能的值进行匹配会失败的模式称为 可反驳的(refutable).
    // 例子是 if let Some(x) = a_value 表达式中的Some(x);如果变量a_value 中的值是None 而不是Some,那么Some(x) 不匹配
}

// 所有的模式语法
pub fn ch18_03_pattern_syntax() {
    // 匹配字面量
    {
        let x = 1;
        match x {
            1 => println!("one"),
            2 => println!("two"),
            3 => println!("three"),
            _ => println!("anything"),
        }
    }
    // 匹配命名变量
    // 命名变量是匹配任何值的不可反驳模式
    // 因为match 会开始一个新作用域,match表达式中作为模式的一部分声明的变量会覆盖match结构外的同名变量,与所有变量一样.
    {
        let x = Some(5);
        let y = 10;
        match x {
            Some(50) => println!("Got 50"),

            // 匹配, 这个新的y 绑定会匹配任何Som额的值, 在这里就是 x 中的值
            Some(y) => println!("Matched,y = {:?}", y),
            _ => println!("Default case.x = {:?}", x),
        }

        println!("at the end: x = {:?}, y = {:?}", x, y);
    }

    // 多个模式
    // 在match 表达式中可以使用 | 语法匹配多个模式,它代表 (or) 的意思.
    {
        let x = 1;
        match x {
            1 | 2 => println!("one or two"),
            3 => println!("three"),
            _ => println!("anything"),
        }
    }

    {
        let x = 9;
        let message = match x {
            0 | 1 => "not many",
            2..=9 => "a few",
            _ => "lots"
        };

        assert_eq!(message, "a few");

        // Demonstration of pattern match order.
        struct S(i32, i32);

        match S(1, 2) {
            S(z @ 1, _) | S(_, z @ 2) => assert_eq!(z, 1),
            _ => panic!(),
        }
    }


    // 注意：使用 | 运算符的多个匹配可能会导致模式守卫及其必须多次执行的副作用。例如：
    {
        let i: Cell<i32> = Cell::new(0);
        match 1 {
            1 | _ if {
                i.set(i.get() + 1);
                false
            } => {}
            _ => {}
        }
        assert_eq!(i.get(), 2);
    }

    // 通过..= 匹配值的范围
    {
        let x = 5;
        match x {
            // 匹配 1 ~ 5
            1..=5 => println!("one through five"),
            _ => println!("something else"),
        }
    }

    // 解构分解值
    // 解构结构体
    {
        struct Point {
            x: i32,
            y: i32,
        }

        let p = Point { x: 0, y: 7 };
        let Point { x, y: b } = p;
        assert_eq!(0, x);
        assert_eq!(7, b);
    }
    {
        struct Point {
            x: i32,
            y: i32,
        }
        let p = Point { x: 0, y: 7 };
        match p {
            Point { x, y: 0 } => println!("On the x axis at {}", x),
            Point { x: 0, y } => println!("On the y axis at {}", y),
            Point { x, y } => println!("On neither axis:({},{})", x, y),
        }
        // On the y axis at 7
    }
    // @绑定后解构（Rust1.56新增）
    {
        #[derive(Debug)]
        struct Point {
            x: i32,
            y: i32,
        }

        // 等同于 解构 后 还保留完整的对象变量
        let p @ Point { x: px, y: py } = Point { x: 10, y: 23 };
        println!("x:{},y:{}", px, py);
        println!("{:?}", p);

        let point = Point { x: 10, y: 10 };
        if let p @ Point { x: 10, y } = point {
            println!("x is 10 and y is {} in {:?}", y, p);
        } else {
            println!("x is not 10");
        }
    }

    // 结构枚举
    {
        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(i32, i32, i32),
        }

        let msg = Message::ChangeColor(0, 160, 255);

        match msg {
            Message::Quit => {
                println!("The Quit variant has no data to destructure.")
            }
            Message::Move { x, y } => {
                println!("Move in the x direction {} an in the y direction {}", x, y);
            }
            Message::Write(text) => {
                println!("Text message: {}", text);
            }
            Message::ChangeColor(r, g, b) => {
                println!("Change the color to red {}, green {} and blue {}", r, g, b);
            }
        }

        // Change the color to red 0, green 160 and blue 255
    }
    // 解构嵌套的结构体和枚举
    {
        enum Color {
            Rgb(i32, i32, i32),
            Hsv(i32, i32, i32),
        }

        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(Color),
        }

        let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));
        match msg {
            Message::ChangeColor(Color::Rgb(r, g, b)) => {
                println!(
                    "Change the color the red {}, green {}, and blue {}",
                    r, g, b
                )
            }
            Message::ChangeColor(Color::Hsv(h, s, v)) => {
                println!(
                    "Change the color the hue {}, saturation {}, and value {}",
                    h, s, v
                );
            }
            _ => (),
        };

        // Change the color the hue 0, saturation 160, and value 255
    }

    // 解构机构体 和元组
    {
        struct Point {
            x: i32,
            y: i32,
        }
        let ((fleet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
    }

    // 忽略模式中的值
    // 有时忽略模式中的一些值是有用的,比如match 中最后捕获全部情况的分支世界上没有做任何事,但是它确实对所有剩余情况负责.
    // 有一些简单的方法可以忽略模式中全部或部分值:
    // 使用 _ 模式
    // 使用一个下滑线开始的名称
    // 使用 .. 忽略所剩余的值

    // 使用 _ 忽略整个值
    {
        // 忽略参数
        fn foo(_: i32, y: i32, _p: i32) {
            println!("This code only uses the y parameter: {}", y);
        }

        foo(3, 4, 5);
    }

    // 使用嵌套 _ 忽略部分值
    {
        let mut setting_value = Some(5);
        let new_setting_value = Some(10);

        match (setting_value, new_setting_value) {
            (Some(_), _) => {
                println!("Can't overwrite an existing customized value");
            }
            _ => setting_value = new_setting_value,
        }

        println!("setting is {:?}", setting_value);

        let numbers = (2, 4, 6, 8, 16, 32);
        match numbers {
            (first, _, third, _, fifth, _) => {
                println!("Some number: {},{},{}", first, third, fifth);
            }
        }
    }

    // 用..忽略剩余值
    {
        struct Point {
            x: i32,
            y: i32,
            z: i32,
        }

        let origin = Point { x: 0, y: 0, z: 0 };
        match origin {
            Point { x, .. } => println!("x is {}", x),
        }
    }

    {
        let number = (2, 4, 8, 16, 32);
        match number {
            // 忽略 除 首尾 之外的值
            (first, .., last) => {
                println!("Some number: {} ,{}", first, last);
            }
        }
    }

    // 匹配首尾提供额外条件
    {
        // 匹配守卫(match guard)是一个指定与 match 分支模式之外的额外 if 条件,它也必须被满足才能选择此分支.
        // 匹配守卫用于表达比单独的模式所能允许的更为复杂的情况.
        {
            let num = Some(4);
            match num {
                // 先匹配 Some  然后 匹配守卫
                Some(x) if x < 5 => println!("less than five: {}", x),
                Some(x) => println!("{}", x),
                None => (),
            }
        }

        {
            let x = Some(5);
            let y = 10;

            match x {
                Some(50) => println!("Got 50"),
                Some(n) if n == y => println!("Matched , n = {}", n),
                _ => println!("Default case, x = {:?}", x),
            }

            println!("at the end: x = {:?}, y ={}", x, y)
        }
    }

    // 在匹配守卫中 使用 | 运算符 来指定多个模式,同时匹配守卫的条件会作用于所有的模式.
    {
        let x = 4;
        let y = true;
        match x {
            // x 是4 5 6 之一 且 y 为 true 时 匹配
            4 | 5 | 6 if y => println!("yes"),
            _ => println!("no"),
        }
    }

    // @绑定
    // at运算符(@)允许我们在创建一个存放值的变量的同时测试其值是否匹配模式.
    {
        enum Message {
            Hello { id: i32 },
        }

        let msg = Message::Hello { id: 5 };

        match msg {
            // 测试 Message::Hello 的 id 字段是否位于 3..=7 范围内，
            // 同时也希望能将其值绑定到 id_variable 变量中以便此分支相关联的代码可以使用它
            Message::Hello {
                id: id_variable @ 3..=7,
            } => {
                println!("Found an id in range {}", id_variable)
            }
            Message::Hello { id: 10..=12 } => {
                println!("Found an id in another range")
            }
            Message::Hello { id } => {
                println!("Found some other id: {}", id)
            }
        }
    }
}
