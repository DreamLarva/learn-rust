#![allow(unused_variables)]


#[derive(Debug)]
pub struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}

// Rust 中的测试就是一个带有 test 属性注解的函数。
// 为了将一个函数变成测试函数，需要在 fn 行之前加上 #[test]。
// 当使用 cargo test 命令运行测试时，Rust 会构建一个测试执行程序用来调用标记了 test 属性的函数，并报告每一个测试是通过还是失败。
#[cfg(test)]
mod tests1 {
    // fn 行之前的 #[test]：这个属性表明这是一个测试函数
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

//            #[test]
//            fn another() {
//                panic!("Make this test fail");
//            }
}

// 使用assert!宏来检查结果
// assert! 宏由标准库提供，在希望确保测试中一些条件为 true 时非常有用。需要向 assert! 宏提供一个求值为布尔值的参数。
// 如果值是 true，assert! 什么也不做，同时测试会通过。
// 如果值为 false，assert! 调用 panic! 宏，这会导致测试失败。
// assert! 宏帮助我们检查代码是否以期望的方式运行。
#[cfg(test)]
mod tests2 {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { length: 8, width: 7 };
        let smaller = Rectangle { length: 5, width: 1 };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle { length: 8, width: 7 };
        let smaller = Rectangle { length: 5, width: 1 };

        assert!(!smaller.can_hold(&larger));
    }
}

// 使用 assert_eq! 和 assert_ne! 宏来测试相等
// assert_eq! 和 assert_ne!。这两个宏分别比较两个值是相等还是不相等。

pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests3 {
    use super::*;

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }
}

// 自定义失败信息
// 你也可以向 assert!、assert_eq! 和 assert_ne!
// 宏传递一个可选的失败信息参数，可以在测试失败时将自定义失败信息一同打印出来
pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

#[cfg(test)]
mod test4 {
    use super::greeting;

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"), "Greeting did not contain name, value was `{}`", result);
    }
}

// 使用 should_panic 检查 panic
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {}.",
                   value);
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {}.",
                   value);
        }

        Guess {
            value
        }
    }
}

#[cfg(test)]
mod tests5 {
    use super::*;

    #[test]
    // 指定报的是什么错误
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
}

// 将Rest<T,E>用于测试
// 使用Result 枚举 这样就不用 声明#[should_panic]了 处理了 同时处理了成功和失败的两种情况
#[cfg(test)]
mod tests6 {
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}


