#![allow(unused_variables)]

// 控制测试如何执行
// 使用 cargo test --help 查看cargo test 可用的参数

// 并行或连续的运行测试
// 当运行多个测试时， Rust 默认使用线程来并行运行。
// 这意味着测试会更快地运行完毕，所以你可以更快的得到代码能否工作的反馈。
// 因为测试是在同时运行的，你应该确保测试不能相互依赖，或依赖任何共享的状态，包括依赖共享的环境，比如当前工作目录或者环境变量。

// 如果你不希望测试并行运行，或者想要更加精确的控制线程的数量，可以传递 --test-threads 参数和希望使用线程的数量给测试二进制文件。
fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}

// 注意输出中不会出现测试通过时打印的内容
// 如果希望显示 可以使用  cargo test -- --nocapture
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    #[test]
    #[should_panic]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(5, value);
    }
}

// 通过制定名字来运行部分测试
pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests1 {
    use super::*;

    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2))
    }

    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }

    #[test]
    #[ignore] // 除了 运行 cargo test -- --ignored 之外的指令这个测试都会被忽略
    fn expensive_test() {
        // code that takes an hour to run
    }
}

// 运行单个测试
// 直接运行 cargo test one_hundred 就会只运行这个one_hundred方法 好nb啊

// 过滤运行多个测试
// cargo test add 运行所有方法名中包含 超nb

// 忽略某些测试
// 使用额外(需要已经有$[test])#[ignore] 注释
// 只运行ignore 的测试 cargo test -- --ignored
