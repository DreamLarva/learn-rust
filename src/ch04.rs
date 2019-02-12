#![allow(unused_variables)] // 不对 未使用的变量 warning

pub fn ch04_01_what_is_ownership() {
    // 下列代码 报错
//    let s1 = String::from("hello");
//    let s2 = s1; // 将s1 移动(move)到 s2 上 同时废弃了 s1

//    println!("{}, world!", s1); // s1 现在并没有值 所以报错了

    // 克隆 clone
    // 使用 s1 关键字 现在s1 和 s2 各有一份内容相同 但是互不相干的内容
    let mut s1 = String::from("hello");
    let mut s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    s1.push_str(", world!");
    s2.push_str(", hell!");
    println!("s1 = {}, s2 = {}", s1, s2);


    // 拷贝 Copy
    // 在栈 上面的数据 是直接拷贝的 因为栈上面的数据是可以知道 内存的大小的
    // 这些类型的值可以 被拷贝
    // 1. 所有整数类型，比如 u32。
    // 2. 布尔类型，bool，它的值是 true 和 false。
    // 3. 所有浮点数类型，比如 f64。
    // 4. 字符类型，char。
    // 5. 元组，当且仅当其包含的类型也都是 Copy 的时候。比如，(i32, i32) 是 Copy 的，但 (i32, String) 就不是。
    let x = 5;
    let y = x; // y 拷贝了 x 的值 在函数结束的时候 就各自清理各自的内存

    println!("x = {}, y = {}", x, y);

    // 所有权 与 函数
    {
        fn main() {
            let s = String::from("hello");  // s 进入作用域

            takes_ownership(s);             // s 的值移动到函数里 ...
            // ... 所以到这里不再有效

            let x = 5;                      // x 进入作用域

            makes_copy(x);                  // x 应该移动函数里，
            // 但 i32 是 Copy 的，所以在后面可继续使用 x
        } // 这里, x 先移出了作用域，然后是 s。但因为 s 的值已被移走，
        // 所以不会有特殊操作

        fn takes_ownership(some_string: String) { // some_string 进入作用域
            println!("{}", some_string);
        } // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放

        fn makes_copy(some_integer: i32) { // some_integer 进入作用域
            println!("{}", some_integer);
        } // 这里，some_integer 移出作用域。不会有特殊操作
    }

    // 返回值 与 作用域
    {
        fn main() {
            let s1 = gives_ownership();         // gives_ownership 将返回值
            // 移给 s1

            let s2 = String::from("hello");     // s2 进入作用域

            let s3 = takes_and_gives_back(s2);  // s2 被移动到
            // takes_and_gives_back 中,
            // 它也将返回值移给 s3
        } // 这里, s3 移出作用域并被丢弃。s2 也移出作用域，但已被移走，
        // 所以什么也不会发生。s1 移出作用域并被丢弃

        fn gives_ownership() -> String {             // gives_ownership 将返回值移动给
            // 调用它的函数

            let some_string = String::from("hello"); // some_string 进入作用域.

            some_string                              // 返回 some_string 并移出给调用的函数
        }

        // takes_and_gives_back 将传入字符串并返回该值
        fn takes_and_gives_back(a_string: String) -> String { // a_string 进入作用域
            a_string  // 返回 a_string 并移出给调用的函数
        }
    }
}

pub fn ch04_02_references_and_borrowing() {
    // 引用 与 借用
    {
        // 使用 & 符号 进行引用; 不移动目标变量 到 新的变量上 ,但是给出一个引用目标变量的引用
        fn main() {
            let s1 = String::from("hello");

            // &s1 语法让我们创建一个 指向 值 s1 的引用，但是并不拥有它。
            // 因为并不拥有这个值，当引用离开作用域时其指向的值也不会被丢弃。
            let len = calculate_length(&s1);

            println!("The length of '{}' is {}.", s1, len);
        }

        // 我们将获取引用作为函数参数称为 借用（borrowing）
        // 这里 函数数 的 s 指向 传入的 s1
        fn calculate_length(s: &String) -> usize { // s 是对 String 的引用
            s.len()
        }// 这里，s 离开了作用域。但因为它并*不拥有*引用值的所有权，
        // 所以什么也不会发生

        // 声明就报错
        // 因为 *借用* 的值, (默认）不允许修改引用的值。
//        fn change(some_string: &String) {
//            some_string.push_str(", world");
//        }
    }

    // 可变引用
    // 如果引用的变量 可变 那么借用的之后 也是可变的
    // 不过可变引用有一个很大的限制：在特定作用域中的特定数据有且只有一个可变引用(避免数据竞争)
    // 可能引起数据竞争的 行为:
    // 两个或更多指针同时访问同一数据。
    // 至少有一个指针被用来写入数据。
    // 没有同步数据访问的机制。
    {
        fn main() {
            let mut s = String::from("hello");

            change(&mut s);
        }

        fn change(some_string: &mut String) {
            some_string.push_str(", world");
        }


        // 一如既往，可以使用大括号来创建一个新的作用域，以允许拥有多个可变引用，只是不能 同时 拥有：
        let mut s = String::from("hello");

        {
            let r1 = &mut s;
        } // r1 在这里离开了作用域，所以我们完全可以创建一个新的引用

        let r2 = &mut s;
    }
    // 我们 也 不能在拥有不可变引用的同时拥有可变引用。
    // 因为已经使用的不可变引用的 部分 一定不希望出现 在可变引用使值发生改变
    //
    {
//        let mut s = String::from("hello");
//
//        let r1 = &s; // no problem
//        let r2 = &s; // no problem
//        let r3 = &mut s; // BIG PROBLEM

//        println!("{}, {}, and {}", r1, r2, r3);
    }
    // 在具有指针的语言中，很容易通过释放内存时保留指向它的指针而错误地生成一个 悬垂指针（dangling pointer），所谓悬垂指针是其指向的内存可能已经被分配给其它持有者。
    // 相比之下，在 Rust 中编译器确保引用永远也不会变成悬垂状态：当你拥有一些数据的引用，编译器确保数据不会在其引用之前离开作用域。
    {
//        fn main() {
//            let reference_to_nothing = dangle();
//        }

//        fn dangle() -> &String {
//            let s = String::from("hello");
//
//            &s
//            // s 到这里离开作用域 被丢弃
//            // 这样引用的 s 也失效了
//        }
    }
}

pub fn ch04_03_slices() {
    {
        // 另一个没有所有权的数据类型是 slice。
        // slice 允许你引用集合中一段连续的元素序列，而不用引用整个集合。
        fn first_word(s: &String) -> usize {
            // 检查 String 中的值是否为空格，需要用 as_bytes 方法将 String 转化为字节数组
            let bytes = s.as_bytes();
            // 接下来，使用 iter 方法在字节数组上创建一个迭代器：
            // enumerate 将 数据包装成 (i, &item)  的元组
            for (i, &item) in bytes.iter().enumerate() {
                // 因为是借用  bytes.iter().enumerate() 的值 所以要使用 &
                if item == b' ' {
                    return i;
                }
            }

            s.len()
        }
        let mut s = String::from("hello world");
        let word = first_word(&s);
        s.clear();

        // s的值改变了  但是 word 的值 也就是 s的长度没有改变 所以逻辑上存在问题
//        println!("{},{}",s,word)
    }
    // 字符串 slice（string slice）
    {
        let s = String::from("hello world");
        // start..end 语法代表一个以 start 开头并一直持续到但不包含 end 的 range
        // 如果需要包含 end，可以使用 ..=
        let hello = &s[0..5];
        let world = &s[6..11];
    }
    {
        let s = String::from("hello world");

        fn first_word(s: &String) -> &str { // & str 切片类型
            let bytes = s.as_bytes();

            for (i, &item) in bytes.iter().enumerate() {
                if item == b' ' {
                    return &s[0..i]; // 返回切片
                }
            }

            &s[..] // 返回切片
        }
        let word = first_word(&s);
        // s.clear(); // 添加这句就会报错 因为 first_word 函数有一个 不可变的引用 并且返回一个切片 那么就不能再修改 s了

        println!("the first word is: {}", word);
    }
    {
        // 传入切片类型
        fn first_word(s: &str) -> &str { // & str 切片类型
            let bytes = s.as_bytes();

            for (i, &item) in bytes.iter().enumerate() {
                if item == b' ' {
                    return &s[0..i]; // 返回切片
                }
            }

            &s[..] // 返回切片
        }
        let my_string = String::from("hello world");

        // first_word 中传入 `String` 的 slice 相当于整个引用
        let word = first_word(&my_string[..]);

        let my_string_literal = "hello world";

        // first_word 中传入字符串字面值的 slice
        let word = first_word(&my_string_literal[..]);

        // 因为字符串字面值 **就是** 字符串 slice，
        // 这样写也可以，即不使用 slice 语法！
        let word = first_word(my_string_literal);
    }
    // 其他类型的切片
    // 字符串 slice，正如你想象的那样，是针对字符串的。不过也有更通用的 slice 类型。考虑一下这个数组：
    let a = [1, 2, 3, 4, 5];
    // 本来数组的长度 就不能改变
    let slice = &a[1..3];

    println!("{}", slice[1]);
}
