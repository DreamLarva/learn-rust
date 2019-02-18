use std::thread;
use std::time::Duration;
use std::collections::HashMap;

// Rust 的 闭包（closures）是可以保存进变量或作为参数传递给其他函数的匿名函数。可以在一个地方创建闭包，然
// 后在不同的上下文中执行闭包运算。
// 不同于函数，闭包允许捕获调用者作用域中的值。
// 我们将展示闭包的这些功能如何复用代码和自定义行为。
pub fn ch13_01_closures() {
    // 使用闭包常见行为的抽象
    {
        // 这里将通过调用 simulated_expensive_calculation 函数来模拟调用假象的算法，如示例 13-1 所示，
        // 它会打印出 calculating slowly...，等待两秒，并接着返回传递给它的数字

        fn generate_workout(intensity: u32, random_number: u32) {

            // 使用闭包存储代码
            // 闭包的定义以一对竖线 | 开始,在竖线中指定闭包的参数；
            // 这个闭包有一个参数 num；如果有多于一个参数，可以使用逗号分隔，比如 |param1, param2|
            // 参数之后是存放闭包体的大括号 —— 如果闭包体只有一行则大括号是可以省略的。
            let expensive_closure = |num| {
                println!("calculating slowly...");
                thread::sleep(Duration::from_secs(2));
                num
            };

            if intensity < 25 {
                println!(
                    "Today, do {} pushups!",
                    expensive_closure(intensity)
                );
                println!(
                    "Next, do {} situps!",
                    expensive_closure(intensity)
                );
            } else {
                if random_number == 3 {
                    println!("Take a break today! Remember to stay hydrated!");
                } else {
                    println!(
                        "Today, run for {} minutes!",
                        expensive_closure(intensity)
                    );
                }
            }
        }

        let simulated_user_specified_value = 10;
        let simulated_random_number = 7;

        generate_workout(
            simulated_user_specified_value,
            simulated_random_number,
        );
    }

    // 闭包类型推断和注解
    {
        // 闭包不要求像 fn 函数那样在参数和返回值上注明类型。函数中需要类型注解是因为他们是暴露给用户的显
        // 式接口的一部分。
        fn add_one_v1(x: u32) -> u32 { x + 1 }
        let add_one_v2 = |x: u32| -> u32 { x + 1 };
        let add_one_v3 = |x| { x + 1 };
        let add_one_v4 = |x| x + 1;

        // 注意没有声明类型的闭包 必须调用 否则会报错没法识别类型
        add_one_v3(1);
        add_one_v4(1);
        // 这些的方法除了类型是相同 使用起来是 相同的
        let example_closure = |x| x;
        //
        let s = example_closure(String::from("hello"));
//        let n = example_closure(5); // error 如果尝试对同一闭包使用不同类型则会得到类型错误
    }

    // 使用带有泛型和Fn trait 的闭包
    {
        struct Cacher<T>
        // Fn 系列 trait 由标准库提供。所有的闭包都实现了 trait Fn、FnMut 或 FnOnce 中的一个。
        // 注意：函数也都实现了这三个 Fn trait。如果不需要捕获环境中的值，则可以使用实现了 Fn trait 的函数而不是闭包。
            where T: Fn(u32) -> u32 // 声明了 T 发型
        {
            calculation: T,
            value: Option<u32>,
        }

        impl<T> Cacher<T>
            where T: Fn(u32) -> u32
        {
            fn new(calculation: T) -> Cacher<T> {
                Cacher {
                    calculation,
                    value: None,
                }
            }
            fn value(&mut self, arg: u32) -> u32 {
                match self.value { // 检测value 值 因为默认是 None 一定会进入None 分支
                    Some(v) => v,
                    None => {
                        // 注意 这里使用 () 包裹了self.calculation  才能调用
                        let v = (self.calculation)(arg); // 执行方法
                        self.value = Some(v); // 存入结构体
                        v // 返回本次的结果
                    }
                }
            }
        }





        fn generate_workout(intensity: u32, random_number: u32) {
            let mut expensive_result = Cacher::new(|num| {
                println!("calculating slowly...");
                thread::sleep(Duration::from_secs(2));
                num
            });

            if intensity < 25 {
                println!(
                    "Today, do {} pushups!",
                    expensive_result.value(intensity)
                );
                println!(
                    "Next, do {} situps!",
                    expensive_result.value(intensity)
                );
            } else {
                if random_number == 3 {
                    println!("Take a break today! Remember to stay hydrated!");
                } else {
                    println!(
                        "Today, run for {} minutes!",
                        expensive_result.value(intensity)
                    );
                }
            }
        }

        // 不同于直接将闭包保存进一个变量，我们保存一个新的 Cacher 实例来存放闭包。接着，
        // 在每一个需要结果的地方，调用 Cacher 实例的 value 方法。可以调用 value 方法任意多次，
        // 或者一次也不调用，而慢计算最多只会运行一次。
    }
    {
        // 存储值使用HashMap
        struct Cacher<T>
            where T: Fn(u32) -> PartialOrd
        {
            calculation: T,
            value: HashMap<u32, u32>,
        }

        impl<T> Cacher<T>
            where T: Fn(u32) -> u32
        {
            fn new(calculation: T) -> Cacher<T> {
                Cacher {
                    calculation,
                    value: HashMap::new(),
                }
            }
            fn value(&mut self, arg: u32) -> u32 {

                match self.value.get(&arg) {
                    Some(v) => *v, // v type为 &u32 需要 用 * 解引用
                    None => {
                        let value = (self.calculation)(arg);
                        self.value.insert(arg, value);
                        value
                    }

                }
            }
        }

        let mut cacher = Cacher::new(|a| a + 1);
        println!("{}", cacher.value(1));
        println!("{}", cacher.value(1));
        println!("{}", cacher.value(2));
        println!("{}", cacher.value(2));

        // 当前 Cacher 实现的第二个问题是它的应用被限制为只接受获取一个 u32 值并返回一个 u32 值的闭包。
        // 比如说，我们可能需要能够缓存一个获取字符串 slice 并返回 usize 值的闭包的结果。请尝试引入更多
        // 泛型参数来增加 Cacher 功能的灵活性。
        // todo
    }
}