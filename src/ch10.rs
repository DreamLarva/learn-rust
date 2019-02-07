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
        enum Option<T>{
            Some(T),
            None,
        }
        enum Result<T,E> {
            Ok(T),
            Err(E),
        }
    }


}