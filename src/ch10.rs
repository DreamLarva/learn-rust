#![allow(unused_variables)] // 不对 未使用的变量 warning

pub fn ch10_00_generics() {
    // 这里入参的类型 是 &切片 使用切片的时候必定 有&
    fn largest(list: &[i32]) -> i32 {
        let mut largest = list[0]; // Copy 操作

        for &item in list {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    // 注意这里是 矢量
    let number_list = vec![34, 50, 25, 100, 65];
    let test = &vec![34, 50, 25, 100, 65][..]; // &[i32]

    let result = largest(&number_list);
    println!("The largest number is {}", result);
    assert_eq!(result, 100);

    // 切片
    let number_list = [102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest(&number_list);
    println!("The largest number is {}", result);
    assert_eq!(result, 6000);
}

pub fn ch10_01_syntax() {
    // 结构体中定义的泛型
    {
        struct Point<T> {
            x: T,
            y: T,
        }

        let integer = Point { x: 5, y: 10 };
        let float = Point { x: 1.0, y: 4.0 };
    }
    {
        struct Point<T, U> {
            x: T,
            y: U,
        }
        let both_integer = Point { x: 5, y: 10 };
        let both_float = Point { x: 1.0, y: 4.0 };
        let integer_and_float = Point { x: 5, y: 4.0 };
    }
    // 枚举定义中的泛型
    {
        enum Option<T> {
            Some(T),
            None,
        }
        enum Result<T, E> {
            Ok(T),
            Err(E),
        }
    }
    // 方法中使用泛型
    {
        struct Point<T> {
            x: T,
            y: T,
        }
        // 注意必须在 impl 后面声明 T，这样就可以在 Point<A> 上实现的方法中使用它了
        impl<A> Point<A> {
            fn x(&self) -> &A {
                &self.x
            }
        }
        // 给予是 f32 类型的实例才有的方法(有点像重载)
        impl Point<f32> {
            fn distance_from_origin(&self) -> f32 {
                (self.x.powi(2) + self.y.powi(2)).sqrt()
            }
        }

        let mut p = Point { x: 5, y: 10 };

        let q = Point { x: 5.0, y: 10.0 };

        println!("p.x = {}", p.x());
        // println!("p.x = {}", p.distance_from_origin()); // 报错
        println!("p.x = {}", q.distance_from_origin());
    }

    // 结构体定义中的泛型类型参数并不总是与结构体方法签名中使用的泛型是同一类型
    {
        struct Point<T, U> {
            x: T,
            y: U,
        }
        // 这里的泛型的个数 是死的 impl 右边必须是2个 Point右边也必须是2个
        impl<T, U> Point<T, U> {
            // 来自结构体的泛型 V W 完全是 mix_up 方法作用域中 用到的泛型
            // 直接获取的 传入的 Point 的所有权
            fn mix_up<V, W>(self, other: Point<V, W>) -> Point<T, W> {
                // 来自函数的泛型
                Point {
                    x: self.x,  // self Point 的 T
                    y: other.y, // Other Point 的 W
                }
            }
        }
        let p1 = Point { x: 5, y: 10.4 };
        let p2 = Point { x: "Hello", y: 'c' };

        let p3 = p1.mix_up(p2);
        // p1.x; // error p1 已经移动
        // move occurs because `p1` has type `Point<i32, f64>`, which does not implement the `Copy` trait
        // p1调用的时候 就 moved了
        // p2.x; // error p2 已经移动

        println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

        let p4 = Point { x: 1, y: ["2"] };
        let p5 = Point { x: true, y: [1] };
        // 就算出现不能 Copy 的trait 的类型也没有关系
        // 因为 所有权全部移交了 当然没有关系 因为没有 任何借的东西
        let p6 = p4.mix_up(p5);
    }

    // const泛型（Rust 1.51版本引入的重要特性）
    {
        fn display_array1(arr: [i32; 3]) {
            println!("{arr:?}");
        }
        let arr: [i32; 3] = [1, 2, 3];
        display_array1(arr);

        let arr: [i32; 2] = [1, 2];
        // display_array(arr); // error [i32; 2] 和 [i32; 3] 是不同的数组类型

        fn display_array2<T: std::fmt::Debug>(arr: &[T]) {
            println!("{arr:?}");
        }
        display_array2(&[1, 2]);
        display_array2(&[1, 2, 3]);

        // N 这个泛型参数，它是一个基于值的泛型参数！因为它用来替代的是数组的长度。
        // N 就是const泛型，定义的语法是 const N: usize，表示const泛型N，它基于的值类型是 usize。
        fn display_array3<T: std::fmt::Debug, const N: usize>(arr: &[T; N]) {
            println!("{arr:?}")
        }
        display_array3(&[1, 2, 3]);

        // 数组泛型的长度 直接使用 值
        fn display_array4<T: std::fmt::Debug>(arr: &[T; 2]) {
            println!("{arr:?}")
        }
        display_array4(&[1, 2]);
        // display_array4(&[1, 2, 3]); // error
    }
}

pub fn ch10_02_traits() {
    // 定义 trait
    // 一个类型的行为由其可供调用的方法构成。如果可以对不同类型调用相同的方法的话，这些类型就可以共享相同的行为了。
    // trait 定义是一种将方法签名组合起来的方法，目的是定义一个实现某些目的所必需的行为的集合。

    // trait 体中可以有多个方法：一行一个方法签名且都以分号结尾。
    {
        pub trait Summary {
            fn summarize(&self) -> String;
        }
    }

    // 为类型实现trait
    // 当然必须 struct 和 trait 在同一作用域中
    // 如果已经存在的trait 则不能用同名的trait
    {
        pub trait Summary {
            fn summarize(&self) -> String;
        }
        pub struct NewsArticle {
            pub headline: String,
            pub location: String,
            pub author: String,
            pub content: String,
        }

        // 于 impl 关键字之后，我们提供需要实现 trait 的名称，接着是 for 和需要实现 trait 的类型的名称
        impl Summary for NewsArticle {
            fn summarize(&self) -> String {
                format!("{}, by {} ({})", self.headline, self.author, self.location)
            }
        }
        pub struct Tweet {
            pub username: String,
            pub content: String,
            pub reply: bool,
            pub retweet: bool,
        }
        impl Summary for Tweet {
            fn summarize(&self) -> String {
                format!("{}: {}", self.username, self.content)
            }
        }

        let tweet = Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        };

        // 一旦实现了 trait，我们就可以用与 NewsArticle 和 Tweet 实例的非 trait 方法一样的方式调用 trait 方法
        println!("1 new tweet: {}", tweet.summarize());
    }
    {
        struct Point<T, U> {
            x: T,
            y: U,
        }

        pub trait Summary1<T, S> {
            fn summarize1(&self) -> Point<T, S>;
        }

        // A,B 泛型 供整个 Summary2 trait 使用
        // T 泛型 仅供 summarize2 方法 使用
        pub trait Summary2<A, B> {
            fn summarize2<T>(&self, x: T) -> T;
        }

        pub struct Tweet<A, B> {
            a: A,
            b: B,
        }

        // 为 Tweet<bool, bool> 实现 Summary1<i32, f64>
        // 任何一个泛型不对 实例上是调用不了这个方法的
        // 如果使用ide 自动实现的话 i32, f64 的翻新会自动推导到 summarize1 方法的 返回类型上
        impl Summary1<i32, f64> for Tweet<bool, bool> {
            fn summarize1(&self) -> Point<i32, f64> {
                Point { x: 1, y: 2.0 }
            }
        }

        // 虽然实际没有 用到Summary2 的泛型 但是有几个 还是得写几个
        impl Summary2<i32, f64> for Tweet<i32, i32> {
            fn summarize2<T>(&self, x: T) -> T {
                x
            }
        }

        // 上面没有 summarize1 方法 有 summarize2 方法
        let tweet1 = Tweet { a: 2, b: 1 };
        tweet1.summarize2(1);

        // 只有 summarize1 方法`
        let tweet2 = Tweet { a: true, b: true };
        tweet2.summarize1();
    }

    // 实现 trait 时需要注意的一个限制是，只有当 trait 或者要实现 trait 的类型位于 crate 的本地作用域时，才能为该类型实现 trait。
    // 例如，可以为 aggregator crate 的自定义类型 Tweet 实现如标准库中的 Display trait，这是因为 Tweet 类型位于 aggregator crate 本地的作用域中。
    // 类似地，也可以在 aggregator crate 中为 Vec<T> 实现 Summary，这是因为 Summary trait 位于 aggregator crate 本地作用域中。

    // 通俗的说 类型(需要实现trait的目标) 和 trait 两者必须至少一个 在你的 crate 中
    // 但是不能为外部类型实现外部 trait。例如，不能在 aggregator crate 中为 Vec<T> 实现 Display trait。
    // 这是因为 Display 和 Vec<T> 都定义于标准库中，它们并不位于 aggregator crate 本地作用域中。
    // 这个限制是被称为 相干性（coherence） 的程序属性的一部分，
    // 或者更具体的说是 孤儿规则（orphan rule），其得名于不存在父类型。这条规则确保了其他人编写的代码不会破坏你代码，
    // 反之亦然。没有这条规则的话，两个 crate 可以分别对相同类型实现相同的 trait，而 Rust 将无从得知应该使用哪一个实现。

    // 默认实现
    {
        // 有时为 trait 中的某些或全部方法提供默认的行为，而不是在每个类型的每个实现中都定义自己的行为是很有用的。
        // 这样当为某个特定类型实现 trait 时，可以选择保留或重载每个方法的默认行为。
        pub trait Summary {
            // 有默认实现
            fn summarize(&self) -> String {
                String::from("(Read more...)")
            }
        }
        pub struct NewsArticle {
            pub headline: String,
            pub location: String,
            pub author: String,
            pub content: String,
        }
        impl Summary for NewsArticle {} // 空的实现
        let article = NewsArticle {
            headline: String::from("Penguins win the Stanley Cup Championship!"),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best
                    hockey team in the NHL.",
            ),
        };

        println!("New article available! {}", article.summarize());

        impl<T> Summary for Vec<T> {}
        // 只在这里有效 在main.rs 中依然没有 实现Summary
        println!("imply Summary for Vec, {}", vec![1].summarize());
    }
    // 重载一个默认的实现
    // 注意无法从相同方法的重载实现中 调用默认方法。(这不是当然吗?)
    {
        pub struct Tweet {
            pub username: String,
            pub content: String,
            pub reply: bool,
            pub retweet: bool,
        }
        pub trait Summary {
            fn summarize_author(&self) -> String;

            fn summarize(&self) -> String {
                self.summarize_author();
                format!("(Read more from {}...)", self.summarize_author())
            }
        }

        impl Summary for Tweet {
            fn summarize_author(&self) -> String {
                // self.summarize();
                format!("@{}", self.username)
            }
            // 默认实现了 summarize
        }

        let tweet = Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        };

        println!("1 new tweet: {}", tweet.summarize());
    }

    // trait 作为参数
    {
        // region
        pub trait Summary {
            fn summarize(&self) -> String {
                String::from("(Read more...)")
            }
        }
        pub struct NewsArticle {
            pub headline: String,
            pub location: String,
            pub author: String,
            pub content: String,
        }
        impl Summary for NewsArticle {} // 空的实现

        // endregion
        let article = NewsArticle {
            headline: String::from("Penguins win the Stanley Cup Championship!"),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best
                    hockey team in the NHL.",
            ),
        };

        pub fn notify(item: impl Summary) {
            // 传入一个已经实现了 Summary的实例
            println!("Breaking news! {}", item.summarize());
        }

        notify(article)
    }

    // Trait Bounds
    // impl Trait 语法适用于短小的例子,它不过是一个较长形式的语法糖 这被称为 trait bound
    {
        // region
        pub trait Summary {
            fn summarize(&self) -> String {
                String::from("(Read more...)")
            }
        }
        pub struct NewsArticle {
            pub headline: String,
            pub location: String,
            pub author: String,
            pub content: String,
        }
        impl Summary for NewsArticle {} // 空的实现

        pub struct Another {}
        impl Summary for Another {} // 空的实现

        // endregion
        // 完整的写法 确定了trait就是 Summary
        pub fn notify<T: Summary>(item: T) {
            println!("Breaking news! {}", item.summarize());
        }

        // 当多个参数为同样的trait 的时候 使用Trait Bounds 更加简略
        let a = NewsArticle {
            headline: String::from(""),
            location: String::from(""),
            author: String::from(""),
            content: String::from(""),
        };
        let b = Another {};
        pub fn notify1(item1: impl Summary, item2: impl Summary) {}
        notify1(a, b);

        let a = NewsArticle {
            headline: String::from(""),
            location: String::from(""),
            author: String::from(""),
            content: String::from(""),
        };
        let b = Another {};
        pub fn notify2<T: Summary>(item1: T, item2: T) {}
        // notify2(a, b); // error a , b 必须是相同的类型

        // 如果想要 NewArticle 中的属性也想要 Summary trait
        // 那当然设置类型为 NewsArticle 就行了 因为 NewsArticle 已经实现了 Summary trait
        pub fn notify3(item: NewsArticle) {
            println!("{} {}", item.author, item.summarize());
        }
    }

    // 通过 + 指定多个 trait
    {
        pub struct NewsArticle {
            pub headline: String,
            pub location: String,
            pub author: String,
            pub content: String,
        }
        pub trait Summary {
            fn a(&self) -> String {
                String::from("(1Read more...)")
            }
            fn b(&self) {}
        }
        pub trait Display {
            fn a(&self) -> String {
                String::from("(2Read more...)")
            }
        }
        impl Summary for NewsArticle {}
        impl Display for NewsArticle {}

        let article = NewsArticle {
            headline: String::from("Penguins win the Stanley Cup Championship!"),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best
    hockey team in the NHL.",
            ),
        };

        pub fn notify1(item: impl Summary + Display) {
            // item.a(); // error 找到多个 a 实现
            item.b();
        }
        pub fn notify2<T: Summary + Display>(item: T) {}

        notify1(article);
    }

    // 通过where 简化代码  trait bound
    {
        pub trait Display {}
        pub trait Debug {}
        fn some_function1<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {
            1
        }

        fn some_function2<T, U>(t: &T, u: U) -> i32
        where
            T: Display + Clone,
            U: Clone + Debug,
        {
            1
        }
    }

    // 返回实现了 trait 的类型
    // 返回一个只是指定了需要实现的 trait 的类型的能力在闭包和迭代器场景十分的有用
    // impl Trait 允许你简单的指定函数返回一个 Iterator 而无需写出实际的冗长的类型。
    // 不能再出现 返回两种类型 但这两种类型都 实现了 trait 因为编译器没法确切判断返回的是哪个
    {
        pub struct Tweet {
            pub username: String,
            pub content: String,
            pub reply: bool,
            pub retweet: bool,
        }
        pub struct Tweet2 {
            pub username: String,
            pub content: String,
            pub reply: bool,
            pub retweet: bool,
        }
        pub trait Summary {
            fn abc(&self) -> String {
                String::from("(1Read more...)")
            }
        }
        impl Summary for Tweet {}
        fn returns_summarizable() -> impl Summary {
            Tweet {
                username: String::from("horse_ebooks"),
                content: String::from("of course, as you probably already know, people"),
                reply: false,
                retweet: false,
            }
        }

        returns_summarizable().abc();
    }

    {
        pub trait Summary {
            fn abc(&self) -> String {
                String::from("(1Read more...)")
            }
        }

        struct Test1 {
            a: i32,
        }
        struct Test2 {
            a: i32,
        }
        struct Test3 {
            b: i32,
        }
        impl Summary for Test1 {}
        impl Summary for Test2 {}
        impl Summary for Test3 {}

        fn switch_return1(switch: bool) -> impl Summary {
            if switch {
                Test1 { a: 1 }
            } else {
                Test1 { a: 2 }
            }
        }

        // error `if` and `else` have incompatible types
        // 不能返回 有相同的 实现了 所需返回类型相同的 trait , 但是 struct不同 的情况
        // fn switch_return2(switch: bool) -> impl Summary {
        //     if switch {
        //         Test1 { a: 1 }
        //     } else {
        //         Test2 { a: 1 }
        //     }
        // }

        switch_return1(true);
    }

    // 使用 trait bounds 来修复 largest 函数
    {
        // 在 largest 函数体中我们想要使用大于运算符（>）比较两个 T 类型的值。
        // 这个运算符被定义为标准库中 trait std::cmp::PartialOrd 的一个默认方法。
        // 所以需要在 T 的 trait bound 中指定 PartialOrd，这样 largest 函数可以用于任何可以比较大小的类型的 slice。
        // 像 i32 和 char 这样的类型是已知大小的并可以储存在栈上，所以他们实现了 Copy trait。
        // 当我们将 largest 函数改成使用泛型后，现在 list 参数的类型就有可能是没有实现 Copy trait 的。

        // &[T] : 为切片类型 现在的可知的有 String Array Vector
        fn largest1<T: PartialOrd + Copy>(list: &[T]) -> T {
            // 所以需要 PartialOrd 和 Copy 两个泛型
            let mut largest = list[0]; // 如果没有 Copy trait 这里就会报错 不能 move
            for &item in list.iter() {
                if item > largest {
                    largest = item;
                }
            }
            largest
        }

        let number_list = vec![34, 50, 25, 100, 65];
        let result = largest1(&number_list);
        println!("The largest number is {}", result);
        let char_list = vec!['y', 'm', 'a', 'q'];
        let result = largest1(&char_list);
        println!("The largest char is {}", result);

        // 另一种 largest 的实现方式是返回在 slice 中 T 值的引用。
        // 如果我们将函数返回值从 T 改为 &T 并改变函数体使其能够返回一个引用，
        // 我们将不需要任何 Clone 或 Copy 的 trait bounds 而且也不会有任何的堆分配。
        fn largest3<T: PartialOrd>(list: &[T]) -> &T {
            // 所以需要 PartialOrd 和 Copy 两个泛型
            let mut largest = &list[0];
            for item in list.iter() {
                if item > largest {
                    largest = item;
                }
            }
            largest
        }
        let number_list = vec![String::from("123")];
        let result = largest3(&number_list);
        // number_list.push(String::from("321")); // error
        println!("The largest number is {}", result);
        println!("{number_list:?}");

        let char_list = vec!['y', 'm', 'a', 'q'];
        let result = largest1(&char_list);
        println!("The largest char is {}", result);

        // 另一种 largest 的实现方式是返回在 slice 中 T 值的引用。
        // 如果我们将函数返回值从 T 改为 &T 并改变函数体使其能够返回一个引用，
        // 我们将不需要任何 Clone 或 Copy 的 trait bounds 而且也不会有任何的堆分配。
        fn largest2<T: PartialOrd>(list: &[T]) -> &T {
            // 所以需要 PartialOrd 和 Copy 两个泛型
            let mut largest_index: usize = 0;
            // 存储的是 最大的的值的索引 那就不用保证 T 的值类型能够copy了
            for index in 0..list.len() - 1 {
                if list[index] > list[largest_index] {
                    largest_index = index;
                }
            }
            return &list[largest_index];
        }

        let number_list = vec![34, 50, 25, 100, 65];

        let result = largest1(&number_list);
        println!("The largest number is {}", result);
        let char_list = vec!['y', 'm', 'a', 'q'];

        let result = largest1(&char_list);
        println!("The largest char is {}", result);
    }

    // 使用 trait bound 有条件的实现方法
    // 通过使用带有 trait bound 的泛型参数的 impl 块，可以有条件地只为那些实现了特定 trait 的类型实现方法。
    {
        use std::fmt::Display;
        struct Pair<T> {
            x: T,
            y: T,
        }
        impl<T> Pair<T> {
            fn new(x: T, y: T) -> Self {
                Self { x, y }
            }
        }
        // 只有那些为 T 类型实现了 PartialOrd trait（来允许比较）和 Display trait（来启用打印）
        // 的 Pair<T> 才会实现 cmp_display 方法
        impl<T: Display + PartialOrd> Pair<T> {
            fn cmp_display(&self) {
                if self.x >= self.y {
                    println!("The largest member is x = {}", self.x);
                } else {
                    println!("The largest member is y = {}", self.y);
                }
            }
        }

        // 也可以对任何实现了特定 trait 的类型有条件地实现 trait。
        // 对任何满足特定 trait bound 的类型实现 trait 被称为 blanket implementations，他们被广泛的用于 Rust 标准库中。
        // 例如，标准库为任何实现了 Display trait 的类型实现了 ToString trait。这个 impl 块看起来像这样：
        // impl<T: Display> ToString for T {
        //     // --snip--
        // }
        //
        // 完整的是这样的
        // impl<T: fmt::Display + ?Sized> ToString for T {
        //     // --snip--
        // }

        // 因为标准库有了这些 blanket implementation，我们可以对任何实现了 Display trait 的类型调用由 ToString 定义的 to_string 方法。
        let s = 3.to_string();
    }
}

pub fn ch10_03_lifetime_syntax() {
    // 函数中的泛型生命周期
    /*{
        // 提示文本揭示了返回值需要一个泛型生命周期参数，因为 Rust 并不知道将要返回的引用是指向 x 或 y。
        // 事实上我们也不知道，因为函数体中 if 块返回一个 x 的引用而 else 块返回一个 y 的引用！
        fn longest(x: &str, y: &str) -> &str {
            if x.len() > y.len() {
                x
            } else {
                y
            }
        }
        let string1 = String::from("abcd");
        let string2 = "xyz";

        let result = longest(string1.as_str(), string2);
        println!("The longest string is {}", result);
    }*/

    // 生命周期注解语法
    // 生命周期注解并不改变任何引用的生命周期的长短。
    // 与当函数签名中指定了泛型类型参数后就可以接受任何类型一样，当指定了泛型生命周期后函数也能接受任何生命周期的引用。
    // 生命周期注解描述了多个引用生命周期相互的关系，而不影响其生命周期。

    // 生命周期注解有着一个不太常见的语法：生命周期参数名称必须以撇号（'）开头，其名称通常全是小写，类似于泛型其名称非常短。
    // 'a 是大多数人默认使用的名称。
    // 生命周期参数注解位于引用的 & 之后，并有一个空格来将引用类型与生命周期注解分隔开。
    {
        // 现在函数签名表明对于某些生命周期 'a，函数会获取两个参数，他们都是与生命周期 'a 存在的一样长的字符串 slice。
        // 函数会返回一个同样也与生命周期 'a 存在的一样长的字符串 slice。
        // 注意 longest 函数并不需要知道 x 和 y 具体会存在多久，而只需要知道有某个可以被 'a 替代的作用域将会满足这个签名。
        fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
            if x.len() > y.len() {
                x
            } else {
                y
            }
        }
        // 泛型生命周期 'a 的具体生命周期等同于 x 和 y 的生命周期中较小的那一个。
        // 因为我们用相同的生命周期参数 'a 标注了返回的引用值，所以返回的引用值就能保证在 x 和 y 中 *较短* 的那个生命周期结束之前保持有效。
        let string1 = String::from("abcd");
        let string2 = "xyz";

        let result = longest(string1.as_str(), string2);
        let result = longest(&string1, &string2);
        println!("The longest string is {}", result);
        println!("{},{}", string1, string2);
    }

    // 函数签名中的生命周期注解
    {
        fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
            if x.len() > y.len() {
                x
            } else {
                y
            }
        }
        let string1 = String::from("long string is long");
        let result;
        {
            let string2 = String::from("xyz");
            result = longest(string1.as_str(), string2.as_str());
        } // string2 生命周期在这里结束 要短于 string1
          // 然而，我们通过生命周期参数告诉 Rust 的是： longest 函数返回的引用的生命周期应该与传入参数的生命周期中较短那个保持一致。
          // println!("The longest string is {}", result); // error
          // 较短的 生命周期是 string2 已经结束了 result 引用较短的那个 在这里已经没有了
    }

    // 深入理解生命周期
    {
        // 返回类型是引用 但是只和 x 有关系只指定 x 和 返回的 生命周期
        fn longest1<'a>(x: &'a str, y: &str) -> &'a str {
            x
        }
        fn longest3<'a>(x: &'a str, y: &'a str) -> &'a str {
            x
        }

        // 当从函数返回一个引用，返回值的生命周期参数需要与一个参数的生命周期参数相匹配。
        // 如果返回的引用 没有 指向任何一个参数，那么唯一的可能就是它指向一个函数内部创建的值，它将会是一个悬垂引用，因为它将会在函数结束时离开作用域。
        // fn longest2<'a>(x: &str, y: &str) -> &'a str {
        //     let result = String::from("really long string");
        //     result.as_str()
        //      ------^^^^^^^^^
        //      |             |
        //      |             returns a value referencing data owned by the current function
        //      |             `result` is borrowed here
        // }

        // 出现的问题是 result 在 longest 函数的结尾将离开作用域并被清理，而我们尝试从函数返回一个 result 的引用。
        // 无法指定生命周期参数来改变悬垂引用，而且 Rust 也不允许我们创建一个悬垂引用。在这种情况，
        // 最好的解决方案是返回一个**有所有权**的数据类型而不是一个引用，这样函数调用者就需要负责清理这个值了。

        // 既然没有
        fn longest2<'a>(x: &str, y: &str) -> String {
            let result = String::from("really long string");
            result
        }
    }

    // 结构体定义中的生命周期注解
    {
        // 这个结构体有一个字段，part，它存放了一个字符串 slice，这是一个引用。
        // 类似于泛型参数类型，必须在结构体名称后面的尖括号中声明泛型生命周期参数
        struct ImportantExcerpt<'a> {
            part: &'a str,
        }
        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence /* :&str */ = novel.split('.')
            .next()
            .expect("Could not find a '.'");
        // 函数创建了一个 ImportantExcerpt 的实例，它存放了变量 novel 所拥有的 String 的第一个句子的引用。
        // novel 的数据在 ImportantExcerpt 实例创建之前就存在。
        // 直到 ImportantExcerpt 离开作用域之后 novel 都不会离开作用域，
        // 所以 ImportantExcerpt 实例中的引用是有效的。
        let i = ImportantExcerpt {
            part: first_sentence,
        };
    }
    // 生命周期省略
    {
        // 没有生命周期 但是可以编译成功
        fn first_word(s: &str) -> &str {
            let bytes = s.as_bytes();
            for (i, &item) in bytes.iter().enumerate() {
                if item == b' ' {
                    return &s[0..i];
                }
            }
            &s[..]
        }
        // 被编码进 Rust 引用分析的模式被称为 生命周期省略规则（lifetime elision rules）。
        // 这并不是需要程序员遵守的规则；
        // 这些规则是一系列特定的场景，此时编译器会考虑，如果代码符合这些场景，就无需明确指定生命周期。

        // 省略规则并不提供完整的推断：如果 Rust 在明确遵守这些规则的前提下变量的生命周期仍然是模棱两可的话，
        // 它不会猜测剩余引用的生命周期应该是什么。
        // 在这种情况，编译器会给出一个错误，这可以通过增加对应引用之间相联系的生命周期注解来解决。

        // 函数或方法的参数的生命周期被称为 输入生命周期（input lifetimes），
        // 而返回值的生命周期被称为 输出生命周期（output lifetimes）。

        // 编译器采用三条规则来判断引用何时不需要明确的注解。
        // 第一条规则适用于输入生命周期，后两条规则适用于输出生命周期。
        // 如果编译器检查完这三条规则后仍然存在没有计算出生命周期的引用，
        // 编译器将会停止并生成错误。这些规则适用于 fn 定义，以及 impl 块。

        // 1. 第一条规则是每一个是引用的参数都有它自己的生命周期参数。换句话说就是，
        // 有一个引用参数的函数有一个生命周期参数：fn foo<'a>(x: &'a i32)，
        // 有两个引用参数的函数有两个不同的生命周期参数，fn foo<'a, 'b>(x: &'a i32, y: &'b i32)，依此类推。

        // 2. 第二条规则是如果只有一个输入生命周期参数，
        // 那么它被赋予所有输出生命周期参数：fn foo<'a>(x: &'a i32) -> &'a i32。

        // 3. 第三条规则是如果方法有多个输入生命周期参数并且其中一个参数是 &self 或 &mut self，
        // 说明是个对象的方法(method),
        // 那么所有输出生命周期参数被赋予 self 的生命周期。
        // 第三条规则使得方法更容易读写，因为只需更少的符号。

        // fn first_word(s: &str) -> &str {
        // 按照第一条规则转换为
        // fn first_word<'a>(s: &'a str) -> &str {
        // 按照第二条规则
        // fn first_word<'a>(s: &'a str) -> &'a str {
        // 所以只有一个参数的 且输入输出 都是引用的情况 默认可以省略 生命周期

        // fn longest(x: &str, y: &str) -> &str {
        // 根据第一条规则
        // fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str {
        // 至此 编译器不能推断 返回值的生命周期 所以需要手动添加
    }

    // 声明和使用生命周期参数的位置依赖于生命周期参数是否同结构体字段或方法参数和返回值相关。
    {
        // 生命周期可省略
        struct ImportantExcerpt {}
        impl ImportantExcerpt {
            fn level(&self) -> i32 {
                3
            }
        }
        // impl<'a> ImportantExcerpt<'a> {
        //     fn level(&self) -> i32 {
        //         3
        //     }
        // }
    }
    {
        struct ImportantExcerpt<'a> {
            part: &'a str,
        }

        // impl 之后和类型名称之后的生命周期参数是必要的，
        // 不过因为第一条生命周期规则我们并不必须标注 self 引用的生命周期。
        impl<'a> ImportantExcerpt<'a> {
            fn level(&self) -> i32 {
                3
            }
            fn announce_and_return_part(&self, announcement: &str) -> &str {
                println!("Attention please: {}", announcement);
                self.part
            }

            // fn announce_and_return_part(&'a self, announcement: &'a str) -> &'a str {
            //     println!("Attention please: {}", announcement);
            //     self.part
            // }
        }

        // 用不到 ImportantExcerpt 的生命周期 的时候的简写
        impl ImportantExcerpt<'_> {
            fn a(&self, announcement: &str) -> &str {
                self.part
            }
        }
    }

    // 静态生命周期
    // 这里有一种特殊的生命周期值得讨论：'static，其生命周期存活于整个程序期间。
    // 所有的字符串字面值都拥有 'static 生命周期，我们也可以选择像下面这样标注出来：
    {
        let mut s: &'static str = "I have a static lifetime.";
        // 这个字符串的文本被直接储存在程序的二进制文件中而这个文件总是可用的。因此所有的字符串字面值都是 'static 的。
    }

    // 结合泛型类型参数 trait bounds 和 生命周期
    {
        use std::fmt::Display;

        fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
        where
            T: Display,
        {
            println!("Announcement! {}", ann);
            if x.len() > y.len() {
                x
            } else {
                y
            }
        }

        longest_with_an_announcement("1", "2", "3");
    }
}
