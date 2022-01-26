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

struct Config<'a> {
    query: &'a String,
    filename: &'a String,
}

impl Config<'_> {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = &args[1];
        let filename = &args[2];

        // Config 类型 要的类型 都是用完整的 所有权 所以必须上面 那两个解的 只能 clone 了
        Ok(Config { query, filename })
    }
}

pub fn minigrep() {
    let args: Vec<_> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("problem parsing arguments: {}", err);
        process::exit(1)
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = run(config) {
        println!("Application error : {}", e);

        process::exit(1);
    }
}

// Box<dyn Error> 意味着函数会返回 任意实现了 Error trait 的类型，
// 不过无需指定具体将会返回的值的类型。这提供了在不同的错误场景可能有不同类型的错误返回值的灵活性。
// 这也就是 dyn，它是 “动态的”（“dynamic”）的缩写。
// 默认的情况就是 dyn, 你不写 编译器会提示你 要加上
// dyn 一般都用在返回类型上, 而 impl 一般用在 入参或者其他位置上
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // 直接给文件名 默认是从 根目录 开始找
    let contents = fs::read_to_string(config.filename)?;

    for line in search(&config.query, &contents) {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive
Pick three.";
        assert_eq!(vec!["safe, fast, productive"], search(query, contents))
    }
}
