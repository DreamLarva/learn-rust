#![allow(unused_variables)] // 不对 未使用的变量 warning


pub fn ch06_01_defining_an_enum() {
    {
        enum IpAddrKind {
            V4,
            // 枚举的成员
            V6,  // 枚举的成员
        }

        // 创建实例
        let four = IpAddrKind::V4;
        let six = IpAddrKind::V6;


        // 关联结构体
        struct IpAddr {
            kind: IpAddrKind,
            address: String,
        }

        let home = IpAddr {
            kind: IpAddrKind::V4,
            address: String::from("127.0.0.1"),
        };

        let loopback = IpAddr {
            kind: IpAddrKind::V6,
            address: String::from("::1"),
        };
    }


    // 简洁的关联方法
    {
        enum IpAddr {
            V4(u8, u8, u8, u8),
            V6(String),
        }
        let home = IpAddr::V4(127, 0, 0, 1);
        let loopback = IpAddr::V6(String::from("::1"));
    }
    // 枚举中放入结构体 也可以是另一个枚举
    {
        struct Ipv4Addr {
            // --snip--
        }

        struct Ipv6Addr {
            // --snip--
        }

        enum SomeEnum {
            A,
            B,
            C,
        }

        enum IpAddr {
            V4(Ipv4Addr),
            V6(Ipv6Addr),
            SomeEnum(SomeEnum),
        }
    }

    {
        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(i32, i32, i32),
        }
        struct QuitMessage; // 类单元结构体
    struct MoveMessage {
        x: i32,
        y: i32,
    }
        struct WriteMessage(String); // 元组结构体
    struct ChangeColorMessage(i32, i32, i32); // 元组结构体

        // 可以在枚举上定义方法
        impl Message {
            fn call(&self) {
                // 在这里定义方法体
            }
        }

        let m = Message::Write(String::from("hello"));
        m.call();
    }

    // Option 枚举 和 其对于空值的优势
    // rust 没有空值,不过它确实拥有一个可以编码存在编码存在或不存在概念的枚举
    /*
        enum Option<T> {
            Some(T),
            None,
        }
    */
    {
        // 如果使用 None 而不是 Some，需要告诉 Rust Option<T> 是什么类型的，
        // 因为编译器只通过 None 值无法推断出 Some 成员保存的值的类型。
        let some_number = Some(5);
        let some_string = Some("a string");

        let absent_number: Option<i32> = None;

        // 这句话会提示 编译错误 类型不同就不能相加
        // 提前声明 某些值可能为空 那么使用这些值的时候 就必须处理 为空的情况 才能正确编译
        // 所以没有提前准备 Option 申明的 运行时 还是可能会出现 空值错误的
        // some_number + 1;
    }
}

pub fn ch06_03_if_let() {
    // if let 简单控制流
    // if let 语法让我们以一种不那么冗长的方式结合 if 和 let，
    // 来处理只匹配一个模式的值而忽略其他模式的情况。
    let some_u8_value = Some(4);
    // 我们想要对 Some(3) 匹配进行操作但是不想处理任何其他 Some<u8> 值或 None 值。
    // 为了满足 match 表达式（穷尽性）的要求，
    // 必须在处理完这唯一的成员后加上 _ => ()，
    // 这样也要增加很多样板代码。
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (), // 没有佩佩到 Some(3) 就走了这个分支什么都不做
    }

    // 使用 if let
    // 一次 只匹配一种类型 并且忽略其他所有的值
    if let Some(3) = some_u8_value {
        println!("three");
    }
}
