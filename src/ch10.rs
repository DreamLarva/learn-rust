#![allow(unused_variables)] // 不对 未使用的变量 warning


pub fn ch10_01_syntax() {
    // 结构体中定义的泛型
    {
        struct Point<T> {
            x: T,
            y: T,
        }

        let integer = Point { x: 5, y: 10 };
//        let float = Point { x: 1.0, y: 4.0 };
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
        // 注意必须在 impl 后面声明 T,，这样就可以在 Point<T> 上实现的方法中使用它了
        impl<T> Point<T> {
            fn x(&self) -> &T {
                &self.x
            }
        }
        // 给予是 f32 类型的实例才有的方法(有点像重载)
        impl Point<f32> {
            fn distance_from_origin(&self) -> f32 {
                (self.x.powi(2) + self.y.powi(2)).sqrt()
            }
        }

        let p = Point { x: 5, y: 10 };
        let q = Point { x: 5.0, y: 10.0 };

        println!("p.x = {}", p.x());
//        println!("p.x = {}", p.distance_from_origin()); // 报错
        println!("p.x = {}", q.distance_from_origin());
    }

    // 结构体定义中的泛型类型参数并不总是与结构体方法签名中使用的泛型是同一类型
    {
        struct Point<T, U> {
            x: T,
            y: U,
        }
        impl<T, U> Point<T, U> {
            // 来自结构体的泛型
            fn mix_up<V, W>(self, other: Point<V, W>) -> Point<T, W> { // 来自函数的泛型
                Point {
                    x: self.x, // self Point 的 T
                    y: other.y, // Other Point 的 W
                }
            }
        }
        let p1 = Point { x: 5, y: 10.4 };
        let p2 = Point { x: "Hello", y: 'c' };

        let p3 = p1.mix_up(p2);

        println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
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

    // 默认实现
    {
        // 有时为 trait 中的某些或全部方法提供默认的行为，而不是在每个类型的每个实现中都定义自己的行为是很有用的。这样当为某个特定类型实现 trait 时，可以选择保留或重载每个方法的默认行为。
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
        let article = NewsArticle {
            headline: String::from("Penguins win the Stanley Cup Championship!"),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from("The Pittsburgh Penguins once again are the best
    hockey team in the NHL."),
        };

        println!("New article available! {}", article.summarize());
    }
    // 重载一个默认的实现
    // 注意无法从相同方法的重载实现中调用默认方法。
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
                format!("(Read more from {}...)", self.summarize_author())
            }
        }

        impl Summary for Tweet {
            fn summarize_author(&self) -> String {
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
            content: String::from("The Pittsburgh Penguins once again are the best
    hockey team in the NHL."),
        };

        pub fn notify(item: impl Summary) { // 传入一个已经实现了 Summary的实例
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

        // endregion
        // 完整的写法 确定了trait就是 Summary
        pub fn notify<T: Summary>(item: T) {
            println!("Breaking news! {}", item.summarize());
        }

        // 当多个参数为同样的trait 的时候 使用Trait Bounds 更加简略
        pub fn notify1(item1: impl Summary, item2: impl Summary) {}
        pub fn notify2<T: Summary>(item1: T, item2: T) {}
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
            content: String::from("The Pittsburgh Penguins once again are the best
    hockey team in the NHL."),
        };

        pub fn notify1(item: impl Summary + Display) {
//            item.a(); // error 找到多个 a 实现
        }
        pub fn notify2<T: Summary + Display>(item: T) {}

//        notify1(article)
    }

    // 返回trait
    // 不能再出现 返回两种类型 但这两种类型都 实现了 trait 因为编译器没法确切判断返回的是哪个
    {
        pub struct Tweet {
            pub username: String,
            pub content: String,
            pub reply: bool,
            pub retweet: bool,
        }
        pub trait Summary {
            fn a(&self) -> String {
                String::from("(1Read more...)")
            }
        }
        impl Summary for Tweet {};
        fn returns_summarizable() -> impl Summary {
            Tweet {
                username: String::from("horse_ebooks"),
                content: String::from("of course, as you probably already know, people"),
                reply: false,
                retweet: false,
            }
        }
    }


    {
        // 在 largest 函数体中我们想要使用大于运算符（>）比较两个 T 类型的值。
        // 这个运算符被定义为标准库中 trait std::cmp::PartialOrd 的一个默认方法。
        // 所以需要在 T 的 trait bound 中指定 PartialOrd，这样 largest 函数可以用于任何可以比较大小的类型的 slice。
        // 像 i32 和 char 这样的类型是已知大小的并可以储存在栈上，所以他们实现了 Copy trait。
        // 当我们将 largest 函数改成使用泛型后，现在 list 参数的类型就有可能是没有实现 Copy trait 的。

        // &[T] : 为切片类型 现在的可知的有 String Array Vector
        fn largest1<T: PartialOrd + Copy>(list: &[T]) -> T { // 所以需要 PartialOrd 和 Copy 两个泛型
            let mut largest = list[0];
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
        // 如果我们将函数返回值从 T 改为 &T 并改变函数体使其能够返回一个引用，我们将不需要任何 Clone 或 Copy 的 trait bounds 而且也不会有任何的堆分配。
        fn largest2<T: PartialOrd>(list: &[T]) -> &T { // 所以需要 PartialOrd 和 Copy 两个泛型
            let mut largest_index:usize = 0;
            // 存储的是 最大的的值的索引 那就不用保证 T 的值类型能够copy了
            for index in 0..list.len() - 1 {
                if list[index] > list[largest_index]{
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
    // 通过where 简化代码
    {
        pub trait Display {}
        fn some_function<T, U>(t: T, u: U)
            where T: Display,
                  U: Clone
        {}
    }
    // 使用 trait bound 有条件的实现方法
    {
        use std::fmt::Display;
        struct Pair<T> {
            x: T,
            y: T,
        }
        impl<T> Pair<T> {
            fn new(x: T, y: T) -> Self {
                Self {
                    x,
                    y,
                }
            }
        }
        // 为T泛型实现了 Display 和partOrd 的实现Pair
        impl<T: Display + PartialOrd> Pair<T> {
            fn cmp_display(&self) {
                if self.x >= self.y {
                    println!("The largest member is x = {}", self.x);
                } else {
                    println!("The largest member is y = {}", self.y);
                }
            }
        }
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
    // 生命周期注解并不改变任何引用的生命周期的长短。与当函数签名中指定了泛型类型参数后就可以接受任何类型一样，当指定了泛型生命周期后函数也能接受任何生命周期的引用。
    // 生命周期注解描述了多个引用生命周期相互的关系，而不影响其生命周期。

    // 生命周期注解有着一个不太常见的语法：生命周期参数名称必须以撇号（'）开头，其名称通常全是小写，类似于泛型其名称非常短。'a 是大多数人默认使用的名称。
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
        // 因为我们用相同的生命周期参数 'a 标注了返回的引用值，所以返回的引用值就能保证在 x 和 y 中较短的那个生命周期结束之前保持有效。
        let string1 = String::from("abcd");
        let string2 = "xyz";

        let result = longest(string1.as_str(), string2);
//        let result = longest(&string1.as_str(), &string2); // todo 也能够执行 强制类型转换?
        println!("The longest string is {}", result);
        println!("{},{}", string1,string2);
    }

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
//         println!("The longest string is {}", result); // error
        // 较短的 生命周期是 string2 已经结束了 result 引用较短的那个 在这里已经没有了
    }

    {
        struct ImportantExcerpt<'a> {
            part: &'a str,
        }
        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.')
            .next()
            .expect("Could not find a '.'");
        let i = ImportantExcerpt { part: first_sentence };
    }
}