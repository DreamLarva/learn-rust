#![allow(unused_variables)] // 不对 未使用的变量 warning

use std::collections::HashMap;
use std::error::Error;
use std::fs::{self, File};
use std::future::Future;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

//  panic!宏。当执行这个宏时，程序会打印出一个错误信息，展开并清理栈数据，然后接着退出。
pub fn ch09_01_unrecoverable_errors_with_panic() {
    // !panic 与不可恢复错误
    {
        // panic!("crash and burn"); // 抛出错误
    }
    //  当出现 panic 时，程序默认会开始 展开（unwinding），
    // 这意味着 Rust 会回溯栈并清理它遇到的每一个函数的数据，不过这个回溯并清理的过程有很多工作。
    // 另一种选择是直接 终止（abort），这会不清理数据就退出程序。
    // 那么程序所使用的内存需要由操作系统来清理。
    // 如果你需要项目的最终二进制文件越小越好，
    // panic 时通过在 Cargo.toml 的 [profile] 部分增加 panic = 'abort'，可以由展开切换为终止。
    // 例如，如果你想要在release模式中 panic 时直接终止：
    // [profile.release]
    // panic = 'abort'

    let v = vec![1, 2, 3];
    // v[99]; // panic
}

pub fn ch09_02_recoverable_errors_with_result() {
    // enum Result<T, E> {
    //     Ok(T), // 成功的类型
    //     Err(E), // 错误的类型
    // }

    /*{
        let f = File::open("hello.txt");
        let f = match f {
            Ok(file) => file,
            Err(error) => { // 匹配错误
                panic!("there is was a problem opening the file: {:?}",error) // 不能注释 否则不能编译

            }
        };
    }*/
    // 匹配不同的错误
    {
        let f = File::open("hello.txt");
        let f = match f {
            Ok(file) => file,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("Tired to create file but there is a problem: {:?}", e),
                },
                other_error => panic!("there was a problem opening the file: {:?}", other_error),
            },
        };
    }
    // 个更老练的 Rustacean 可能会这么写
    {
        let f = File::open("hello.txt").unwrap_or_else(|error| {
            if error.kind() == ErrorKind::NotFound {
                File::create("hello.txt").unwrap_or_else(|error| {
                    panic!("Tried to create file but there was a problem: {:?}", error);
                })
            } else {
                panic!("There was a problem opening the file: {:?}", error);
            }
        });
    }

    // 失败时 panic 的简写 : unwrap 和 expect
    {
        // 如果 Result 值是成员 Ok，unwrap 会返回 Ok 中的值。
        // 如果 Result 是成员 Err，unwrap 会为我们调用 panic!。
        let f = File::open("hello1.txt").unwrap();

        // 使用 expect 而不是 unwrap 并提供一个好的错误信息可以表明你的意图并更易于追踪 panic 的根源。
        let f = File::open("hello1.txt").expect("Failed to open hello.txt");
    }

    // 传播错误
    // 当编写一个其实现会调用一些可能会失败的操作的函数时，除了在这个函数中处理错误外，还可以选择让调用者知道这个错误并决定该如何处理。
    // 这被称为 传播（propagating）错误，这样能更好的控制代码调用，因为比起你代码所拥有的上下文，调用者可能拥有更多信息或逻辑来决定应该如何处理错误。
    {
        fn read_username_from_file() -> Result<String, io::Error> {
            let f = File::open("hello.txt");

            let mut f = match f {
                // 如果没有文件就直接报错
                Ok(file) => file,
                Err(e) => return Err(e),
            };

            let mut s = String::new();
            // 有文件返回的内容
            match f.read_to_string(&mut s) {
                // 将读取到的内容 放到 字符串s中
                Ok(_) => Ok(s),
                Err(e) => Err(e),
            }
        }

        match read_username_from_file() {
            Ok(s) => println!("data is: {}", s),
            Err(e) => panic!("{:?}", e),
        }
    }

    // 传播错误的简写 : ?
    {
        fn read_username_from_file_origin() -> Result<String, io::Error> {
            let f = File::open("hello.txt");
            let mut f = match f {
                Ok(file) => file,
                Err(e) => return Err(e),
            };
            let mut s = String::new();
            match f.read_to_string(&mut s) {
                Ok(_) => Ok(s),
                Err(e) => Err(e),
            }
        }
        // 在Result 的第二个泛型
        // 如果值是ok 就返回ok 中的值 继续执行
        // 如果是Err 就将Err 中的值作为整个函数的返回值 就像使用了return 的关键字一样
        fn read_username_from_file() -> Result<String, io::Error> {
            let mut f = File::open("hello.txt")?;
            let mut s = String::new();
            f.read_to_string(&mut s)?;
            Ok(s)
        }
        match read_username_from_file() {
            Ok(s) => println!("data is: {}", s),
            Err(e) => panic!("{:?}", e),
        }
    }
    // 进一步使用链式调用缩短代码 链式调用中也能够是用 ?
    {
        fn read_username_from_file() -> Result<String, io::Error> {
            let mut s = String::new();
            File::open("hello.txt")?.read_to_string(&mut s)?;
            Ok(s)
        }
    }
    {
        //  Rust 提供了名为 fs::read_to_string 的函数，它会打开文件、新建一个 String、读取文件的内容，并将内容放入 String，接着返回它。
        // 当然，这样做就没有展示所有这些错误处理的机会了
        fn read_username_from_file() -> Result<String, io::Error> {
            fs::read_to_string("hello.txt")
        }
    }

    // ? 只能被用于返回 Result 的函数
    {
        fn main() {
            // 错误指出只能在返回 Result 的函数中使用 ?。
            // 在不返回 Result 的函数中，当调用其他返回 Result 的函数时，需要使用 match 或 Result 的方法之一来处理，而不能用 ? 将潜在的错误传播给代码调用方。
            // let f = File::open("hello.txt")?; // 编译出错
            // 编译器会直接检查出 外侧的函数的 返回类型 不是 Result 而报错
        }
    }
    {
        // Box<dyn Error> 理解为 任何的错误
        fn main() -> Result<(), Box<dyn Error>> {
            let f = File::open("hello.txt")?; // 编译出错

            Ok(())
        }
    }
}

pub fn ch09_03_to_panic_or_not_to_panic() {
    // 示例、代码原型和测试都非常适合 panic
    // 当你编写一个示例来展示一些概念时，在拥有健壮的错误处理代码的同时也会使得例子不那么明确。
    // 例如，调用一个类似 unwrap 这样可能 panic! 的方法可以被理解为一个你实际希望程序处理错误方式的占位符，
    // 它根据其余代码运行方式可能会各不相同。
    //
    // 类似地，在我们准备好决定如何处理错误之前，unwrap和expect方法在原型设计时非常方便。
    // 当我们准备好让程序更加健壮时，它们会在代码中留下清晰的标记。
    //
    // 如果方法调用在测试中失败了，我们希望这个测试都失败，即便这个方法并不是需要测试的功能。
    // 因为 panic! 是测试如何被标记为失败的，调用 unwrap 或 expect 就是应该发生的事情。

    // 当我们比编译器知道更多的情况
    // 当你有一些其他的逻辑来确保 Result 会是 Ok 值时，调用 unwrap 也是合适的，
    // 虽然编译器无法理解这种逻辑。你仍然需要处理一个 Result 值：即使在你的特定
    // 情况下逻辑上是不可能的，你所调用的任何操作仍然有可能失败。如果通过人工检
    // 查代码来确保永远也不会出现 Err 值，那么调用 unwrap 也是完全可以接受的，
    // 这里是一个例子：
    {
        use std::net::IpAddr;
        // 硬编码的字符
        let home: IpAddr = "127.0.0.1".parse().unwrap(); // 我们知道绝对不会报错 但是编译器不知道仍然返回是Result 所以用 unwrap 而不是 ?
    }

    // 错误处理知道原则
    // 在当有可能会导致有害状态的情况下建议使用 panic! —— 在这里，有害状态是指当一些假设、
    // 保证、协议或不可变性被打破的状态，例如无效的值、自相矛盾的值或者被传递了不存在的值 ——
    // 外加如下几种情况：
    //  有害状态并不包含 预期 会偶尔发生的错误
    //  之后的代码的运行依赖于处于这种有害状态
    //  当没有可行的手段来将有害状态信息编码进所使用的类型中的情况

    // 如果别人调用你的代码并传递了一个没有意义的值，最好的情况也许就是 panic! 并警告使用你的
    // 库的人他的代码中有 bug 以便他能在开发时就修复它。类似的，panic! 通常适合调用不能够控制
    // 的外部代码时，这时无法修复其返回的无效状态。

    // 然而当错误预期会出现时，返回 Result 仍要比调用 panic! 更为合适。这样的例子包括解析器接
    // 收到错误数据，或者 HTTP 请求返回一个表明触发了限流的状态。在这些例子中，应该通过返回
    // Result 来表明失败预期是可能的，这样将有害状态向上传播，调用者就可以决定该如何处理这个问
    // 题。使用 panic! 来处理这些情况就不是最好的选择。

    // 当代码对值进行操作时，应该首先验证值是有效的，并在其无效时 panic!。这主要是出于安全的原
    // 因：尝试操作无效数据会暴露代码漏洞，这就是标准库在尝试越界访问数组时会 panic! 的主要原
    // 因：尝试访问不属于当前数据结构的内存是一个常见的安全隐患。函数通常都遵循 契约（contracts）
    // ：他们的行为只有在输入满足特定条件时才能得到保证。当违反契约时 panic 是有道理的，因为这通
    // 常代表调用方的 bug，而且这也不是那种你希望调用方必须处理的错误。事实上也没有合理的方式来
    // 恢复调用方的代码：调用方的 程序员 需要修复其代码。函数的契约，尤其是当违反它会造成 panic
    // 的契约，应该在函数的 API 文档中得到解释。

    // 虽然在所有函数中都拥有许多错误检查是冗长而烦人的。幸运的是，可以利用 Rust 的类型系统（以
    // 及编译器的类型检查）为你进行很多检查。如果函数有一个特定类型的参数，可以在知晓编译器已经
    // 确保其拥有一个有效值的前提下进行你的代码逻辑。例如，如果你使用了一个不同于 Option 的类型
    // ，而且程序期望它是 有值 的并且不是 空值。你的代码无需处理 Some 和 None 这两种情况，它只
    // 会有一种情况就是绝对会有一个值。尝试向函数传递空值的代码甚至根本不能编译，所以你的函数在
    // 运行时没有必要判空。另外一个例子是使用像 u32 这样的无符号整型，也会确保它永远不为负。
}
