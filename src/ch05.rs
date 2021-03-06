#![allow(unused_variables)] // 不对 未使用的变量 warning

pub fn ch05_01_defining_structs() {
    // 定义并实例化结构体
    // 定义结构体，需要使用 struct 关键字并为整个结构体提供一个名字。
    // 结构体的名字需要描述它所组合的数据的意义。
    // 接着，在大括号中，定义每一部分数据的名字和类型，我们称为 字段（field）。
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // 要更改结构体中的值，如果结构体的实例是可变的，我们可以使用点号并为对应的字段赋值。
    let mut user2 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user2.email = String::from("anotheremail@example.com");

    // 变量名 如果 与 字段 同名可以使用简写 同es6
    fn build_user(email: String, username: String) -> User {
        User {
            email,
            username,
            active: true,
            sign_in_count: 1,
        }
    }

    // 使用结构体更新语法 从其他实例创建实例
    // 不同于js 运算符是 ..  rust 先来的属性不能被覆盖 只取还没有值的属性
    // 由于都必须 实现User 结构 所以最多只可能有一个 展开的变量
    {
        let user2 = User {
            email: String::from("another@example.com"),
            username: String::from("anotherusername567"),
            ..user1
        };

        println!("user2.email : {}", user2.email);
        println!("user2.active : {}", user2.active);

        /*
            结果
            {
                email: String::from("another@example.com"),
                username: String::from("anotherusername567"),
                active: user1.active,
                sign_in_count: user1.sign_in_count,
            };
        */
    }

    // 使用没有命名字段的元组结构体来创建不同的类型 （tuple structs）
    // 既然是元祖 当然可以包含不同的类型
    // 注意就算是两个 元祖结构体的每个类型都相同,也是连个不同的类型 (这是元祖的特性啦)
    {
        struct Color(i32, i32, i32);
        struct Point(i32, i32, i32);
        struct AnotherTupleStruct(i8, i16, i32);

        let black = Color(0, 0, 0);
        let origin = Point(1, 0, 0);
        // black 和 origin 类型不同  ; origin 并不能传递 类型是 Color 的

        // 取值和 元祖一样
        // println!("{}", origin.0);
        // println!("{}", origin.1);

        // todo 怎么没法解构呢 x 没法判断类型
        let (x) = &origin;
    }

    // 没有任何字段类单元结构体
    // 称为 类单元结构体

    struct NoDataStruct;
    // 因为它们类似于 ()，即 unit 类型。

    // 结构体数据的所有权
    {
        // struct User {
        //     username: &str,
        //     // 报错 需要生命周期
        //     email: &str,
        //     // 报错 需要生命周期
        //     sign_in_count: u64,
        //     active: bool,
        // }

        // 需要添加所有权
        struct User<'a> {
            username: &'a str,
            email: &'a str,
            sign_in_count: u64,
            active: bool,
        }

        let user1 = User {
            email: "someone@example.com",
            username: "someusername123",
            active: true,
            sign_in_count: 1,
        };
    }
}

pub fn ch05_02_example_structs() {
    // 需求
    // rectangles 的二进制程序，它获取以像素为单位的长方形的宽度和高度，
    // 并计算出长方形的面积

    // 原版
    {
        fn main() {
            let width1 = 30;
            let height1 = 50;

            println!(
                "The area of the rectangle is {} square pixels.",
                area(width1, height1)
            );
        }

        fn area(width: u32, height: u32) -> u32 {
            width * height
        }

        main();
    }
    // 使用元组 重构
    {
        fn main() {
            let rect1 = (30, 50);
            println!(
                "The area of the rectangle is {} square pixels.",
                area(rect1)
            )
        }

        fn area(dimensions: (u32, u32)) -> u32 {
            dimensions.0 * dimensions.1
        }
    }

    // 使用结构体重构
    {
        // Rust 为我们提供了很多可以通过 derive 注解来使用的 trait，他们可以为我们的自定义类型增加实用的行为。
        // 附录 C 中列出了这些 trait 和行为。
        // 第十章会介绍如何通过自定义行为来实现这些 trait，同时还有如何创建你自己的 trait。
        #[derive(Debug)] // 这个注解 只能作用域 struct 上
        struct Rectangle {
            width: u32,
            height: u32,
        }
        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };

        println!("{:?}", rect1);
        println!(
            "The area of the rectangle is {} square pixels.",
            area(&rect1)
        );

        fn area(rectangle: &Rectangle) -> u32 {
            rectangle.width * rectangle.height
        }
    }
}

pub fn ch05_03_method_syntax() {
    // 定义方法
    {
        // 相当于 js 类的方法
        #[derive(Debug)]
        struct Rectangle {
            width: u32,
            height: u32,
        }

        // 为了使函数定义于 Rectangle 的上下文中，我们开始了一个 impl 块（impl 是 implementation 落实 实现 的缩写）
        // 并将签名中的第一个（在这里也是唯一一个）参数和函数体中其他地方的对应参数改成 self
        impl Rectangle {
            fn area(&self) -> u32 {
                // &self 当然是借用的 不允许修改原始的实例
                self.width * self.height
            }

            // 修改结构体的属性
            fn set_height(&mut self, new_value: u32) {
                self.height = new_value
            }

            fn set_weight(&mut self, new_value: u32) {
                self.width = new_value
            }
            // self 作为第一个参数来使方法获取实例的所有权是很少见的；
            // 这种技术通常用在当方法将 self 转换成别的实例的时候，这时我们想要防止调用者在转换之后使用原始的实例。
            fn test1(self) {}
        }

        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };
        println!(
            "The area of the rectangle is {} square pixels.",
            // Rust 有一个叫 自动引用和解引用（automatic referencing and dereferencing）的功能。
            // 方法调用是 Rust 中少数几个拥有这种行为的地方。
            // 当使用 object.something()
            // 调用方法时，Rust 会自动为 object 添加 &、&mut 或 * 以便使 object 与方法签名匹配。
            // 也就是说，这些代码是等价的：
            // p1.distance(&p2);
            //(&p1).distance(&p2);
            rect1.area()
        );

        let mut rect2 = Rectangle {
            width: 30,
            height: 50,
        };
        rect2.set_height(100);
        println!("rect2 height : {}", rect2.height)
    }

    // 带有更多参数的方法
    {
        #[derive(Debug)]
        struct Rectangle {
            width: u32,
            height: u32,
        }

        impl Rectangle {
            fn area(&self) -> u32 {
                // &self 当然是借用的 不允许修改原始的实例
                self.width * self.height
            }
            fn can_hold(&self, other: &Rectangle) -> bool {
                self.width > other.width && self.height > other.height
            }
        }

        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };
        let rect2 = Rectangle {
            width: 10,
            height: 40,
        };
        let rect3 = Rectangle {
            width: 60,
            height: 45,
        };

        println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
        println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    }

    // 关联函数
    // impl 块的另一个有用的功能是：允许在 impl 块中定义 不 以 self 作为参数的函数。
    // 这被称为 关联函数（associated functions），因为它们与结构体相关联。
    // 它们仍是函数而不是方法，因为它们并不作用于一个结构体的实例。
    // 类似于 js 的静态方法
    {
        #[derive(Debug)]
        struct Rectangle {
            width: u32,
            height: u32,
        }

        impl Rectangle {
            fn square(size: u32) -> Rectangle {
                Rectangle {
                    width: size,
                    height: size,
                }
            }
        }
        // 使用结构体名和 :: 语法来调用这个关联函数
        println!("{:?}", Rectangle::square(100)); // 关联方法 的调用方式
    }

    // 多个impl 块
    {
        #[derive(Debug)]
        struct Rectangle {
            width: u32,
            height: u32,
        }

        impl Rectangle {
            fn square(size: u32) -> Rectangle {
                Rectangle {
                    width: size,
                    height: size,
                }
            }
        }

        impl Rectangle {
            fn area(&self) -> u32 {
                self.width * self.height
            }
        }

        impl Rectangle {
            fn can_hold(&self, other: &Rectangle) -> bool {
                self.width > other.width && self.height > other.height
            }
        }
    }
}
