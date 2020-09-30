#![allow(unused_variables)] // 不对 未使用的变量 warning

use std::collections::HashMap;
use std::fs::{self, File};
use std::io;
use std::io::ErrorKind;
use std::io::Read;
use std::ops::Add;

pub fn ch08_01_vectors() {
    // 新建 Vector
    {
        // vector 矢量 向量
        // 方法1 创建矢量 存放 i32 内容的矢量
        let v1: Vec<i32> = Vec::new();

        // 方法2 让Rust 自己推断 出存放的类型
        let v2 = vec![1, 2, 3];
    }

    // 更新 Vector
    {
        // 厉害啊 rust 直接靠之后 第一次 update 使用的类型 来声明Vec 的类型
        let mut v = Vec::new();
        v.push(5);
        v.push(6);
        v.push(7);
        v.push(8);
        // v.push("1"); // error
    }
    {
        // 正确推断成了 &str
        let mut v = Vec::new();
        v.push("string");
    }

    // 丢弃vector 时 也会丢弃其所有的元素
    {
        let v = vec![1, 2, 3];
    } // 此处已经清理所有的元素 和vector

    // 读取vector 元素
    {
        let v = vec![1, 2, 3, 4, 5];
        let third: &i32 = &v[2]; // 索引语法
        let third = &v[2]; // 自动推断
        println!("the third element is {}", third);

        // get 语法 返回的 是以索引作为参数来返回一个 Option<&T>
        match v.get(2) {
            Some(third) => println!("The third element is {}", third),
            None => println!("There is no third element"),
        }

        // 尝试读取 越界的元素
        // let does_not_exist = &v[100]; // 运行时 error
        let does_not_exist = v.get(100); // 不会报错 因为返回的是 Option类型
                                         // 当 get 方法被传递了一个数组外的索引时，它不会 panic 而是返回 None。
    }

    // 一旦程序获取了一个有效的引用，
    // 借用检查器将会执行所有权和借用规则（第四章讲到）来确保 vector 内容的这个引用和任何其他引用保持有效。
    // 只能有 一个 mut 或者 多个 &
    {
        // let mut v = vec![1, 2, 3, 4, 5];
        // let first = &v[0]; // 获取所有权 读取数据
        // v.push(6); // borrow 改变内容
        // println!("The first element is: {}", first);
    }
    // 为什么第一个元素的引用会关心 vector 结尾的变化？
    // 不能这么做的原因是由于 vector 的工作方式：在 vector 的结尾增加新元素时，在没有足够空间将所有所有元素依次相邻存放的情况下，可能会要求分配新内存并将老的元素拷贝到新的空间中。
    // 这时，第一个元素的引用就指向了被释放的内存。
    // 借用规则阻止程序陷入这种状况。

    // 遍历vector 中的元素
    {
        let v = vec![100, 32, 57];

        for i in &v {
            println!("{}", i);
        }
    }
    // 同时需要 index
    {
        let v = vec![100, 32, 57];

        for (v, index) in v.iter().enumerate() {
            println!("{},{}", v, index);
        }
    }
    {
        let mut v = vec![100, 32, 57];
        for i in &mut v {
            // 注意是 borrow mut
            // *i 类型 是 i32
            *i += 50; // 为了修改可变引用所指向的值，在使用 += 运算符之前必须使用解引用运算符（*）获取 i 中的值
            println!("{}", i);
        }
    }

    // 使用枚举来存储多种类型
    // 枚举的成员都被定义为相同的枚举类型，所以当需要在 vector 中储存不同类型值时，我们可以定义并使用一个枚举！
    {
        #[derive(Debug)]
        enum SpreadSheetCell {
            Int(i32),
            Float(f64),
            Text(String),
        }
        // 同样 rust 推断出了 vector 中的类型是 SpreadSheetCell 枚举
        let mut row = vec![
            SpreadSheetCell::Int(3),
            SpreadSheetCell::Text(String::from("blue")),
            SpreadSheetCell::Float(10.12),
        ];

        let some_data = row.pop(); // pop 弹出一个元素
        println!("{:?}", some_data.unwrap());
    }
}

pub fn ch08_02_strings() {
    // 使用字符串存储 UTF-8编码的文本
    // Rust 的核心语言中只有一种字符串类型：str，字符串 slice，它通常以被借用的形式出现，&str
    // String 是一个 Vec<u8> 的封装

    // 称作 String 的类型是由标准库提供的，而没有写进核心语言部分，
    // 它是可增长的、可变的、有所有权的、UTF-8 编码的字符串类型。

    // 新建字符串
    {
        let mut a = String::new(); // 使用类似Vector 的方法 新建一个空的字符串

        let data = "initial contents";
        let data: &str = "initial contents";

        let s = data.to_string();
        let s: String = data.to_string();

        // 该方法也可直接用于字符串字面值：
        let s = "initial contents".to_string();
        let s: String = "initial contents".to_string();

        // String::from 和 .to_string 最终做了完全相同的工作
        // 可以按照个人喜好选择使用
    }

    // 更新字符串
    {
        // 使用push 换个 push_str 更新字符串
        // 可以通过 push_str 方法来附加**字符串 slice**，从而使 String 变长
        let mut s1 = String::from("foo");
        let s2 = "bar";
        s1.push_str(s2);
        println!("s2 is {}", s2);

        // push 方法被定义为获取一个单独的字符作为参数，并附加到 String 中。
        let mut s3 = String::from("lo");
        s3.push('l');
        println!("s3 is {}", s3);
    }

    // 使用 + 运算符 或者 format!宏拼接字符串
    {
        let s1 = String::from("Hello, ");
        let s7 = String::from("Hello, ");
        let s2 = String::from("world");
        let s4 = String::from("!");
        // &s2 是 &String 可以被 强转（coerced）成 &str。
        // Rust 使用了一个被称为 解引用强制多态（deref coercion）的技术，你可以将其理解为它把 &s2 变成了 &s2[..]。
        let mut s3 = s1 + &s2 + &s4; // 注意 s1 被移动了，不能继续使用
        let s3 = s3.add(" 哈 ");
        // let s5 = &s2 + &s4; // error 不能全是 &str 类型
        let s6 = String::from("1");
        let s7 = String::from("2");
        // let s8 = s6 + s7; // error 第二个开始 就必须是 &str 类型
        // 其实就是 String 上 add 方法调用了2次
        // println!("{}",s1);
        println!("{}", s2);
        println!("{}", &s3);

        // 类似模块字符串
        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");

        // 并不会 获取所有权
        let s = format!("{}-{}-{}", s1, s2, s3);
        println!("{} {}", s, s1); // s1 s2 s3 依然有效

        // 之所以能够在 add 调用中使用 &s2 是因为 &String 可以被 强转（coerced）成 &str。
        // 当add函数被调用时，Rust 使用了一个被称为 解引用强制多态（deref coercion）的技术，
        // 你可以将其理解为它把 &s2 变成了 &s2[..]。
        let test = String::from("123");
        let test = &test[..];
    }
    // 索引字符串
    // rust 中是不能使用索引访问字符串的
    // 当使用 UTF-8 访问的时候 不能判断是 多少个字节是一个字符
    {
        // let s1 = String::from("hello");
        // let h = s1[0]; // error
    }
    // 字符串 slice
    // 可以使用 [] 和一个 range 来创建含特定字节的字符串 slice
    {
        let hello = "Здравствуйте";

        let s = &hello[0..4]; // 每个 Unicode 标量值需要两个字节存储
                              // let s = &hello[0..1]; // error
                              // thread 'main' panicked at 'byte index 1 is not a char boundary; it is inside 'З' (bytes 0..2) of `Здравствуйте`'
                              // rust 自动判断了 一个字符的边界

        println!("字符串 slice {}", s)
    }
    // 遍历字符串
    {
        // 返回六个 char 类型的值
        for c in "नमस्ते".chars() {
            println!("{}", c);
            // न
            // म
            // स
            // ्
            // त
            // े
        }

        // bytes 方法返回每一个原始字节
        for b in "नमस्ते".bytes() {
            println!("{}", b);
            // 打印18个字节
        }
    }
}

pub fn ch08_03_hash_maps() {
    // 哈希 map（hash map）。HashMap<K, V> 类型储存了一个键类型 K 对应一个值类型 V 的映射。
    // 它通过一个 哈希函数（hashing function）来实现映射，决定如何将键和值放入内存中。
    // 很多编程语言支持这种数据结构，不过通常有不同的名字：哈希、map、对象、哈希表或者关联数组，

    // 新建一个 hash map
    {
        use std::collections::HashMap;

        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);

        // 使用两个vector 生成 hashMap

        let teams = vec![String::from("Blue"), String::from("Yellow")];
        let initial_scores = vec![10, 50];

        // 这里 HashMap<_, _> 类型注解是必要的，因为可能 collect 很多不同的数据结构，而除非显式指定否则 Rust 无从得知你需要的类型。
        // 但是对于键和值的类型参数来说，可以使用下划线占位，而 Rust 能够根据 vector 中数据的类型推断出 HashMap 所包含的类型。
        // <_, _> 也就告诉 rust 不知道类型让他自己根据之后的操作自己判断吧
        let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    }
    // 哈希 map 和所有权
    // 对于像 i32 这样的实现了 Copy trait 的类型，其值可以拷贝进哈希 map。
    // 对于像 String 这样拥有所有权的值，其值将被移动而哈希 map 会成为这些值的所有者，
    {
        let field_name = String::from("Favorite color");
        let field_value = String::from("Blue");

        let mut map = HashMap::new();
        map.insert(field_name, field_value); // 移交所有权 虽然也能借用但是并不常用
        println!("{:?}", map);
    }

    // 访问哈希 map 中的值
    {
        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);

        let team_name = String::from("Blue");
        let score = scores.get(&team_name); // 返回的是 Option<&T> 类型
    }

    // 遍历 hashmap
    {
        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);

        for (key, value) in &scores {
            println!("{}: {}", key, value);
        }
    }

    // 更新哈希 map
    // 覆盖一个值
    {
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Blue"), 25);
        println!("{:?}", &scores);
    }
    // 只在键没有对应值时插入
    {
        // 此哈希 map 有一个特有的 API，叫做 entry，它获取我们想要检查的键作为参数
        // entry 函数的返回值是一个枚举
        // Entry，它代表了可能存在也可能不存在的值
        let mut scores = HashMap::new();
        scores.insert(String::from("blue"), 10);

        // 使用 entry 方法只在键没有对应一个值时插入
        scores.entry(String::from("Yellow")).or_insert(50);
        scores.entry(String::from("Blue")).or_insert(50);

        println!("{:?}", scores);
    }
    // 根据就值 更新一个值
    {
        let text = "hello world wonderful world";

        let mut map = HashMap::new();

        for word in text.split_whitespace() {
            // or_insert 方法事实上会返回这个键的值的一个可变引用（&mut V）。
            let count = map.entry(word).or_insert(0);
            // 这里我们将这个可变引用储存在 count 变量中，所以为了赋值必须首先使用星号（*）解引用 count。
            // 这个可变引用在 for 循环的结尾离开作用域，这样所有这些改变都是安全的并符合借用规则。
            *count += 1;
        }

        println!("{:?}", map); // {"world": 2, "hello": 1, "wonderful": 1}
    }
}
