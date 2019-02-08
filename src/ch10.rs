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
        // 给予是 f32 类型才有的方法(有点像重载)
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
}