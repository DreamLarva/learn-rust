use std::rc::Rc;

pub fn ch17_01whatis_oo() {
    pub struct AveragedCollection {
        list: Vec<i32>,
        average: f64,
    }

    impl AveragedCollection {
        pub fn add(&mut self, value: i32) {
            self.list.push(value);
            self.update_average();
        }

        pub fn remove(&mut self) -> Option<i32> {
            let result = self.list.pop();
            match result {
                Some(Value) => {
                    self.update_average();
                    Some(Value)
                }
                None => None,
            }
        }

        pub fn average(&self) -> f64 {
            self.average
        }

        fn update_average(&mut self) {
            let total: i32 = self.list.iter().sum();
            self.average = total as f64 / self.list.len() as f64
        }
    }
}

pub fn ch17_02_trait_objects() {
    pub trait Draw {
        fn draw(&self);
    }

    // 这与定义使用了带有 trait bound 的泛型类型参数的结构体不同。
    // 泛型类型参数一次只能替代一个具体类型，而 trait 对象则允许在运行时替代多种具体类型。
    pub struct Screen {
        pub components: Vec<Box<dyn Draw>>,
    }

    impl Screen {
        pub fn run(&self) {
            for component in self.components.iter() {
                component.draw();
            }
        }
    }

    // region 使用 trait bound(不正确)
    {
        // 这限制了 Screen 实例必须拥有一个全是 Button 类型或者全是 TextField 类型的组件列表。
        // 如果只需要同质（相同类型）集合，则倾向于使用泛型和 trait bound，
        // 因为其定义会在编译时采用具体类型进行单态化。
        pub struct Screen<T: Draw> {
            // T 一次只能指代一种类型
            pub components: Vec<T>,
        }

        impl<T> Screen<T>
        where
            T: Draw,
        {
            pub fn run(&self) {
                for component in self.components.iter() {
                    // 这里的 component 只能是一种类型
                    component.draw();
                }
            }
        }
    }
    // endregion

    pub struct Button {
        pub width: u32,
        pub height: u32,
        pub label: String,
    }

    impl Draw for Button {
        fn draw(&self) {
            println!("Button draw");
        }
    }

    struct SelectBox {
        width: u32,
        height: u32,
        options: Vec<String>,
    }

    impl Draw for SelectBox {
        fn draw(&self) {
            println!("SelectBox draw");
        }
    }

    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();

    // trait 对象执行动态分发
    // 当对泛型使用 trait bound 时编译器所进行单台化处理: 编译器为每一个被泛型类型参数代替的具体类型生成了非泛型的函数和方法实现.
    // 单态化所产生的代码进行 静态分发(static dispatch). 静态分发发生于编译器在编译时就知晓调用了什么方法的时候.
    // 这与动态分发(dynamic dispatch)相对,这是编译器在编译时无法知晓调用了什么方法.
    // 在动态分发的情况下,编译器会生成在运行时去调用了什么方法的代码的代码.

    // 当使用 trait 对象时,Rust 必须使用动态分发.编译器无法知晓所有可能用于 trait对象代码的类型,所以它也不知道应该调用那个类型的哪个方法实现.
    // 为此 Rust在运行时使用trait 对象中的指针来知晓需要调用哪个方法.动态分发也阻止编译器有选择的内联方法和代码,
    // 这会相应的禁用一些优化.

    // Trait 对象要求对象安全
    // 只有对象安全(object safe)的trait 才可以组成 trait对象.围绕所有是的trait 对象安全的属性 存在一些付复杂的规则
    // 不过在实践中,直接寄两条规则.如果一个 trait 中所有的方法有如下属性时,则该 trait是对象安全的.
    //  1. 返回值的类型不为 Self
    //  2. 方法没有任何泛型类型参数
    // Self关键字是我们要实现 trait 或方法的类型的别名.对象安全对于 trait对象是必须的,
    // 因为一旦有了 trait 对象,就不再知晓实现该 trait的对象的具体类型是什么了. 如果trait方法返回具体的Self类型,
    // 但是 trait 对象忘记了真正的类型,那么方法不可能使用已经忘记却的原始具体类型.
    // 同理对于泛型类型参数来说,当使用trait 对象时具体类型被抹去了,故无从得知放入泛型参数类型的类型是什么.
    {
        // 一个 trait 方法不是安全的列子就是 标准库中的 Clone trait.
        pub trait Clone {
            fn clone(&self) -> Self;
        }

        // String 实现了 Clone trait,当在String 实例上调用 clone 方法时 会得到一个 string 实例.
        // 类似的, 当调用 Vec<T>实例的clone 方法会得到一个 Vec<T>实例. clone 的签名需要知道什么类型代替 Self,因为这是他的返回值.

        // error
        // pub struct Screen {
        //     pub components: Vec<Box<dyn Clone>>
        // }
        // consider moving `clone` to another trait
        // note: for a trait to be "object safe" it needs to allow building a vtable to allow the call to be resolvable dynamically;
        // for more information visit <https://doc.rust-lang.org/reference/items/traits.html#object-safety>

        // This trait is object-safe, but these methods cannot be dispatched on a trait object.
        /*{
            trait NonDispatchable {
                // Non-methods cannot be dispatched.
                fn foo()
                where
                    Self: Sized,
                {
                }
                // Self type isn't known until runtime.
                fn returns(&self) -> Self
                where
                    Self: Sized;
                // `other` may be a different concrete type of the receiver.
                fn param(&self, other: Self)
                where
                    Self: Sized,
                {
                }
                // Generics are not compatible with vtables.
                fn typed<T>(&self, x: T)
                where
                    Self: Sized,
                {
                }
            }

            struct S;
            impl NonDispatchable for S {
                fn returns(&self) -> Self
                where
                    Self: Sized,
                {
                    S
                }
            }
            let obj: Box<dyn NonDispatchable> = Box::new(S);
            obj.returns(); // ERROR: cannot call with Self return
            obj.param(S); // ERROR: cannot call with Self parameter
            obj.typed(1); // ERROR: cannot call with generic type
        }*/

        // Examples of non-object safe traits.
        /*{
            trait NotObjectSafe {
                const CONST: i32 = 1; // ERROR: cannot have associated const

                fn foo() {} // ERROR: associated function without Sized
                fn returns(&self) -> Self; // ERROR: Self in return type
                fn typed<T>(&self, x: T) {} // ERROR: has generic type parameters
                fn nested(self: Rc<Box<Self>>) {} // ERROR: nested receiver not yet supported
            }

            struct S;
            impl NotObjectSafe for S {
                fn returns(&self) -> Self {
                    S
                }
            }
            let obj: Box<dyn NotObjectSafe> = Box::new(S); // ERROR
        }*/
    }
}

// 面向对象设计模式的实现
pub fn ch17_03_oo_design_patterns() {
    /*
        实现一个增量式的发布博文的工作流。这个博客的最终功能看起来像这样：
            1. 博文从空白的草案开始。
            2. 一旦草案完成，请求审核博文。
            3. 一旦博文过审，它将被发表。
            4. 只有被发表的博文的内容会被打印，这样就不会意外打印出没有被审核的博文的文本。
    */
    {
        struct Post {
            state: Option<Box<dyn State>>,
            content: String,
        }

        impl Post {
            pub fn new() -> Post {
                Post {
                    state: Some(Box::new(Draft {})),
                    content: String::new(),
                }
            }

            // add_text 的方法并向其传递一个 &str 来将文本增加到博文的内容中
            pub fn add_text(&mut self, text: &str) {
                self.content.push_str(text);
            }

            // 选择实现为一个方法而不是将 content 字段暴露为 pub,
            // 这意味着之后可以实现一个方法来控制 content 字段如何被读取
            pub fn content(&self) -> &str {
                // 这里调用 Option 的 as_ref 方法是因为需要 Option 中值的引用而不是获取其所有权
                // 因为 state 是 一个 Option<Box<State>> 调用 as_ref 返回一个 Option<&Box<State>>
                // 如果调用as_ref 将会得到一个错误,因为不能将 state 移除借用的&self 函数
                self.state.as_ref().unwrap().content(self)

                // self.state.unwrap().content(self) // error
                // move occurs because `self.state` has type `std::option::Option<Box<dyn State>>`,
                // which does not implement the `Copy` trait
            }

            pub fn request_review(&mut self) {
                // 调用 take 方法将 state 字段中的 Some 值取出并留下一个 None，因为 Rust 不允许在结构体中存在空的字段
                if let Some(s) = self.state.take() {
                    self.state = Some(s.request_review())
                }
            }

            pub fn approve(&mut self) {
                if let Some(s) = self.state.take() {
                    self.state = Some(s.approve())
                }
            }

            // 增加 reject 方法将博文的状态从 PendingReview 变回 Draft
            pub fn reject(&mut self) {
                if let Some(s) = self.state.take() {
                    self.state = Some(s.reject())
                }
            }
        }

        // State trait 定义了所有不同状态的博文所共享的行为，同时 Draft、PendingReview 和 Published 状态都会实现 State 状态。
        trait State {
            fn request_review(self: Box<Self>) -> Box<dyn State>;
            fn approve(self: Box<Self>) -> Box<dyn State>;
            // 增加 reject 方法将博文的状态从 PendingReview 变回 Draft
            fn reject(self: Box<Self>) -> Box<dyn State>;
            fn content<'a>(&self, post: &'a Post) -> &'a str {
                ""
            }
        }

        struct Draft {}

        impl State for Draft {
            fn request_review(self: Box<Self>) -> Box<dyn State> {
                Box::new(PendingReview { approve_time: 0 })
            }

            fn approve(self: Box<Self>) -> Box<dyn State> {
                self
            }

            fn reject(self: Box<Self>) -> Box<dyn State> {
                self
            }
        }

        struct PendingReview {
            approve_time: i32,
        }

        impl State for PendingReview {
            fn request_review(self: Box<Self>) -> Box<dyn State> {
                self
            }

            fn approve(mut self: Box<Self>) -> Box<dyn State> {
                // 在将状态变为 Published 之前需要两次 approve 调用
                self.approve_time += 1;

                if self.approve_time == 2 {
                    Box::new(Published {})
                } else {
                    self
                }
            }

            fn reject(self: Box<Self>) -> Box<dyn State> {
                Box::new(Draft {})
            }
        }

        struct Published {}

        impl State for Published {
            fn request_review(self: Box<Self>) -> Box<dyn State> {
                self
            }
            fn approve(self: Box<Self>) -> Box<dyn State> {
                self
            }

            fn reject(self: Box<Self>) -> Box<dyn State> {
                self
            }

            fn content<'a>(&self, post: &'a Post) -> &'a str {
                &post.content
            }
        }

        let mut post = Post::new();

        post.add_text("I ate a salad for lunch today");
        assert_eq!("", post.content());

        post.request_review();
        assert_eq!("", post.content());

        post.reject();
        assert_eq!("", post.content());

        post.request_review();
        assert_eq!("", post.content());

        post.approve();
        post.approve();
        assert_eq!("I ate a salad for lunch today", post.content());
    }

    // 状态模式的权衡取舍
    {
        pub struct Post {
            content: String,
        }
        pub struct DraftPost {
            content: String,
        }
        impl Post {
            pub fn new() -> DraftPost {
                DraftPost {
                    content: String::new(),
                }
            }
            pub fn content(&self) -> &str {
                &self.content
            }
        }

        impl DraftPost {
            pub fn add_text(&mut self, text: &str) {
                self.content.push_str(text);
            }

            pub fn request_review(self) -> PendingReviewPost {
                PendingReviewPost {
                    content: self.content,
                }
            }
        }

        pub struct PendingReviewPost {
            content: String,
        }

        impl PendingReviewPost {
            pub fn approve(self) -> Post {
                Post {
                    content: self.content,
                }
            }
        }

        let mut post = Post::new();
        post.add_text("I ate a salad for lunch today");
        let post = post.request_review();
        let post = post.approve();
        assert_eq!("I ate a salad for lunch today", post.content());
    }
}
