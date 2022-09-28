// 指针 （pointer）是一个包含内存地址的变量的通用概念。
// 这个地址引用，或 “指向”（points at）一些其他数据。
// Rust 中最常见的指针是第四章介绍的 引用（reference）。
// 引用以 & 符号为标志并借用了他们所指向的值。
// 除了引用数据没有任何其他特殊功能。

// 在 Rust 中，普通引用和智能指针的一个额外的区别是引用是一类只借用数据的指针；
// 相反，在大部分情况下，智能指针 拥有 他们指向的数据。

// 智能指针通常使用结构体实现。
// 智能指针区别于常规结构体的显著特性在于其实现了 Deref 和 Drop trait。
// Deref trait 允许智能指针结构体实例表现的像引用一样，
// 这样就可以编写既用于引用、又用于智能指针的代码。
// Drop trait 允许我们自定义当智能指针离开作用域时运行的代码。
// 本章会讨论这些 trait 以及为什么对于智能指针来说他们很重要。

// 智能指针通常使用结构体实现。
// 智能指针区别于常规结构体的显著特性在于其实现了 Deref 和 Drop trait。
// Deref trait 允许智能指针结构体实例表现的像引用一样，
// 这样就可以编写既用于引用、又用于智能指针的代码。
// Drop trait 允许我们自定义当智能指针离开作用域时运行的代码。
// 本章会讨论这些 trait 以及为什么对于智能指针来说他们很重要。

/*
Box<T>, Rc<T>, RefCell<T>比较：

Rc<T>，允许多重拥有，不可变借用，编译时检查

Box<T>，单一拥有者，可变或不可变借用，编译时检查(Deref, DerefMut)

RefCell<T>, 单一拥有者，可变或不可变借用，运行时检查。可变不可变是对外的，都可以在内部改变。其实是把不安全的操作包装在安全的接口中，适用于：我比编译器明白，我知道我在干什么。
*/

pub fn ch15_01_box() {
    // box 允许你将一个值放在堆上而不是栈上。留在栈上的则是指向堆数据的指针。
    // 除了数据被储存在堆上而不是栈上之外，box 没有性能损失。不过也没有很多额外的功能。它们多用于如下场景：
    //
    // 当有一个在编译时未知大小的类型，而又想要在需要确切大小的上下文中使用这个类型值的时候
    // 当有大量数据并希望在确保数据不被拷贝的情况下转移所有权的时候
    // 当希望拥有一个值并只关心它的类型是否实现了特定 trait 而不是其具体类型的时候

    // 使用 Box<T> 在堆上储存数据
    {
        let b = Box::new(5); // 这里定义了变量 b，其值是一个指向被分配在堆上的值 5 的 Box。

        println!("b = {}", b);
        // 在这个例子中，我们可以像数据是储存在栈上的那样访问 box 中的数据。
        // 正如任何拥有数据所有权的值那样，当像 b 这样的 box 在 main 的末尾离开作用域时，它将被释放。
        // 这个释放过程作用于 box 本身（位于栈上）和它所指向的数据（位于堆上）。
    }

    // Box 允许创建递归类型
    // Rust 需要在编译时知道类型占用多少空间。一种无法在编译时知道大小的类型是 递归类型（recursive type），
    // 其值的一部分可以是相同类型的另一个值。
    // 这种值的嵌套理论上可以无限的进行下去，所以 Rust 不知道递归类型需要多少空间。
    // 不过 box 有一个已知的大小，所以通过在循环类型定义中插入 box，就可以创建递归类型了。

    // cons list 的更多内容
    // cons list 是一个来源于 Lisp 编程语言及其方言的数据结构。在 Lisp 中，
    // cons 函数（“construct function" 的缩写）利用两个参数来构造一个新的列表，
    //
    // cons 函数的概念涉及到更常见的函数式编程术语；“将 x 与 y 连接” 通常意味着构建一个新的容器而将 x 的元素放在新容器的开头，其后则是容器 y 的元素。
    //
    // cons list 的每一项都包含两个元素：当前项的值和下一项。其最后一项值包含一个叫做 Nil 的值且没有下一项。
    // cons list 通过递归调用 cons 函数产生。
    // 代表递归的终止条件（base case）的规范名称是 Nil，它宣布列表的终止。注意这不同于第六章中的 “null” 或 “nil” 的概念，他们代表无效或缺失的值。
    // 注意虽然函数式编程语言经常使用 cons list,但是它并不是一个 Rust的常见类型.大部分在Rust中需要列表的时候,Vec<T> 是一个更好的选择.
    // 其他更为复杂的递归类型.大部分在Rust很多场景中很有用, 不过通过以cons list 作为你开始,我我们可以探索如何使用 box 毫不费力的定义一个递归数据类型。们

    // 使用 Box<T> 给递归类型一个已知的大小
    // 因为 Box<T> 是一个指针，我们总是知道它需要多少空间：指针的大小并不会根据其指向的数据量而改变。
    // 这意味着可以将 Box 放入 Cons 成员中而不是直接存放另一个 List 值。Box 会指向另一个位于堆上的 List 值，
    // 而不是存放在 Cons 成员中。从概念上讲，我们仍然有一个通过在其中 “存放” 其他列表创建的列表，
    // 不过现在实现这个概念的方式更像是一个项挨着另一项，而不是一项包含另一项。
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }

    let list = List::Cons(
        1,
        Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
    );
    // box 只提供了间接存储和堆分配；他们并没有任何其他特殊的功能，
    // Box<T> 类型是一个智能指针，因为它实现了 Deref trait，它允许 Box<T> 值被当作引用对待。
    // 当 Box<T> 值离开作用域时，由于 Box<T> 类型 Drop trait 的实现，box 所指向的堆数据也会被清除。

    if let List::Cons(v, _) = list {
        println!("List::Cons:{v}");
    }
}

pub fn ch15_02_deref() {
    // 实现 Deref trait 允许我们重载 解引用运算符（dereference operator）*（与乘法运算符或通配符相区别）。
    // 通过这种方式实现 Deref trait 的智能指针可以被当作常规引用来对待，可以编写操作引用的代码并用于智能指针。

    // 通过解引用运算符追踪指针的值
    {
        let x = 5;
        let y = &x;

        assert_eq!(5, x);
        assert_eq!(5, *y); // 解引用了 y，就可以访问 y 所指向的整型值并可以与 5 做比较。
    }

    // 像引用一样使用 Box<T>
    {
        let x = 5;
        let y = Box::new(x);

        assert_eq!(5, x);
        assert_eq!(5, *y); // 解引用运算符以 y 为引用时相同的方式追踪 box 的指针
    }
    {
        let x = 5;
        let mut y = Box::new(x);
        *y = 1;
    }

    // 自定义智能指针

    // 实现一个 Box
    #[derive(Debug)]
    struct MyBox<T>(T);
    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }

    // 通过实现 Deref trait 将某类型像引用一样处理
    use std::ops::Deref;
    impl<T> Deref for MyBox<T> {
        type Target = T;
        // 语法定义了用于此 trait 的关联类型。关联类型是一个稍有不同的定义泛型参数的方式，现在还无需过多的担心它；
        // 第十九章会详细介绍。
        // deref 方法向编译器提供了获取任何实现了 Deref trait 的类型的值，
        // 并且调用这个类型的 deref 方法来获取一个它知道如何解引用的 & 引用的能力。
        fn deref(&self) -> &Self::Target {
            &self.0 // deref 方法体中写入了 &self.0，这样 deref 返回了我希望通过 * 运算符访问的值的引用
        }
    }

    {
        let x = 5;
        let y = MyBox::new(x);
        let mut z = MyBox::new(x);
        // *z = 1; //  trait `DerefMut` is required to modify through a dereference
        println!("z:{z:?}");

        assert_eq!(5, x);
        assert_eq!(5, *y);
        assert_eq!(5, *y);
        // Rust 事实上在底层运行了如下代码：
        // *(y.deref())

        // deref 方法返回值的引用，以及 *(y.deref()) 括号外边的普通解引用仍为必须的原因在于所有权。
        // 如果 deref 方法直接返回值而不是值的引用，其值（的所有权）将被移出 self。
        // 在这里以及大部分使用解引用运算符的情况下我们并不希望获取 MyBox<T> 内部值的所有权。

        // 注意，每次当我们在代码中使用 * 时， * 运算符都被替换成了先调用 deref 方法再接着使用 * 解引用的操作，
        // 且只会发生一次，不会对 * 操作符无限递归替换，解引用出上面 i32 类型的值就停止了
    }

    // 函数和方法的隐式Deref 强制转换
    // Deref 强制转换（deref coercions）是 Rust 在函数或方法传参上的一种便利。
    // 其将实现了 Deref 的类型的引用转换为原始类型通过 Deref 所能够转换的类型的引用。
    // 当这种特定类型的引用作为实参传递给和形参类型不同的函数或方法时，Deref 强制转换将自动发生。
    // 这时会有一系列的 deref 方法被调用，把我们提供的类型转换成了参数所需的类型。
    {
        fn hello(name: &str) {
            println!("Hello, {}!", name);
        }

        let m = MyBox::new(String::from("Rust"));
        // MyBox<T> 上实现了 Deref trait，Rust 可以通过 deref 调用将 &MyBox<String> 变为 &String
        // 标准库中提供了 String 上的 Deref 实现，其会返回字符串 slice，这可以在 Deref 的 API 文档中看到。
        // Rust 再次调用 deref 将 &String 变为 &str，这就符合 hello 函数的定义了。
        hello(&m);
        // 如果MyBox 没有实现 Deref ,则需要 hello(&(*m)[..]);
        // (*m) 将 MyBox<String> 解引用为 String。接着 & 和 [..] 获取了整个 String 的字符串 slice 来匹配 hello 的签名。

        // 当所涉及到的类型定义了 Deref trait，Rust 会分析这些类型并使用任意多次 Deref::deref 调用以获得匹配参数的类型。
        // 这些解析都发生在编译时，所以利用 Deref 强制转换并没有运行时损耗！
        hello(&&&&&m);
        hello(&&&&&String::from("牛逼"));
    }

    // Deref 强制转换如何与可变性交互
    // 类似于如何使用 Deref trait 重载不可变引用的 * 运算符，Rust 提供了 DerefMut trait 用于重载可变引用的 * 运算符。
    // Rust 在发现类型和 trait 实现满足三种情况时会进行 Deref 强制转换：
    //  当 T: Deref<Target=U> 时从 &T 到 &U。
    //  当 T: DerefMut<Target=U> 时从 &mut T 到 &mut U。
    //  当 T: Deref<Target=U> 时从 &mut T 到 &U。
    // 第一种情况表明如果有一个 &T，而 T 实现了返回 U 类型的 Deref，则可以直接得到 &U。第二种情况表明对于可变引用也有着相同的行为。
    // 第三个情况有些微妙: Rust也会将可变引用转为不可变引用.反之是不可能的: 不可变引用永远不能转为可变引用.
    // 因为借用规则,如果一个可变引用,其必须是这些数据的唯一引用(否则程序无法编译).
    // 将一个可变引用,而借用规则无法保证这一点.因此,Rust无法假设将不可变引用转换为可变引用是可能的.
    {
        #[derive(Debug)]
        struct MyBox<T>(T);
        impl<T> MyBox<T> {
            fn new(x: T) -> MyBox<T> {
                MyBox(x)
            }
        }

        use std::ops::Deref;
        use std::ops::DerefMut;
        impl<T> Deref for MyBox<T> {
            type Target = T;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
        impl<T> DerefMut for MyBox<T> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }

        let a = 1;
        let mut b = MyBox::new(a);
        *b = 3;
        println!("b:{b:?}");
    }
}

pub fn ch15_03_drop() {
    // 对于智能指针模式来说第二个重要的 trait 是 Drop,其云溪我们在值要离开作用域时执行一些代码.
    // 可以为任何类型提供Drop trait 的实现,同时所指定的代码用于释放类似于文件或网络连接的资源.
    // 我们在 智能指针上下文讨论 Drop 是因为其功能几乎总是用于实现智能指针.
    // 例如Box<T> 用来释放box所指向的堆空间

    // 在Rust 中可以指定每当值离开作用域时被执行的代码.编译器会插入这些代码.

    // 指定在值离开作用域时 执行代码的方式是 实现Drop trait.
    // Drop trait 要求实现一个叫做 drop的方法它获取一个 self 的可变引用

    struct CustomSmartPointer {
        data: String,
    }
    impl Drop for CustomSmartPointer {
        fn drop(&mut self) {
            println!("Droping CustomSmartPointer with data `{}!`", self.data);
        }
    }
    {
        let c = CustomSmartPointer {
            data: String::from("my stuff"),
        };
        let d = CustomSmartPointer {
            data: String::from("other stuff"),
        };
        println!("CustomSmartPointers created");
        // 打印次序
        // CustomSmartPointers created.
        // Dropping CustomSmartPointer with data `other stuff`!
        // Dropping CustomSmartPointer with data `my stuff`!
    }

    // 通过 std::mem::drop 提早丢弃值
    // 不用主动调用 drop 方法, 整个 Drop trait 是自动处理的
    // 如果希望在作用域前结束前就强制释放变量的话,我们应该使用 std::mem::drop
    {
        let c = CustomSmartPointer {
            data: String::from("some data"),
        };
        println!("CustomSmarterPointer created.")
        // c.drop() // 直接就提示错误了
        // 错误信息表明不允许显式调用 drop。错误信息使用了术语 析构函数（destructor），这是一个清理实例的函数的通用编程概念。
        // 析构函数 对应创建实例的 构造函数。Rust 中的 drop 函数就是这么一个析构函数。
    }

    // std::mem::drop 函数不同于 Drop 中的 drop 方法.可以通过传递希望提早强制丢弃的值作为参数.
    // std::mem::drop 位于 prelude
    {
        let c = CustomSmartPointer {
            data: String::from("some data"),
        };
        println!("CustomSmartPointer created.");
        drop(c);
        println!("CustomSmartPointer dropped before the end of main.");
        // 打印次序
        // CustomSmartPointer created.
        // Dropping CustomSmartPointer with data `some data`!
        // CustomSmartPointer dropped before the end of main.
    }
}

use std::cell::RefCell;
use std::ops::DerefMut;
use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

pub fn ch15_04_rc() {
    // Rc<T> 引用计数智能指针
    // 有些情况某个值可能有多个所有者.
    // 例如在图数据结构中,多个边可能指向相同的几点,而这个节点从概念上讲为所有指向它的便所有
    // 节点直到没有任何边指向它之前都不应该被清理.

    // 为了启用所有权,Rust 有一个叫做Rc<T> 的类型.其名称为引用计数(reference counting)的缩写
    // 引用计数意味着记录一个值引用的数量来知晓这个值是否仍在被使用.
    // 如果某个值有零个引用,就代表没有任何有效引用并可以被清理.

    // Rc<T> 用于当我们希望在堆上分配一些内存供程序的多个部分读取,而且无法在编译时去顶程序的哪一部分会灾后结束使用它的时候
    // 如果确实知道哪部分是最后一个结束使用的话,就可以令其数据的所有者,正常的所有权规则就可以在编译时生效.
    // 注意 Rc<T> 只用于单线程场景;

    // 每个Cons 变量都包含一个值 和 一个指向 List 的Rc<T>.
    // 当常见 b 时,不同于获取 a 的所有权,这里会克隆 a 所包含的 Rc<List>,
    // 这将会引用计数从1 增加到2 并允许 a 和 b 共享 Rc<List> 中数据的所有权
    // 创建c 时也会克隆 a, 这会将引用计数 从2 增加为 3.每次调用 Rc::clone, Rc<List>中数据的引用计数都会增加,直到有0个引用之前数据不会被清理.

    use self::List::{Cons, Nil};
    {
        let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
        let b = Cons(3, Rc::clone(&a));
        let c = Cons(4, Rc::clone(&a));

        // 也可以调用 a.clone() 而不是 Rc::clone(&a), 不过在这里Rust 习惯是使用Rc::clone.
        // Rc::clone 的实现并不像大部分类型的clone 实现那样对所有数据进行深拷贝.
        // Rc::clone只会增加引用计数,这并不会话费很长时间.
        // 通过使用 Rc::clone 进行引用计数,可以明显区别深拷贝类的克隆和增加引用计数类的克隆
    }
    // 克隆Rc<T>会增加引用计数
    {
        let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
        println!("count after creating a = {}", Rc::strong_count(&a));
        let b = Cons(3, Rc::clone(&a));
        println!("count after creating b = {}", Rc::strong_count(&a));
        {
            let c = Cons(4, Rc::clone(&a));
            println!("count after creating c = {}", Rc::strong_count(&a));
        }
        println!("count after c goes out of scope = {}", Rc::strong_count(&a));
        // 打印次序
        // count after creating a = 1
        // count after creating b = 2
        // count after creating c = 3
        // count after c goes out of scope = 2
    }
    {
        // let a = RefCell::new(1).borrow(); // Ref<i32>
        // let b = &RefCell::new(1).borrow(); // &Ref<i32>
        // let c = (&RefCell::new(1)).borrow(); // Ref<i32>

        // let d = Rc::new(RefCell::new(1)).borrow(); // Ref<i32>
        // let d = Rc::new(&RefCell::new(1)).borrow(); // Ref<i32>
        // let d = &Rc::new(RefCell::new(1)).borrow(); // &Ref<i32>
        // let d = (&Rc::new(RefCell::new(1))).borrow(); // Ref<i32>

        fn e(f: Rc<RefCell<i32>>) -> i32 {
            *f.borrow()
        }
    }
}

// RefCell<T> 和 内部可变性模式

// 内部可变性(Interior mutability) 是 Rust 的一个设计模式,它允许你即使在有不可变引用时 也可也以改变数据,这通常是借用规则所不允许的.
// 为了改变数据,该模式在数据结构中使用unsafe 代码来模糊可变性和借用规则.
// 党可以确保在运行时遵守借用规则,即使编译器不能保证的情况下,可以选择使用那些运用内部可变性模式的了类型.
// 所涉及的unsafe 代码将被封禁安全的 API 中,而外部类型任然是不可变的

// 通过 RefCell<T> 在运行时检车借用规则
// 对于引用和Box<T> ,借用规则的不可变性作用域编译时.对于 RefCell<T>,这些不可变性作用域运行时.
// 对于引用,如果违反这些规则,就会得到一个编译错误. 而对于RefCell<T>,如果违反这些规则陈旭会 panic并退出.
// 在运行时检车借用规则的好处是允许出现特定内存安全的场景,而他们在编译时检查中是不允许的.
// RefCell<T> 正是用于当你确信代码遵守借用规则，而编译器不能理解和确定的时候。
// 类似于 Rc<T>，RefCell<T> 只能用于单线程场景。如果尝试在多线程上下文中使用RefCell<T>，会得到一个编译错误。

// Rc<T>允许多个数据有多个所有者: Box<T> 和RefCell<T> 有单一所有者
// Box<T> 允许在编译时执行不可变或可变借用检车;Rc<T> 仅允许在编译是执行不可变借用检查;RefCell<T> 允许在运行时执行不可变或可变借用检查
// 因为 RefCell<T> 允许在运行时执行可变借用检车,所以我们可以在即便RefCell<T> 自身是不可变的情况下修改内部的值

// 在不可变值内部改变值就是 内部可变性模式.

// 内部可变性的用例: mock 对象
// 测试替身(test double) 是一个通用编程概念, 它代表一个测试中替代某个类型的类型.mock对象是特定类型的测试替身,
// 它们记录测试过程中发生了什么以便断言操作是正确的.

// 下面是一个测试场景: 我们在编写一个记录某个值与最大值差距的库,并根据当前值与最大值的差距来发送消息.
// 该库只提供记录与最大值的差距,以及何种情况发送什么消息的功能.
// 使用此库的程序则期望提供实际发送消息的机制: 程序可以选择记录一条消息,发送email,发送短消息等.库本身无需知道这些细节;
// 只实现其提供的 Messenger trait 即可.
pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;
        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!")
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent Warning: You've used up over 90% of you quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!")
        }
    }
}

// 我们所需的 mock 对象是, 调用 send 并不实际发送email 或信息, 而是只记录消息被通知要发送了.
// 可以新基建一个 mock 对象实例,用其创建 LimitTracker, 调用 LimitTracker 的 set_value 方法,
// 然后检查mock 对象是否有我们期望的消息

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    pub trait Messenger {
        fn send(&self, msg: &str);
    }

    pub struct LimitTracker<'a, T: Messenger> {
        messenger: &'a T,
        value: usize,
        max: usize,
    }

    impl<'a, T> LimitTracker<'a, T>
    where
        T: Messenger,
    {
        pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
            LimitTracker {
                messenger,
                value: 0,
                max,
            }
        }

        pub fn set_value(&mut self, value: usize) {
            self.value = value;

            let percentage_of_max = self.value as f64 / self.max as f64;

            if percentage_of_max >= 1.0 {
                self.messenger.send("Error: You are over your quota!");
            } else if percentage_of_max >= 0.9 {
                self.messenger
                    .send("Urgent warning: You've used up over 90% of your quota!");
            } else if percentage_of_max >= 0.75 {
                self.messenger
                    .send("Warning: You've used up over 75% of your quota!");
            }
        }
    }

    struct MockMessenger {
        sent_message: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_message: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            // 改变 sent_message 的值
            self.sent_message.borrow_mut().push(String::from(message));
        }

        // fn send(&self, message: &str) {
        //     let mut one_borrow = self.sent_message.borrow_mut();
        //     let mut two_borrow = self.sent_message.borrow_mut(); // error 不能有多个可变引用
        //
        //     one_borrow.push(String::from(message));
        //     two_borrow.push(String::from(message));
        // }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_message.borrow().len(), 1);
    }
}
