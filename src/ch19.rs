use std::cmp::Ordering;
use std::fmt::{Arguments, Formatter};
use std::io;
use std::panic::RefUnwindSafe;

pub fn ch19_01_unsafe_rust() {
    // 不安全的超能力
    // 可通过 unsafe 关键字来切换到 不安全Rust.
    // 这里有5类可以在不安全Rust中惊醒而不能用与安全Rust 的操作
    // 1. 解引用裸指针
    // 2. 调用不安全的函数或方法
    // 3. 访问或修改可变静态变量
    // 4. 实现不安全trait
    // 5. 访问union的字段
    // unsafe 并不会关闭借用检查器或禁用其他Rust安全检查:如果在不安全代码中使用引用,它人会被检查.
    // unsafe 关键字只是提供了那五个不会被编译器内存检查内存安全的功能.
    // 再者,unsafe 不意味着开中的代码一定是危险的或是边然导致内存安全问题:
    // 其意图在于作为程序员你会将确保unsafe块中的代码以有效的方式访问内存

    // 人是会犯错误的,错误总会发生,不过通过要求这五类操作必须位于标记为 unsafe 的块中,
    // 就能够知道任何与内存安全相关的错误必定位于 unsafe块中. 保持unsafe 块尽可能小,如此之后检查内存bug时会感谢你自己
    // 为了尽可能隔离不安全代码.将不安全代码封装进一个安全的抽象并提供安全API是一个好主意,
    // 当我们学习不安全函数和方法时会讨论到.
    // 标砖库的一部分被实现为在评审过得不安全代码智商的安全抽象.
    // 这个技术防止了 unsafe 泄露到所有你活着用户希望使用由 unsafe代码实现的功能的地方,因为使用其安全抽象是安全的

    // 解引用裸指针
    {
        // 不安全的Rust 有两个被称为裸指针(raw pointers)的类似于 引用的新类型
        // 和引用一样,裸指针是不可变或可变的,分别写作
        // *const T 和 *mut T
        // 这里的星号不是解引用运算符;他是类型名称的一部分.
        // 它在裸指针上下文中,不可变意味着 指针解引用之后不能直接赋值.
        // 与引用和智能指针的区别在于
        // 1. 允许忽略借用规则,可以同时拥有不可变和可变的指针,或多个指向同位置的可变指针
        // 2. 不保证指向有效的内存
        // 3. 允许为空
        // 4. 不能实现任何自动清理功能
        // 通过去掉Rust强加的保证,你可
        // 以放弃安全保证以换取性能或使用另一个语言或硬件接口的能力,此时Rust的保证不适用
        {
            let mut num = 5;
            // 常见一个指针不会造成任何危险; 只有当访问其指向的值时才有可能遇到无效的值.
            let r1 = &num as *const i32;
            let r2 = &mut num as *mut i32;
            // 同时创建了 可变指针和 不可变指针

            // 下面正常的下法 是违反 借用规则的
            // let r1 = &num;
            // let r2 = &mut num;

            unsafe {
                println!("r1 is: {}", *r1);
                println!("r2 is: {}", *r2);
            }

            println!("r1 is: {:?}", r1); // r1 is: 0x7ffedfd180dc
            println!("r2 is: {:?}", r2); // r2 is: 0x7ffedfd180dc
        }
    }

    // 调用不安全的
    // 第二类要求使用不安全块的操作是调用不安全函数.
    // 在此上下文中, 关键字unsafe块中调用不安全函数,表明我们已经阅读过次函数的文档并对其是否满足函数自身的契约负责.
    {
        unsafe fn dangerous() {}
        // 只能在 unsafe 中调用
        unsafe {
            // 通过将 dangerous 调用插入unsafe 块中,
            // 我们就向 Rust 保证了我们已经阅读过函数的文档,理解如何正确使用,并验证过其满足函数的契约
            dangerous();
        }
    }

    // 创建不安全带的安全抽象
    // 仅仅因为函数包含不安全代码并不意味着整个函数都需要标记为不安全的.
    // 事实上,将不安全代码封装进安全函数是一个常见的抽象.
    // 作为一个例子,标准中的函数, split_mut,他需要一些不安全代码,让我们探索如何可以实现它.
    // 整个安全函数定义于 可变slice之上:它将获取一个slice并从给定的索引参数开始将其分为两个slice.

    {
        // 下面代码 不符合 借用规则
        // Rust 的借用检查器不能理解我们要借用这个 slice 的两个不同部分：
        // 它只知道我们借用了同一个 slice 两次。本质上借用 slice 的不同部分是可以的，
        // 因为结果两个 slice 不会重叠，不过 Rust 还没有智能到能够理解这些。
        // fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        //     let len = slice.len();
        //     assert!(mid <= len);
        //     (&mut slice[..mid], &mut slice[mid..])
        // }

        use std::slice;
        fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
            let len = slice.len(); // 获取slice 长度

            // as_mut_ptr 返回一个 *mut i32 类型的裸指针，储存在 ptr 变量中
            let ptr = slice.as_mut_ptr();
            // 我们保持索引 mid 位于 slice 中的断言。
            assert!(mid <= len);

            unsafe {
                // slice::from_raw_parts_mut 函数获取一个裸指针和一个长度来创建一个 slice
                (
                    slice::from_raw_parts_mut(ptr, mid),
                    slice::from_raw_parts_mut(ptr.add(mid), len - mid),
                )
            }

            // slice::from_raw_parts_mut 函数是不安全的因为它获取一个裸指针,并必须确信这个指针是有效的.
            // 裸指针上的add 方法也是不安全的,因为其必须确信此地址偏移量也是有效的指针.
            // 因此必须将slice::from_raw_parts_mut 和 add 放入unsafe 块中以便能调用他们.
            // 通过观察代码和 增加 mid 必然小于等于len 的断言,我们可以说 unsafe 块中的裸指针将是有效的slice中数据的指针
        }

        // slice::from_raw_parts_mut 在使用 slice 时很有可能会崩溃
        {
            use std::slice;
            let address = 0x01234usize;
            let r = address as *mut i32;
            let slice: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10000) };
            // 我们并不拥有这个任意地址的内存,也不能保证这段代码创建的slice包含有效的i32值
        }
    }

    // 使用extern 函数调用外部代码
    // 有时 你的 Rust代码可能需要与其他语言编写的代码交互.
    // 为此 Rust 有一个关键字,extern,有助于创建和使用 外部函数接口(Foreign Function Interface, FFI)
    // 外部函数接口是一个编程语言以定义函数的方式,其不允许不同(外部)编程语言调用这些函数.

    // 集成 C 标准库中的 abs 函数
    // extern 块中声明的函数在啊Rust 代码中总是不安全的. 因为其他语言不会强制执行rust的规则
    // 定义了 外部函数所使用的 应用二进制的接口(application binary interface, ABI) --ABI定义了如何在汇编语言层面调用此函数
    extern "C" {
        fn abs(input: i32) -> i32;
    }
    {
        unsafe {
            println!("Absolute value of -3 according to C: {}", abs(-3));
        }
    }
    // 从其他语言调用Rust函数
    // 也可以使用 extern 来创建一个允许其他语言调用Rust函数的接口.
    // 不同于extern块,就在fn关键字之前曾加extern 关键字并制定所用到的ABI.
    // 还需增加 #[no_mangle] 注解来告诉Rust 编译器不要 mangle次函数的名称
    // Mangling发生于当编译器将我们制定的函数名改为不同名称时,这将增加用于其他编译过程的额外信息,不过会使其名称更难以阅读
    // 每个编程语言的编译器都会以稍微不同的方式mangle函数名,所以为了使Rust函数能在其他语言中指定,必须禁用Rust 的name mangling.

    // 下面代码,一旦编译为动态库并从c语言中链接,call_from_c 函数就能够在C代码中访问

    // #[no_mangele]
    // pub extern "C" fn call_from_c() {
    //     println!("Just called a Rust function from C!");
    // }

    // extern 的使用无需 unsafe

    // 创建或访问静态变量
    // Rust 支持全局变量, 不过对于Rust 所有权规则来说是有问题的.
    // 如果两个线程同时访问相同的可变全局变量,则可能造成数据竞争.
    // 全局变量在Rust中称为 静态(static)变量.
    {
        static HELLO_WORLD: &str = "Hello, world!";

        println!("name is: {}", HELLO_WORLD);
    }
    // static 变量类似于常量, 命名采用 SCREAMING_SNAKE_CASE.
    // 静态变量只能存储拥有'static 声明变量的引用,这意味着 Rust 的编译器可以自己计算出生命周期而无需显示标注
    // 访问不可变静态变量是安全的.
    // 常量与不可变静态变量看起来很类似,区别是静态变量的值有一个固定的内存地址.
    // 使用这个值总是会访问相同的地址.
    // 另一方面,常量则允许任何在被用到的时候复制其数据.
    // 常量和静态变量的另个一个区别只在于静态变量可以是可变的.访问和修改可变静态变量都是不安全额
    {
        static mut COUNTER: u32 = 0;
        fn add_to_count(inc: u32) {
            unsafe {
                COUNTER += inc;
            }
        }

        add_to_count(3);
        unsafe {
            println!("COUNTER: {}", COUNTER);
        }
    }

    // 实现不安全 trait {
    // unsafe 的拎一个操作用例是 实现不安全 trait. 当至少有一个方法中包含编译器不能验证的变量时 trait 是不安全的.
    // 可以在trait 之前增加unsafe 关键字将 trait 声明为 unsafe,同时trait的实现也必须标记为unsafe
    {
        unsafe trait Foo {
            // methods go here
        }
        unsafe impl Foo for i32 {
            // method implementations go here
        }
        // 通过unsafe impl,我们承诺保证编译器不能验证的不变量
        // Sync 和 Send 标记 trait,编译器会自动为完全由 Send 和 Sync类型组成的类型自动实现他们.
        // 如果实现了 一个包含一些不是 Send 或 Sync 的类型,比如裸指针,并希望将此类型标记为 Send 或 Sync,则必须使用unsafe
        // Rust 不能验证我们的类型保证可以安全的跨线程发送或在多线程间访问
    }

    // 访问联合体的字段
    // 仅适用于 unsafe 的最后一个操作是访问联合体中的字段, union 和 struct类似,但是一个实例中只能使用一个声明的字段
    // 联合体主要用于和 c代码的联合体交互
    // 联合体 https://doc.rust-lang.org/reference/items/unions.html

    // 何时使用不安全代码
    // 使用 unsafe 来进行这五个操作之一时没有问题的,甚至是不需要深思熟虑的,不过是的unsafe 代码正确也实属不易
    // 因为编译器不能帮助保证内存安全.当有理由使用unsafe代码时,是可以这么做的,通过使用显示unsafe 标注可以更容易地在错误发生时追踪问题的源头.
}

pub fn ch19_03_advanced_traits() {
    // 关联类型在trait顶一个中指定占位符类型
    // 关联类型(associated types) 是一个将类型占位符与trait相关联的方式,这样trait的方法签名就可以使用这些占位符类型
    // 如此可以定义一个使用多种类型的trait
    // 一个带有关联类型的trait 的例子是标准库提供的 Iterator trait.它有一个叫做Item 的关联类型来替代遍历的值的类型
    {
        pub trait Iterator {
            // Item 是一个占位类型,同时 next 方法定义表明它 返回 Option<Self::Item> 类型的值
            // 这个 trait实现者会指定 Item 的具体类型,然而不管实现者指定何种类型,
            // next方法都会返回一个包含了次具体类型的Option
            type Item;
            fn next(&mut self) -> Option<Self::Item>;
        }
        struct Counter;
        impl Iterator for Counter {
            type Item = i32;
            fn next(&mut self) -> Option<Self::Item> {
                Some(1)
            }
        }
        // error 只有一个实现
        // impl Iterator for Counter {
        //     type Item = u32;
        //     fn next(&mut self) -> Option<Self::Item> {
        //         Some(1u32)
        //     }
        // }
    }

    // 与 泛型的不同之处
    {
        pub trait Iterator<T> {
            fn next(&mut self) -> Option<T>;
        }
        struct Counter;
        // where 只有在声明的时候能用
        impl Iterator<i32> for Counter {
            fn next(&mut self) -> Option<i32> {
                Some(1)
            }
        }
        // 不同类型可以有多个不同实现
        impl Iterator<u32> for Counter {
            fn next(&mut self) -> Option<u32> {
                Some(1)
            }
        }

        // 不得不在声明的标识泛型的地方 标注类型.这是因为我们可以实现为 Iterator<String> for Counter,
        // 或任何其他类型,这样就可以有多个Counter的Iterator的实现.
        // 换句话说,当trait 有泛型参数时,可以多次实现这个trait,每次需改变泛型参数的具体类型.
        // 接着当使用 Counter 的next 方法时,必须提供类型注解来表明希望 Iterator的哪个实现

        // 通过关联类型,则无需标注类型因为不能多次实现这个trait.
        // 我们只能选择一次 Item会是什么类型,调用时也不必指定 类型
    }

    // 默认泛型类型参数和运算符重载
    // 使用泛型类型参数时,可以为泛型指定一个默认的具体类型.如果默认类型就足够的话,这消除了为具体类型实现trait的需要
    // <PlaceholderType=ConcreteType >

    // 这种情况的一个非常好用例子是用运算符重载. 运算符重载(Operation overloading)是在指定情况下 自定义个运算符(比如 +)行为的操作
    // Rust并不允许创建自定义运算符或重载任意运算符.不过std::ops 中所列出的运算符和相应的trait可以通过实现运算符相关trait来重载.
    {
        use std::ops::Add;
        #[derive(Debug, PartialEq)]
        struct Point {
            x: i32,
            y: i32,
        }

        // 实现 Add trait 重载 Point 实例的 + 运算符
        impl Add for Point {
            type Output = Point;

            fn add(self, other: Point) -> Point {
                Point {
                    x: self.x + other.x,
                    y: self.y + other.y,
                }
            }
        }
        // add 方法将两个 Point 实例的 x值和 y 值分别相加来创建一个新的 Point.
        // Add trait 有一个叫做 Output的关联类型, 它用来决定add 方法的返回值类型
        {
            // RHS=Self：这个语法叫做 默认类型参数（default type parameters）
            trait Add<RHS = Self> {
                type Output;
                fn add(self, rhs: RHS) -> Self::Output;
            }
            // RHS 是一个泛型参数("right hand side"的缩写),它用于顶一个 add 方法中的 rhs 参数.
            // 如果实现 Add trait 时 不指定RHS 的具体类型, RHS的类型就是默认的 self类型,也就是其上实现Add 的类型
            // 当为 Point 实现Add 时,使用了默认的RHS,因为我们希望两个Point实例相加.
        }

        // 不使用默认类型的例子
        {
            use std::ops::Add;
            #[derive(Debug)]
            struct Millimeters(u32);
            #[derive(Debug)]
            struct Meters(u32);
            impl Add<Meters> for Millimeters {
                type Output = Millimeters;
                fn add(self, other: Meters) -> Millimeters {
                    Millimeters(self.0 + (other.0 * 1000))
                }
            }
            let a = Millimeters(1);
            let b = Meters(1);
            println!("{:?}", a + b);

            // 为了使 Millimeters 和Meters 能够相加, 我们制定 Add<Meters> 来设定RHS 类型参数的值 而不是使用默认的 Self.
            // 默认参数主要用于如下两个方面:
            //  1. 拓展类型而不破坏代码
            //  2. 在大部分用户都不需要的特定情况进行自定义.

            // 标准库的 Add trait就是一个 第二个目的的例子: 大部分的时候你会将两个相似的类型相加,
            // 不过它提供了自定义 额外行为的能力.
            // 在 Add trait 顶一个中使用默认类型参数意味着大部分时候无需指定额外的参数.
            // 换句话说, 一小部分实现的样板代码是不必要的,这样使 trait 就更容易了/
        }
    }

    // 完全限定语法与消歧义: 调用相同名称的方法
    // Rust既不能避免一个 trait 与 另一个 trait拥有相同名称的方法,也不能阻止同一类型同时实现这两个trait
    // 升值直接在类型上实现开始已经有的同名方法也是可能的!
    // 不过当调用这些同名方法时,需要告诉Rust我们希望使用哪一个
    {
        trait Pilot {
            fn fly(&self);
        }
        trait Wizard {
            fn fly(&self);
        }
        struct Human;

        impl Pilot for Human {
            fn fly(&self) {
                println!("this is your captain speaking");
            }
        }
        impl Wizard for Human {
            fn fly(&self) {
                println!("up!");
            }
        }
        impl Human {
            fn fly(&self) {
                println!("*waving arms furiously*")
            }
        }

        let person = Human;
        // 调用的是 impl Human 的实现
        person.fly(); // *waving arms furiously*

        // 为了能够调用 Pilot trait 或 Wizard trait的fly方法,我们需要使用更明显的语法以便能指定我们指的是哪个fly方法
        Pilot::fly(&person);
        Wizard::fly(&person);
        person.fly();

        // 因为fly方法获取一个 self 参数.如果有两个类型都实现了同一 trait,Rust 可以根据self的类型计算出应该使用那个trait.
        // 然而关联函数式 trait 的一部分, 但没有self参数.
        // 当当以作用域的两个类型实现了 同一个trait,Rust 就不饿能计算出我们期望的是哪个一个类型,除非使用 完全限定语法(fully qualified syntax)
        trait Animal {
            fn baby_name() -> String;
        }
        struct Dog;
        impl Dog {
            fn baby_name() -> String {
                String::from("Spot")
            }
        }
        impl Animal for Dog {
            fn baby_name() -> String {
                String::from("puppy")
            }
        }
        //                                    <>添加类型注解
        println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
        // 完全限定语法定义为
        // <Type as Trait>::function(receiver_if_method,next_arg,...);
        // 对于关联函数,其没有一个 receiver, 顾只会有其他参数的列表.
        // 可以选择在任何函数或方法调用处使用完全限定语法,然而,允许省略Rust能够从程序中的其他信息中计算出的部分
        // 只有当存在多个同名实现Rust 需要帮助以便知道我们希望调用哪个个实现时,才需要这个较为冗长的语法
    }

    // 父 trait 用于在另一个 trait 中使用 某 trait 的功能
    // 有时 我们可能需要某个 trait 使用另一个trait 的功能.在这种情况下, 需要能够依赖相关的 trait 也被实现.
    // 这个所需的 trait 是我们实现的 trait 的父(超)trait(super trait).
    // 例如我们希望创建一个带有outline_print 方法的 trait OutlinePrint, 它会打印出带有星号框的值.
    // 也就是说,如果Point实现了 Display 并返回(x,y),调用以,调用以1 作为 x 和 3 作为 y的Point 实例的outline_print 会显示 (1, 3)
    // 在outline_print 的实现中,因为希望能够使用Display trait 的功能,
    // 则需要说明OutlinePrint 只能用于同时也实现了 Display  并提供了 OutlinePrint 需要的功能的类型.
    // 可以通过在 trait定义中 指定 OutlinePrint:Display 来做到这一点.
    // 这类似于为 trait增加 trait bound1.
    {
        use std::fmt;
        trait OutlinePrint: fmt::Display {
            fn outline_print(&self) {
                let output = self.to_string();
                let len = output.len();
                println!("{}", "*".repeat(len + 4));
                println!("*{}*", " ".repeat(len + 2));
                println!("* {} *", output);
                println!("*{}*", " ".repeat(len + 2));
                println!("{}", "*".repeat(len + 4));
            }
        }

        struct Point {
            x: i32,
            y: i32,
        }

        // 必须先实现
        impl fmt::Display for Point {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "({}, {})", self.x, self.y)
            }
        }

        impl OutlinePrint for Point {}

        let p = Point { x: 1, y: 2 };

        p.outline_print();
    }

    // newtype 模式用以在外部类型上实现外部trait
    // 在第十章的 为类型实现trait 的部分,我们提到了 孤儿原则(orphan rule),
    // 它说明只要trait 或类型对于当前crate是本地的话,就可以在此类型上实现该trait
    // 一个绕开这个显示的方法是 使用newtype 模式(newtype pattern),它涉及到在一个元组结构体中创建一个新类型
    // 这个元组结构体带有一个字段作为希望实现trait的类型的简单封装.
    // 接着这个封装类型对于crate是本地的,这样就可以在这个封装上实现trait.
    // 使用这个模式没有运行时性能惩罚,这个封装类型在编译时就被省略了.

    // 如果想要在 Vec<T>上实现Display,而孤儿原则阻止我们直接这么做,因为Display trait 和 Vec<T>都定义于我们的crate之外
    // 可以创建一个包含Vec<T> 实例的Wrapper结构体,接着可以在Wrapper 上实现Display并使用Vec<T>的值
    {
        use std::fmt;
        struct Wrapper(Vec<String>);
        // Display的实现使用 self.0来访问内部的Vec<T>,因为Wrapper是元祖结构体而Vec<T>是结构体总位于索引0的项.
        // 接着就可以使用Wrapper 中 Display的功能了
        impl fmt::Display for Wrapper {
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                write!(f, "[{}]", self.0.join(","))
            }
        }
        let w = Wrapper(vec![String::from("hello"), String::from("world")]);
        print!("w= {}", w);

        // 此方法的缺点是,因为 Wrapper是一个新类型,它没有定义于其值之上的方法;必须直接在Wrapper上实现Vec<T>的所有方法,
        // 这样可以代理到self.0 上,这就允许我们完全像Vec<T>那样对待Wrapper.
        // 如果希望新类型拥有其内部类型的每一个方法.为封装类型实现Deref trait(第十五章 “通过 Deref trait 将智能指针当作常规引用处理” 部分讨论过)并返回内部类型一一种解决办法
        //                                                使用*来返回内部类型
        // 如果不希望封装类型拥有所有内部类型的方法--比如为了显示封装类型的行为-- 则必须自行实现所需的方法.
    }
}

// 高级类型
pub fn ch19_04_advanced_types() {
    // newtype 模式可以用于一些其他我们还未讨论的功能,包括静态的确保其值不被混淆,和用来表示一个值的单元.
    // 另一个newtype 模式的引用在于抽象掉一些类型的实现细节:例如,封装类型可以暴露出与直接使用其内部私有类型时所不同的公有API,以便限制其功能.
    // newtype 也可以隐藏其内部的泛型类型.例如,可以提供一个封装了 Hash<i32,String> 的people类型.用来储存人名一级相应的ID.
    // 使用People的代码只需与提供给公有API交互即可,比如向People添加名字字符串方法,这样这些代码就无需知道内部们将一个i32 ID赋予了这个名字了
    // newtype 模式是一种实现第十七章 “封装隐藏了实现细节” 部分所讨论的隐藏实现细节的封装的轻量级方法。

    // 类型别名用来创建类型同义词
    // 连同newtype模式,Rust还提供了声明 类型别名(type alias)的能力.使用type 关键字来基于现有类型另一个名字.
    {
        type Kilometers = i32; // 作为i32 的别名
        let x: i32 = 5;
        let y: Kilometers = 5;
        println!("x + y = {}", x + y); // 可以参加运算
    }
    // 类型别名的组要用途是减少重复
    {
        type Thunk = Box<dyn Fn() + Send + 'static>;
        let f: Thunk = Box::new(|| println!("hi"));
        fn takes_long_type(f: Thunk) {
            // --snip--
        }
        fn returns_long_type() -> Thunk {
            return Box::new(|| println!("hi"));
        }
    }
    // 类型别名也经常与Result<T,E>结合使用来减少重复.
    // 考虑一个标准库中的 std::Error模块.
    // I/O操作通常会 返回一个Result<T,E> ,因为这些操作可能会失败.
    // 标准库std:io:Error 结构体代表了所有可能的I/O错误.
    // std::io中大部分函数会返回Result<T,E> 其中E是std::io:Error
    {
        use std::fmt;
        use std::io::Error;

        // 这里出现了很多的 Result<..., Error>。为此，std::io 有这个类型别名声明：
        // trait Write {
        //     fn write(&mut self, but: &[u8]) -> Result<usize, Error>;
        //     fn flush(&mut self) -> Result<(), Error>;
        //     fn write_all(&mut self, buf: &[u8]) -> Result<(), Error>;
        //     fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(), Error>
        // }

        type Result<T> = std::result::Result<T, std::io::Error>;
        // 因为这位于 std::io中, 可用的完全限定的别名是 std::io::Result<T> -- 也就是说, Result<T> 中 E 放入了 std::io::Error
        trait Write {
            fn write(&mut self, buf: &[u8]) -> Result<usize>;
            fn flush(&mut self) -> Result<()>;
            fn write_all(&mut self, buf: &[u8]) -> Result<()>;
            fn write_fmt(&mut self, fmt: Arguments) -> Result<()>;
        }
        // 类型别名在两个方面有帮助 : 易于编写 并 在整个std:io 中提供了一直的接口.
        // 因为他是一个别名,它只是另一个 Result<T,E> ,这意味着可以在其上使用 Result<T,E>的任何方法.以及像 ?(报错简写) 这样的特殊语法
    }

    // 从不返回的never type
    {
        // Rust 有一个叫做 ! 的特殊类型. 在类型理论中,他被称为 empty type.因为他没有值.
        // 我们更倾向于 称之为 never type.
        // 作用为: 在函数从不返回的时候充当返回值
        {
            // fn bar() -> ! {}
            // 函数bar 从不犯规,而从不返回返回的函数, 称为发散函数.
            // 不能创建 ! 类型的值,所以bar 也不可能有返回值
        }

        {
            let secret_number = 1;

            loop {
                println!("Please input your guess.");
                let mut guess = String::new();
                io::stdin()
                    //             入参 借出的可变字符串
                    .read_line(&mut guess)
                    .expect("Failed to read line");

                println!("You guessed: {}", guess);

                let guess: u32 = match guess.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue, // 类型 为 !
                };

                match guess.cmp(&secret_number) {
                    Ordering::Less => println!("Too small!"),
                    Ordering::Greater => println!("To big!"),
                    Ordering::Equal => {
                        println!("You win!");
                        break;
                    }
                }
            }
        }

        // 如循环中的 match 的返回值是 continue返回是 !, Rust 会自动判断类型是 不是! 的那个
        // 描述 ! 的行为正式方式 是 never type 可以强转为任何类型.
        // 允许 match 以 continue 结束,是因为continue 并不真正返回一个值;
        // 相反它把控制权 交回上层循环,所以在Err 的情况事实上未对 guess 赋值
        // never type 的另一个用途是 panic!.
        {
            // impl<T> Option<T> {
            //     pub fn unwrap(self) -> T {
            //         match self {
            //             Some(val) => val,
            //             None => panic!("called `Option::unwrap()` on a `None` value"), // panic! 是 ! 类型
            //         }
            //     }
            // }
        }

        // loop 表达式也是 ! 类型
        {
            // 无限循环的 loop 为 ! 类型
            // loop {
            //     print!("and ever ");
            // }
        }
    }

    // 动态大小类型 和 Sized trait
    // 因为 Rust 需要知道例如应该为特定类型的值分配多少空间这样的信息其类型系统的一个特定的角落令人迷惑:
    // 这就是动态大小类型(dynamically sized types)的概念. 这有时被称为 "DST 或 "unsized types",
    // 这些类型允许我们处理只有在运行时才知道大小的类型.

    // 动态大小类型的细节: str,不是&str, 而是str本身. str是一个DST ;直到运行时我们都不知道字符串有多伤.
    // 因为直到运行时都不知道其大小,也就意味着不能创建 str 的变量,也不能获取 str 类型的参数.
    {
        // let s1: str = "Hello there!";
        // let s2: str = "How's it going?";
    }
    // rust 需要知道应该为特定类型分配多少内存,同时所有同一个类型的值必须使用相同数量的内存.
    // 如果允许写这样的代码,也就意味着这两个str 需要占用完全相同大小的内存
    // 不过他们有着不同的长度.这也就是为什么不能穿件一个存档动态大小类型的变量的原因

    // 那么该怎么办呢? 已经知道了这种问题的答案: s1 和 s2的类型是 &str. slice 数据储存了开始位置和 slice 的长度.
    // 所以虽然&T 是一个储存了 T 所在内存位置的单个值 ,&T 则是两个值:str 的地址和其长度
    // 这样&str 就有了一个在编译时可以知道的大小: 他是 usize 长度的两倍.
    // 也就是说,我们总是知道&str 的大小,而无论其引用的字符串多长
    // 这里是 Rust中动态大小类型的常规用法: 他们有以下额外的元信息来储存动态信息的大小.
    // 这引出了动态大小类型的黄金规则: 必须将动态代销类型的值置于某种指正之后

    // 可以将 str 与 所有类型的指针相结合: 比如Box<str> 或 Rc<str>.
    // 另一个动态大小类型: trait/ 每个trait 都是一个可以通过 trait 名称来引用的动态大小类型.
    // 为了将 trait用于 trait 对象,必须将它们放入指针之后,比如 &dyn trait 或 Box<dyn trait> Rc<dyn trait>

    // 为了处理 DST, Rust 有一个特定的trait来决定 一个类型的大小是否在编译时可知: 这就是 Sized trait.
    // 这个trait 自动为编译器 在编译时就知道大小类型实现.
    // Rust 隐式的为每个泛型函数 增加了 Sized bound
    {
        fn generic<T>(t: T) {}
    }
    // 实际上被当做如下处理
    {
        fn generic<T: Sized>(t: T) {}
    }
    // 泛型函数只能用于在编译时已知大小的类型.然而可以使用如下如下特殊语法来放宽这个限制
    {
        fn generic<T: ?Sized>(t: &T) {}
    }
    {
        struct Foo<T>(T);
        struct Bar<T: ?Sized>(T);

        // struct FooUse(Foo<[i32]>); // error: Sized is not implemented for [i32]
        struct BarUse(Bar<[i32]>); // OK
        struct BarUse2(Bar<str>); // OK
        struct BarUse3<'a>(Bar<&'a str>); // OK
    }
    {
        trait Foo {}
        trait Bar: Sized {}
        //    ---  ^^^^^ ...because it requires `Self: Sized`

        struct Impl;
        impl Foo for Impl {}
        impl Bar for Impl {}

        let x: &dyn Foo = &Impl;
        // let y: &dyn Bar = &Impl; // error: the trait `Bar` cannot be made into an object
    }
    // ?Sized trait bound 与 Sized 相对;也就是说, 它可以多做"T 可能是也可能不是 Sized的".这个语法只能用于Sized,而不能使用其他 trait
    // 另外注意我们将t参数的类型从T 变为 &T: 因为其类型可能不是Sized 的,所以需要将其置于某种指针之后.这个例子中选择了引用
}

// 高级函数与闭包
pub fn ch19_05_advanced_functions_and_closures() {
    // 函数指针
    // 可以向函数传递常规函数!这在我们希望传递已经定义函数而不是重新定义闭包作为参数时很有用.通过函数指针允许我们使用函数作为另一个函数的参数
    // 函数的类型是 fn () 不同于  闭包 trait 中的 Fn
    {
        fn add_one(x: i32) -> i32 {
            x + 1
        }
        let add_two = |x: i32| x + 1;

        fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
            f(arg) + f(arg)
        }

        let answer = do_twice(add_one, 5);
        let answer = do_twice(add_two, 5); // 闭包函数也可以
        println!("The answer is : {}", answer);
        // 不同于闭包,fn 是一个类型而不是一个 trait ,所以直接指定fn 作为参数,而不是声明一个带有Fn 作为 trait bound 的泛型参数
    }
    // 函数指针实现了 三个闭包trait (Fn,FnMut 和 FnOnce),所以总是可以在调用期望闭包的函数时传递函数指针作为参数.
    // 倾向于编写使用泛型和闭包的函数,这样它就能接受函数或闭包作为参数

    // 一个只期望接受fn而不接受闭包的情况的例子是与不存在闭包的外部代码交互时: C语言的函数可以接受函数作为参数,但c语言没有闭包
    {
        let list_of_number = vec![1, 2, 3];
        let list_of_strings: Vec<String> = list_of_number.iter().map(|i| i.to_string()).collect();
    }
    // 或者可以将函数作为 map参数来替代闭包
    {
        // use std::string::ToString;
        let list_of_number = vec![1, 2, 3];
        let list_of_string: Vec<String> = list_of_number.iter().map(ToString::to_string).collect();
        // 这里必须使用 高级trait部分讲到的完全限定语法,因为存在多个叫做 to_sting 的函数;
        // 这里使用了ToString  trait 的to_string函数,标准库所有实现了 Display 的类型实现了这个trait
    }

    // 另一个实用的模式暴露了元结构体枚举成员的实现细节.这些使用()作为初始化语法,这看起来像函数调用
    // 同时它们确实被实现为返回参数构造实例的函数
    // 它们也被称为实现了 闭包trait的函数指针
    {
        #[derive(Debug)]
        enum Status {
            Value(u32),
            Stop,
        }

        let list_of_status1: Vec<Status> = (0u32..20).map(Status::Value).collect();
        let list_of_status2: Vec<Status> = (0u32..20).map(|v| Status::Value(v)).collect();
        // println!("{:?}", list_of_status1);
        // println!("{:?}", list_of_status2);
    }

    // 返回闭包
    // 闭包表现为 trait,这意味着不能直接返回闭包.
    // 对于大部分需要返回trait 的情况,可以使用实现了期望返回的trait的具体类型来替代函数的返回值
    // 但是这不能用于闭包,因为他们没有一个可返回的具体类型
    // 例如不允许使用函数指针fn作为返回值的类型.
    {
        // error
        // 错误又一次指向了 Sized trait! Rust 并不知道需要多少空间来储存闭包.

        // fn return_closure() -> Fn(i32) -> i32 {
        //     |x| x + 1`
        // }

        fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
            Box::new(|x| x + 1)
        }
    }
}
