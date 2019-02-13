// mod 关键字 定义模块 可嵌套
mod sound {
    pub mod instrument {
        // 使用pub 关键字 使模块成为公有
        pub fn clarinet() {
            // 使用super 开始相对路径 这么做类似于文件系统中以 .. 开头
            // 该路径从 所在模块的 父 模块开始而不是当前模块。
            super::breathe_in();
        }
    }

    fn breathe_in() {
        // 函数体
    }
}

mod plant {
    // 如果使结构体 公有但是结构体的字段仍然是 私有的
    // 可以在每一个字段的基准上 选择其是否是公有
    pub struct Vegetable {
        pub name: String,
        id: i32,
    }

    impl Vegetable {
        pub fn new(name: &str) -> Vegetable {
            Vegetable {
                name: String::from(name),
                id: 1,
            }
        }
    }
}

mod menu {
    //  将枚举设计为公有会使其所有成员公有
    pub enum Appetizer {
        Soup,
        Salad,
    }
}


mod performance_group {
    // 建立从其他mod 中的软连接
//    use crate::sound::instrument; // 更加推荐绝对路径 如果在 main.rs 中使用
    use self::super::sound::instrument; // 在独立的模块中

    // use super::sound::instrument;

    pub fn clarinet_trio() {
        instrument::clarinet();
        instrument::clarinet();
        instrument::clarinet();
    }
}

// 在 mod sound2 后使用分号而不是代码块告诉 Rust 在另一个与模块同名的文件中加载模块的内容。
// 从文件引入的mod 只能放在最外层的作用域下
mod sound1;


// 将文件分割进不同的文件夹
mod sound2; // 需要 sound2.rs 在其中包含 sound2 文件夹内容 模块内功


pub fn main() {
// 路径来引用模块树种的项
// 路径 可以有两种形式：
// 绝对路径（absolute path）从 crate 根开始，以 crate 名或者字面值 crate 开头。
// 相对路径（relative path）从当前模块开始，以 self、super 或当前模块的标识符开头。
// 绝对路径和相对路径都后跟一个或多个由双冒号（::）分割的标识符。 类似于 /

// 绝对路径
//    crate::sound::instrument::clarinet(); // main.rs 中路径
    self::sound::instrument::clarinet(); // 独立模块中的路径

// 相对路径
    sound::instrument::clarinet();

    // 另外注意因为 plant::Vegetable 有私有字段，需要提供一个公有的关联函数来构建 Vegetable 的实例（这里使用了传统的名称 new）。
    // 如果 Vegetable 没有提供这么一个函数，
    // 我们就不能在 main 中创建 Vegetable 的实例，因为在 main 中不允许设置私有字段 id 的值。
    let mut v = plant::Vegetable::new("squash");
    v.name = String::from("butternut squash");
    println!("{} are delicious", v.name); // name 字段是公有的

    // 如果将如下行取消注释代码将无法编译:
    // println!("The ID is {}", v.id); // id 字段是私有的

    let order1 = menu::Appetizer::Soup;
    let order2 = menu::Appetizer::Salad;

    // 使用use 关键字将名称引入作用域
    // use crate::sound::instrument; // 在main.js 中使用的路劲
     use self::sound::instrument; // 在独立的mod 中使用的路劲

    // 在作用域中增加 use 和 路径类似于在文件系统中创建软连接(符号链接,symbolic link).
    // 通过crate根增加 use crate::sound::instrument，现在 instrument 在作用域中就是有效的名称了，如同它被定义于 crate 根一样。
    instrument::clarinet();


    // use 函数路径使用习惯 vs 其他项
    // use crate::sound::instrument::clarinet; // 并不推荐
    // 对于函数来说，通过 use 指定函数的父模块接着指定父模块来调用方法被认为是习惯用法。

    use std::collections::HashMap; // 引入HashMap 的习惯用法 而不是 use std::collections; 在使用 collections::HashMap


    // 不允许在一个作用域内容 引入两个相同的 模块
//     use std::fmt::Result;
//     use std::io::Result;


    // 将两个不同父模块的 Result 类型引入作用域并引用它们
    {
//        use std::fmt;
//        use std::io;
//
//        fn function1() -> fmt::Result {
//        }
//        fn function2() -> io::Result<()> {
//        }
    }

    // 使用 as 关键字 重命名引入作用域的类型
    {
        use std::fmt::Result;
        use std::io::Result as IoResult;

        // IoResult ...
    }

    // 通过pub use 重导出名称
    // 当使用 use 关键字将名称导入作用域时，在新作用域中可用的名称是私有的。如果希望调用你编写的代码的代码能够像你一样在其自己的作用域内引用这些类型，可以结合 pub 和 use。
    {
        mod sound {
            pub mod instrument {
                pub fn clarinet() {
                    // 函数体
                }
            }
        }

        mod performance_group {
            // pub use crate::sound::instrument; // 在main.js 中使用的路劲
            pub use self::super::sound::instrument; // 在独立模块中使用的路劲

            pub fn clarinet_trio() {
                instrument::clarinet();
                instrument::clarinet();
                instrument::clarinet();
            }
        }

        performance_group::clarinet_trio();
        performance_group::instrument::clarinet();
    }

    // 使用外部包
    {
        use rand::Rng;
        let secret_number = rand::thread_rng().gen_range(1, 101);
        println!("secret_number : {}", secret_number)
    }

    // 使用嵌套路径来消除 大量的use 行
    // 修改前 嵌套的路径
    {
        use std::cmp::Ordering;
        use std::io;
    }
    // 修改后
    {
        use std::{cmp::Ordering, io};
    }

    // 修改前 其中一个是另一个的子路径
    {
        use std::io;
        use std::io::Write;
    }
    // 修改后
    {
        use std::io::{self, Write};
    }

    // 通过glob 运算符将所有的公有定义引入作用域
    {
        // 使用 glob 运算符时请多加小心！如此难以推导作用域中有什么名称和它们是在何处定义的。
        use std::collections::*;
    }

    // 通过模块分割近不同文件
    sound2::instrument::clarinet()

}
