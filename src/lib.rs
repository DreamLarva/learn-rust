// lib.rs 使用写 库 就是为了写给其他人调用

pub fn setup() {
    // 编写特定库测试所需的代码
}

// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//     }
// }

mod front_of_house;

fn serve_order() {}

fn main() {}

// 使用 super 起始的相对路径
mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        // super 进入父模块
        super::serve_order();
    }

    fn cook_order() {}

    // 创建公有的结构体和枚举
    // 我们还可以使用 pub 来设计公有的结构体和枚举，不过有一些额外的细节需要注意。
    // 如果我们在一个结构体定义的前面使用了 pub ，这个结构体会变成公有的，但是这个结构体的字段仍然是私有的。
    // 我们可以根据情况决定每个字段是否公有。
    pub struct Breakfast {
        pub toast: String,
        // 只有这个字段 是公有的
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // 将枚举设为公有，则它的所有成员都将变为公有。
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    // 第一种方式，我们在 eat_at_restaurant 中调用 add_to_waitlist 函数，使用的是绝对路径。
    // add_to_waitlist 函数与 eat_at_restaurant 被定义在同一 crate 中，
    // 这意味着我们可以使用 crate 关键字为起始的绝对路径。

    // 在 crate 后面，我们持续地嵌入模块，直到我们找到 add_to_waitlist。
    // 你可以想象出一个相同结构的文件系统，我们通过指定路径 /front_of_house/hosting/add_to_waitlist 来执行 add_to_waitlist 程序。
    // 我们使用 crate 从 crate 根开始就类似于在 shell 中使用 / 从文件系统根开始。
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // 第二种方式，我们在 eat_at_restaurant 中调用 add_to_waitlist，使用的是相对路径。这个路径以 front_of_house 为起始，
    // 这个模块在模块树中，与 eat_at_restaurant 定义在同一层级。与之等价的文件系统路径就是 front_of_house/hosting/add_to_waitlist。
    // 以名称为起始，意味着该路径是相对路径。
    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    // 使用 use 将模块引入作用域
    // hosting 成为了 当前作用域中的有效路径了
    // 当然使用use 也需要检查 私有性
    use crate::front_of_house::hosting;

    pub fn eat_at_restaurant() {
        let order1 = back_of_house::Appetizer::Soup;
        let order2 = back_of_house::Appetizer::Salad;

        hosting::add_to_waitlist();
    }
}

// 使用父模块将两个具有相同名称的类型引入同一作用域
use std::fmt;
// use std::io;
// fn function1() -> fmt::Result {
//     // --snip--
// }
// fn function2() -> io::Result<()> {
//     // --snip--
// }

// 重命名引入的的名称
use std::io::Result as IoResult;
// fn function3() -> IoResult<()> {
//     // --snip--
// }

// 当使用 use 关键字将名称导入作用域时，在新作用域中可用的名称是私有的。
// 如果为了让调用你编写的代码的代码能够像在自己的作用域内引用这些类型，可以结合 pub 和 use。
// 可以重新组织当前模块中 引入的模块和代码 的导出结构
pub use crate::front_of_house::hosting;
pub use rand;

// 嵌套路径来消除大量的 use 行
// use std::cmp::Ordering;
// use std::io;
use std::{borrow, cmp::Ordering};

// use std::io;
// use std::io::Write;
use std::io::{self, Write}; // use io 本身 和 io::Write

// 如果希望将一个路径下 所有 公有项引入作用域，可以指定路径后跟 *
use std::collections::*;
