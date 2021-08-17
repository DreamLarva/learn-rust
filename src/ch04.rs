#![allow(unused_variables)]

// 不对 未使用的变量 warning
/// 对于rust 一个值在 堆上或者在栈上很重要,这影响了 rust 所有权的相关操作.
///
/// 栈和堆都是代码可用的内存,但是他们结构并不相同.
/// 栈遵循先进先出,是不可能直接在栈中或者底部的.所有存储在栈的数据必须知道类型和长度.
/// 可变长度的数据需要存在堆中.在堆中储存数据,会申请以一定的空间,内存分配器会找一块够啊大的内存
/// 然后注明这块内存已经被使用了,然后返回指针指向内存的位置.从栈中使用堆的数据不需要再分配内存
/// 因为指针是也确定的类型,且长度一定的.
///
/// 当然在栈上的数据 速度快于堆,因为堆不会需要内分配器不需要寻找新的位置来安排内存,
/// 栈中的数据的存储位置都在栈顶
///
/// 当你要用一个方法,所有要传递给方法的值(栈上的,堆上的)和方法中自己的值都会被push到栈上
/// 方法调用结束时会被pop出去
///
/// 所有权 原则
/// 1. rust 中的每个值都有一个变量 成为 所有者
/// 2. 同时只能有一个所有者
/// 3. 当所有个离开作用域,值就会被丢弃
pub fn ch04_01_what_is_ownership() {
    {
        // s is not valid here, it’s not yet declared
        let s = "hello"; // s is valid from this point forward
                         // do stuff with s
    } // this scope is now over, and s is no longer valid
      // s 在作用域中 合法 直到离开作用域
      // s = ""; // 这里调用就报错了

    // 用String类型理解 所有权,因为String是存储在堆上的
    // 字面量声明 声明的字符串是不能修改的 类型为 &str
    //  let s0 = "abc";
    // 使用 String::from (注意有 mut) 可以修改 类型为 String
    //   let mut s1 = String::from("hello");

    // 内存和分配
    // 对于字符串字面量,在编译的时候就知道了,所以执行的时候执行的是直接是编译好的硬编码的内容
    // 所以字符串里面量快且方便,但是不可修改
    // 对于 String 类型,为了可变,会在堆上分配一块内存,这在编译的时候是不知道大小的
    //  1. 运行时才会申请内存
    //  2. 我们还需要一在搞完Sting 返回内存给内存分配器
    // 第一部分在我们调用 String::from 的时候,rust自动帮我们搞定了
    // 第二部分就不太一样了,有GC的语言会自动回收没有只用的内存.没有GC的,我们就需要手动的
    // 在需要释放内存的时候释放,时机不能早也不能晚,也不能释放多次

    let x = 5; // 对于字面量大小已经定了
    let y = x; // 所以栈上面有 2 个 5
    println!("{}, {}", x, y); // x y 都可以正常使用

    // 数据和变量的交互操作: move
    // 下列代码 报错
    let s1 = String::from("hello");
    let s2 = s1; // 将s1 移动(move)到 s2 上 同时废弃了 s1
                 // println!("{}, world!", s1); // s1 现在并没有值 所以报错了

    // 数据和变量的交互操作:clone 克隆(更加耗时)
    // 使用 s1 关键字 现在s1 和 s2 各有一份内容相同 但是互不相干的内容
    let mut s1 = String::from("hello");
    let mut s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    s1.push_str(", world!");
    s2.push_str(", hell!");
    println!("s1 = {}, s2 = {}", s1, s2);

    // 仅供栈上的数据的操作 : 拷贝 Copy
    // 在栈 上面的数据 是直接拷贝的 因为栈上面的数据是可以知道 内存的大小的
    // 这些类型的值可以 被拷贝
    // 1. 所有整数类型，比如 u32。
    // 2. 布尔类型，bool，它的值是 true 和 false。
    // 3. 所有浮点数类型，比如 f64。
    // 4. 字符类型，char。
    // 5. 元组，当且仅当其包含的类型也都是 Copy(上面1~4) 的时候。比如，(i32, i32) 是 Copy 的，但 (i32, String) 就不是。
    let x = 5;
    let y = x; // y 拷贝了 x 的值 在函数结束的时候 就各自清理各自的内存

    println!("x = {}, y = {}", x, y);

    // 所有权 与 函数
    {
        fn main() {
            let s = String::from("hello"); // s 进入作用域

            takes_ownership(s); // s 的值移动到函数里 ...
                                // ... 所以到这里不再有效

            let x = 5; // x 进入作用域

            makes_copy(x); // x 应该移动函数里，
                           // 但 i32 是 Copy 的，所以在后面可继续使用 x
        } // 这里, x 先移出了作用域，然后是 s。但因为 s 的值已被移走，
          // 所以不会有特殊操作

        fn takes_ownership(some_string: String) {
            // some_string 进入作用域
            println!("{}", some_string);
            println!("{}", &some_string); // 为啥 println! 不会消耗 所有权呢, 因为他是一个宏?
        } // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放

        fn makes_copy(some_integer: i32) {
            // some_integer 进入作用域
            println!("{}", some_integer);
        } // 这里，some_integer 移出作用域。不会有特殊操作
    }

    // 返回值 与 作用域
    {
        fn main() {
            let s1 = gives_ownership(); // gives_ownership 将返回值
                                        // 移给 s1

            let s2 = String::from("hello"); // s2 进入作用域

            let s3 = takes_and_gives_back(s2); // s2 被移动到

            // takes_and_gives_back 中,
            // 它也将返回值移给 s3
        } // 这里, s3 移出作用域并被丢弃。
          // s2 也移出作用域，但已被移走，所以什么也不会发生。
          // s1 移出作用域并被丢弃

        fn gives_ownership() -> String {
            // gives_ownership 将返回值移动给
            // 调用它的函数

            let some_string = String::from("hello"); // some_string 进入作用域.

            some_string // 返回 some_string 并移出给调用的函数
        }

        // takes_and_gives_back 将传入字符串并返回该值
        fn takes_and_gives_back(a_string: String) -> String {
            // a_string 进入作用域
            a_string // 返回 a_string 并移出给调用的函数
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
        fn calculate_length(s: &String) -> usize {
            // s 是对 String 的引用
            s.len()
        } // 这里，s 离开了作用域。但因为它并*不拥有*引用值的所有权，
          // 所以什么也不会发生

        // 声明就报错
        // 因为 *借用* 的值, (默认）不允许修改引用的值。
        // fn change(some_string: &String) {
        //     some_string.push_str(", world");
        // }
    }

    // 可变引用
    // 如果引用的变量 可变 那么借用的之后 也是可变的
    // 不过可变引用有一个很大的限制：在特定作用域中的特定数据有且只有一个可变引用(避免数据竞争)
    // 可能引起数据竞争的 行为:
    //  两个或更多指针同时访问同一数据。
    //  至少有一个指针被用来写入数据。
    //  没有同步数据访问的机制。
    {
        fn main() {
            let mut s = String::from("hello");

            // 可以两个 因为是同步的 且不是同时保有 s 的
            change(&mut s);
            change(&mut s);

            println!("{}", s)
        }

        main();

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
    // 我们 也不能在拥有不可变引用的同时拥有可变引用。
    // 因为已经使用的不可变引用的 部分 一定不希望出现 在可变引用使值发生改变
    {
        let mut s = String::from("hello");

        let r1 = &s; // no problem
        let r2 = &s; // no problem
        let r3 = &mut s; // BIG PROBLEM 编译和检查时并不会报错

        // println!("{}, {}, and {}", r1, r2, r3); // error
    }
    // 在具有指针的语言中，很容易通过释放内存时保留指向它的指针而错误地生成一个 悬垂指针（dangling pointer），所谓悬垂指针是其指向的内存可能已经被分配给其它持有者。
    // 相比之下，在 Rust 中编译器确保引用永远也不会变成悬垂状态：当你拥有一些数据的引用，编译器确保数据不会在其引用之前离开作用域。
    {
        // fn main() {
        //     let reference_to_nothing = dangle();
        // }
        //
        // fn dangle() -> &String {
        //     let s = String::from("hello");
        //
        //     &s
        //     // s 到这里离开作用域 被丢弃
        //     // 这样引用的 s 也失效了
        // }
    }
}

pub fn ch04_03_slices() {
    {
        // 另一个**没有所有权**的数据类型是 slice。
        // slice 允许你引用集合中一段连续的元素序列，而不用引用整个集合。

        // 编写一个函数，该函数接收一个字符串，并返回在该字符串中找到的第一个单词。
        // 如果函数在该字符串中并未找到空格，则整个字符串就是一个单词，所以应该返回整个字符串。
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
        // s 和 word 并没有同步,这就可能存在bug的隐患
        println!("{},{}", s, word)
    }
    // 字符串 slice（string slice）
    {
        let s = String::from("hello world");
        // start..end 语法代表一个以 start 开头并一直持续到但不包含 end 的 range
        // 如果需要包含 end，可以使用 ..=
        // 从头开始可以不写 0 如 &s[..5] 为 0 ~ 5
        // 到尾部结束 可以不写右侧的数字 如果 &[5..] 为 5 到 尾部
        // 都不写 就是 取全部 [..]
        let hello = &s[0..5];
        let world = &s[6..11];

        // 字符串 slice range 的索引必须位于有效的 UTF-8 字符边界内，
        // 如果尝试从一个多字节字符的中间位置创建字符串 slice，则程序将会因错误而退出。
        // 出于介绍字符串 slice 的目的，本部分假设只使用 ASCII 字符集；
        // 第八章的 “使用字符串存储 UTF-8 编码的文本” 部分会更加全面的讨论 UTF-8 处理问题。
    }
    {
        // 传入切片类型 返回 切片
        let mut s = String::from("hello world");

        fn first_word(s: &String) -> &str {
            // & str 切片类型
            let bytes = s.as_bytes();

            for (i, &item) in bytes.iter().enumerate() {
                if item == b' ' {
                    return &s[0..i]; // 返回切片
                }
            }

            &s[..] // 返回切片
        }
        let word = first_word(&s);
        // 当拥有某值的不可变引用时，就不能再获取一个可变引用。
        // 因为 clear 需要清空 String，它尝试获取一个可变引用。Rust不允许这样做，因而编译失败。
        // s.clear();

        println!("the first word is: {}", word);
    }
    {
        // 传入切片类型 返回 切片
        fn first_word(s: &str) -> &str {
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
        let word = first_word(&my_string);

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
