use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;
use std::thread;
use std::time::Duration;

// Rust 的 闭包（closures）是可以保存进变量或作为参数传递给其他函数的匿名函数。可以在一个地方创建闭包，然
// 后在不同的上下文中执行闭包运算。
// 不同于函数，闭包允许捕获调用者作用域中的值。
// 我们将展示闭包的这些功能如何复用代码和自定义行为。
pub fn ch13_01_closures() {
    // 使用闭包常见行为的抽象
    {
        // 这里将通过调用 simulated_expensive_calculation 函数来模拟调用假象的算法，如示例 13-1 所示，
        // 它会打印出 calculating slowly...，等待两秒，并接着返回传递给它的数字
        fn simulated_expensive_calculation(intensity: u32) -> u32 {
            println!("calculating slowly...");
            thread::sleep(Duration::from_secs(2));
            intensity
        }

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

            // let expensive_closure = |num| simulated_expensive_calculation(num);

            if intensity < 25 {
                println!("Today, do {} pushups!", expensive_closure(intensity) as u32);
                println!("Next, do {} situps!", expensive_closure(intensity) as u32);
            } else {
                if random_number == 3 {
                    println!("Take a break today! Remember to stay hydrated!");
                } else {
                    println!(
                        "Today, run for {} minutes!",
                        expensive_closure(intensity) as u32
                    );
                }
            }
        }

        let simulated_user_specified_value = 10;
        let simulated_random_number = 7;

        generate_workout(simulated_user_specified_value, simulated_random_number);
    }

    // 闭包类型推断和注解
    {
        // 闭包不要求像 fn 函数那样在参数和返回值上注明类型。
        // 函数中需要类型注解是因为他们是暴露给用户的显式接口的一部分。
        fn add_one_v1(x: u32) -> u32 {
            x + 1
        }
        let add_one_v2 = |x: u32| -> u32 { x + 1 };
        let add_one_v3 = |x| x + 1;
        let add_one_v4 = |x| x + 1;

        // ** 注意没有声明类型的闭包 必须调用 否则会报错没法识别类型 **
        // 因为 rust 编译器 会根据之后**第一次**调用时入参的类型 来确定之前申明时候应该是是什么类型
        add_one_v3(1);
        add_one_v4(1);
        // 这些的方法除了类型是相同 使用起来是 相同的
        let example_closure = |x| x;

        let s = example_closure(String::from("hello"));
        // let n = example_closure(5); // error 如果尝试对同一闭包使用不同类型则会得到类型错误
    }

    // 使用带有泛型和Fn trait 的闭包
    {
        // Fn 系列 trait 由标准库提供。所有的闭包都实现了 trait Fn、FnMut 或 FnOnce 中的一个。
        // 注意：函数也都实现了这三个 Fn trait。如果不需要捕获环境中的值，则可以使用实现了 Fn trait 的函数而不是闭包。

        struct Cacher<T>
        where
            T: Fn(u32) -> u32, // 声明了 T 泛型
        {
            calculation: T,
            value: Option<u32>,
        }

        impl<T> Cacher<T>
        where
            T: Fn(u32) -> u32, // 还得再写一遍
        {
            fn new(calculation: T) -> Cacher<T> {
                Cacher {
                    calculation,
                    value: None,
                }
            }
            fn value(&mut self, arg: u32) -> u32 {
                match self.value {
                    // 检测value 值 因为默认是 None 一定会进入None 分支
                    Some(v) => v,
                    None => {
                        // 注意 这里使用 () 包裹了self.calculation  才能调用, 应该是闭包函数需要()
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
                println!("Today, do {} pushups!", expensive_result.value(intensity));
                println!("Next, do {} situps!", expensive_result.value(intensity));
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
        where
            T: Fn(u32) -> u32,
        {
            calculation: T,
            value: HashMap<u32, u32>,
        }

        impl<T> Cacher<T>
        where
            T: Fn(u32) -> u32,
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
        let a = 2;
        println!("{}", cacher.value(a));
    }
    {
        // 当前 Cacher 实现的第二个问题是它的应用被限制为只接受获取一个 u32 值并返回一个 u32 值的闭包。
        // 比如说，我们可能需要能够缓存一个获取字符串 slice 并返回 usize 值的闭包的结果。请尝试引入更多
        // 泛型参数来增加 Cacher 功能的灵活性。
        struct Cacher1<T, U>
        where
            U: Eq + Hash,
            T: Fn(U) -> u32,
        {
            calculation: T,
            value: HashMap<U, u32>,
        }

        impl<T, U> Cacher1<T, U>
        where
            U: Eq + Hash + Clone,
            T: Fn(U) -> u32, // 获取所有权
        {
            fn new(calculation: T) -> Cacher1<T, U> {
                Cacher1 {
                    calculation,
                    value: HashMap::new(),
                }
            }
            fn value(&mut self, arg: U) -> u32 {
                match self.value.get(&arg) {
                    Some(v) => *v, // v type为 &u32 需要 用 * 解引用
                    None => {
                        let value = (self.calculation)(arg.clone());
                        self.value.insert(arg, value);
                        value
                    }
                }
            }
        }
        let mut cacher1 = Cacher1::new(|a| (a + 1) as u32);
        println!("Cacher1 {}", cacher1.value(1));
        let mut cacher1 = Cacher1::new(|a: String| a.len() as u32);
        println!("Cacher1 {}", cacher1.value(String::from("123")));
    }
    {
        // 当前 Cacher 实现的第二个问题是它的应用被限制为只接受获取一个 u32 值并返回一个 u32 值的闭包。
        // 比如说，我们可能需要能够缓存一个获取字符串 slice 并返回 usize 值的闭包的结果。请尝试引入更多
        // 泛型参数来增加 Cacher 功能的灵活性。
        struct Cacher2<T, U>
        where
            U: Eq + Hash,
            T: Fn(&U) -> u32, // 引用不获取所有权
        {
            calculation: T,
            value: HashMap<U, u32>,
        }

        impl<T, U> Cacher2<T, U>
        where
            U: Eq + Hash + Clone,
            T: Fn(&U) -> u32,
        {
            fn new(calculation: T) -> Cacher2<T, U> {
                Cacher2 {
                    calculation,
                    value: HashMap::new(),
                }
            }
            fn value(&mut self, arg: U) -> u32 {
                match self.value.get(&arg) {
                    Some(v) => *v, // v type为 &u32 需要 用 * 解引用
                    None => {
                        let value = (self.calculation)(&arg);
                        self.value.insert(arg, value);
                        value
                    }
                }
            }
        }
        let mut cacher1 = Cacher2::new(|a| (a + 1) as u32);
        println!("Cacher1 {}", cacher1.value(1));
        let mut cacher1 = Cacher2::new(|a: &String| a.len() as u32);
        println!("Cacher1 {}", cacher1.value(String::from("123")));
    }
    {
        // 闭包会捕获其环境
        // 闭包可以通过三种方式捕获其环境，他们直接对应函数的三种获取参数的方式：获取所有权，可变借用和不可变借用。
        // 这三种捕获值的方式被编码为如下三个 Fn trait：
        // FnOnce 消费从周围作用域捕获的变量，闭包周围的作用域被称为其 环境，environment。为了消费捕获到的变量，
        // 闭包必须获取其所有权并在定义闭包时将其移动进闭包。
        // 其名称的 Once 部分代表了闭包不能多次获取相同变量的所有权的事实，所以它只能被调用一次。

        //  FnMut 获取可变的借用值所以可以改变其环境
        //  Fn 从其环境获取不可变的借用值
        {
            let x = 4;

            let equal_to_x1 = |z| z == x; // 这里 x 是拷贝

            let y = 4;

            assert!(equal_to_x1(y));
        }
        // 当创建一个闭包时，Rust 根据其如何使用环境中变量来推断我们希望如何引用环境。
        // 由于所有闭包都可以被调用至少一次，所以所有闭包都实现了 FnOnce 。
        // 那些并没有移动被捕获变量的所有权到闭包内的闭包也实现了 FnMut ，
        // 而不需要对被捕获的变量进行可变访问的闭包则也实现了 Fn 。
        // equal_to_x1 闭包不可变的借用了 x（所以 equal_to_x1 具有 Fn trait），
        // 因为闭包体只需要读取 x 的值。
        {
            let mut vec = vec![1, 2, 3, 4];
            let mut times = 0;
            vec.retain(|&x| {
                times += 1;
                x % 2 == 0
            });
            println!("times {times}");
            assert_eq!(vec, [2, 4]);
        }

        // 如果你希望强制闭包获取其使用的环境值的所有权，可以在参数列表前使用 move 关键字。
        // 这个技巧在将闭包传递给新线程以便将数据移动到新线程中时最为实用。
        {
            fn main() {
                let x = vec![1, 2, 3];

                //                          move 关键字 获取 x 的所有权
                let equal_to_x = move |z| z == x;

                // println!("can't use x here: {:?}", x); // error x 所有权已经被移入 equal_to_x 中

                let y = vec![1, 2, 3];

                assert!(equal_to_x(y));
            }
        }
    }
}

// 使用迭代器出路元素序列
pub fn ch13_02_iterators() {
    // 在 Rust 中，迭代器是惰性的（lazy），这意味着在调用方法使用迭代器之前它都不会有效果。
    {
        let v1 = vec![1, 2, 3];
        let v1_iter = v1.iter();
        for val in v1_iter {
            println!("Got: {}", val);
        }
    }

    // Iterator trait 和 next 方法
    // 迭代器起都实现了一个叫做 Iterator 的定义于标准库的trait
    {
        trait Iterator {
            // type Item 和 Self::Item，他们定义了 trait 的 关联类型（associated type）
            type Item;
            // next 是 Iterator 实现者被要求定义的唯一方法。
            // next 一次返回迭代器中的一个项，封装在 Some 中，当迭代器结束时，它返回 None。
            fn next(&mut self) -> Option<Self::Item>;

            // 此处省略了方法的默认实现
        }
        // 注意 v1_iter 需要是可变的：在迭代器上调用 next 方法改变了迭代器中用来记录序列位置的状态。换句
        // 话说，代码 消费（consume）了，或使用了迭代器。每一个 next 调用都会从迭代器中消费一个项。使用
        // for 循环时无需使 v1_iter 可变因为 for 循环会获取 v1_iter 的所有权并在后台使 v1_iter 可变。

        // 另外需要注意到从 next 调用中得到的值是 vector 的不可变引用。iter 方法生成一个不可变引用的迭代
        // 器。如果我们需要一个获取 v1 所有权并返回拥有所有权的迭代器，则可以调用 into_iter 而不是 iter。
        // 类似的，如果我们希望迭代可变引用，则可以调用 iter_mut 而不是 iter。
    }

    // 消费迭代器的方法
    {
        // Iterator trait 有一系列不同的由标准库提供默认实现的方法
        let v1 = vec![1, 2, 3];

        let v1_iter = v1.iter();

        // 必须声明类型 否则sum 不知道结果是什么类型
        let total = v1_iter.sum::<i32>();
        // 调用 sum 之后不再允许使用 v1_iter 因为调用 sum 时它会获取迭代器的所有权。

        assert_eq!(total, 6);
    }

    // 产生其他迭代器的方法
    {
        // Iterator trait 中定义了另一类方法，被称为 迭代器适配器（iterator adaptors），他们允许我们将当
        // 前迭代器变为不同类型的迭代器。可以链式调用多个迭代器适配器。不过因为所有的迭代器都是惰性的，
        // 必须调用一个消费适配器方法以便获取迭代器适配器调用的结果。
        let v1 = vec![1, 2, 3];

        // collect 方法。这个方法消费迭代器并将结果收集到一个数据结构中。
        let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
        let v2 = v1.iter().map(|x| x + 1).collect::<Vec<_>>();

        assert_eq!(v2, vec![2, 3, 4]);
    }

    // 使用闭包获取环境
    {
        #[derive(PartialEq, Debug)]
        struct Shoe {
            size: u32,
            style: String,
        }
        fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
            // into_iter 来创建一个获取 vector 所有权的迭代器
            shoes.into_iter().filter(|s| s.size == shoe_size).collect()
            // 所以在这里 shoes 已经不能使用了
        }

        #[test]
        fn filters_by_size() {
            let shoes = vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker"),
                },
                Shoe {
                    size: 13,
                    style: String::from("sandal"),
                },
                Shoe {
                    size: 10,
                    style: String::from("boot"),
                },
            ];

            let in_my_size = shoes_in_my_size(shoes, 10);

            assert_eq!(
                in_my_size,
                vec![
                    Shoe {
                        size: 10,
                        style: String::from("sneaker"),
                    },
                    Shoe {
                        size: 10,
                        style: String::from("boot"),
                    },
                ]
            );
        }
    }
    // 实现 Iterator  trait 来创建自定义迭代器
    // 我们已经展示了可以通过在 vector 上调用 iter、into_iter 或 iter_mut 来创建一个迭代器。
    // 也可以用标准库中其他的集合类型创建迭代器，比如哈希 map。
    // 另外，可以实现 Iterator trait 来创建任何我们希望的迭代器。
    // 正如之前提到的，定义中唯一要求提供的方法就是 next 方法。
    // 一旦定义了它，就可以使用所有其他由 Iterator trait 提供的拥有默认实现的方法来创建自定义迭代器了！
    {
        struct Counter {
            count: u32,
        }
        impl Counter {
            fn new() -> Counter {
                Counter { count: 0 }
            }
        }
        impl Iterator for Counter {
            // 迭代器的关联类型 Item 设置为 u32，意味着迭代器会返回 u32 值集合
            type Item = u32;
            fn next(&mut self) -> Option<Self::Item> {
                self.count += 1;

                if self.count < 6 {
                    Some(self.count)
                } else {
                    None
                }
            }
        }

        let mut counter = Counter::new();

        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), Some(4));
        assert_eq!(counter.next(), Some(5));
        assert_eq!(counter.next(), None);

        // 使用自定义的 Iterator trait 方法
        // 通过定义 next 方法实现 Iterator trait，我们现在就可以使用任何标准库定义的拥有默认实现的
        // Iterator trait 方法了，因为他们都使用了 next 方法的功能。
        // 这里是 zip 方法 传入一个 trait IntoIterator的实现 返回一个 Iterator 枚举(Iterator1,Iterator2)
        let sum: u32 = Counter::new()
            .zip(Counter::new().skip(1)) // 注意 zip 只产生四对值
            // 理论上第五对值 (5, None) 从未被产生，因为 zip 在任一输入迭代器返回 None 时也返回 None
            .map(|(a, b)| a * b)
            .filter(|x| x % 3 == 0)
            .sum();
        assert_eq!(18, sum);
    }
}

#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

pub fn ch13_04_performance(buffer: &mut [i32], coefficients: [i64; 12], qlp_shift: i16) {
    // 为了计算 prediction 的值，这些代码遍历了 coefficients 中的 12 个值，
    // 使用 zip 方法将系数与 buffer 的前 12 个值组合在一起。
    // 接着将每一对值相乘，再将所有结果相加，然后将总和右移 qlp_shift 位。
    for i in 12..buffer.len() {
        let prediction = coefficients
            .iter()
            .zip(&buffer[i - 12..i]) // 等同于 (i - 12)..i
            .map(|(&c, &s)| c * s as i64)
            .sum::<i64>()
            >> qlp_shift;
        let delta = buffer[i];
        buffer[i] = prediction as i32 + delta;
    }
}
