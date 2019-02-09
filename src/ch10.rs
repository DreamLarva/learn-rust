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
    }



}