// lib.rs 使用写 库 就是为了写给其他人调用
//! # 注释包含项的结构
//!
//! 这些文案会出现在 目录的部分
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convenient.

/// 缩略标题
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

// use crate::front_of_house::hosting::add_to_waitlist;

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

// 个函数返回一个传递给程序的命令行参数的 迭代器（iterator）
use std::env;
use std::error::Error;
use std::fs;
use std::process;

// 二进制项目的关注分离
// main 函数负责多个任务的组织问题在许多二进制项目中很常见。
// 所以 Rust 社区开发出一类在 main 函数开始变得庞大时进行二进制程序的关注分离的指导性过程。
// 这些过程有如下步骤：
//      将程序拆分成 main.rs 和 lib.rs 并将程序的逻辑放入 lib.rs 中。
//      当命令行解析逻辑比较小时，可以保留在 main.rs 中。
//      当命令行解析开始变得复杂时，也同样将其从 main.rs 提取到 lib.rs 中。
// 经过这些过程之后保留在 main 函数中的责任应该被限制为：
//      使用参数值调用命令行解析逻辑
//      设置任何其他的配置
//      调用 lib.rs 中的 run 函数
//      如果 run 返回错误，则处理这个错误

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        // 使用 迭代器
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

// Box<dyn Error> 意味着函数会返回实现了 Error trait 的类型，
// 不过无需指定具体将会返回的值的类型。这提供了在不同的错误场景可能有不同类型的错误返回值的灵活性。
// 这也就是 dyn，它是 “动态的”（“dynamic”）的缩写。
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // 直接给文件名 默认是从 根目录 开始找
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

//  &String 会自动转成 &str  两个参数手动处理所有权
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive
Pick three.";
        assert_eq!(vec!["safe, fast, productive"], search(query, contents))
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
trust me.";
        assert_eq!(
            vec!["Rust:", "trust me."],
            search_case_insensitive(query, contents)
        )
    }
}
/// 编写有用的文档注释
/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = the_rust_programming_language::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
// 可以运行 cargo doc 来生成这个文档注释的 HTML 文档。
// 这个命令运行由 Rust 分发的工具 rustdoc 并将生成的 HTML 文档放入 target/doc 目录。
// 运行 cargo doc --open 会构建当前 crate 文档（同时还有所有 crate 依赖的文档）的 HTML 并在浏览器中打开

// # Examples Markdown 标题在 HTML 中创建了一个以 “Examples” 为标题的部分
// Panics：这个函数可能会 panic! 的场景。并不希望程序崩溃的函数调用者应该确保他们不会在这些情况下调用此函数。
// Errors：如果这个函数返回 Result，此部分描述可能会出现何种错误以及什么情况会造成这些错误，这有助于调用者编写代码来采用不同的方式处理不同的错误。
// Safety：如果这个函数使用 unsafe 代码（这会在第十九章讨论），这一部分应该会涉及到期望函数调用者支持的确保 unsafe 块中代码正常工作的不变条件（invariants）。

// 文档注释作为测试
// 运行 cargo test 所有文档里面的案例 也会像测试一样运行
// 牛逼啊

// 使用 pub use 导出合适的公有 API
// 你可以选择使用 pub use 重导出（re-export）项来使公有结构不同于私有结构。重导出获取位于一个位置的公有项并将其公开到另一个位置，好像它就定义在这个新位置一样。

// 重新导出到目录
// 会显示在 Re-exports 这栏
pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
    /// The primary colors according to the RYB color model.
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    /// Combines two primary colors in equal amounts to create
    /// a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        // --snip--
        SecondaryColor::Green
    }
}
