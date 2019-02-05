use std::collections::HashMap;


fn main() {
//    ch08_01_vectors();
//    ch08_02_strings();
    ch08_03_hash_maps();
}

fn ch08_01_vectors() {
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
        let mut v = Vec::new();
        v.push(5);
        v.push(6);
        v.push(7);
        v.push(8);
        // v.push("1"); // error
    }

    // 丢弃vector 时 也会丢弃其所有的元素
    {
        let v = vec![1, 2, 3];
    } // 此处已经清理所有的元素 和vector


    // 读取vector 元素
    {
        let v = vec![1, 2, 3, 4, 5];
        let third: &i32 = &v[2]; // 索引语法
        println!("the third element is {}", third);

        // get 语法 返回的 是以索引作为参数来返回一个 Option<&T>
        match v.get(2) {
            Some(third) => println!("The third element is {}", third),
            None => println!("There is no third element"),
        }

        // 尝试读取 越界的元素
//        let does_not_exist = &v[100]; // error
        let does_not_exist = v.get(100);
    }

    // 一旦程序获取了一个有效的引用，
    // 借用检查器将会执行所有权和借用规则（第四章讲到）来确保 vector 内容的这个引用和任何其他引用保持有效。
    // 只能有 一个 mut 或者 多个 &
    {
//        let mut v = vec![1, 2, 3, 4, 5];
//
//        let first = &v[0]; // 获取所有权 读取数据
//
//        v.push(6); // borrow 改变内容
//
//        println!("The first element is: {}", first);
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
    {
        let mut v = vec![100, 32, 57];
        for i in &mut v {
            *i += 50; // 为了修改可变引用所指向的值，在使用 += 运算符之前必须使用解引用运算符（*）获取 i 中的值
            println!("{}", i);
        }
    }

    // 使用枚举来存储多种类型
    // 枚举的成员都被定义为相同的枚举类型，所以当需要在 vector 中储存不同类型值时，我们可以定义并使用一个枚举！
    {
        enum SpreadSheetCell{
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

        row.pop(); // pop 弹出一个元素

    }
}

fn ch08_02_strings(){
    // 使用字符串存储 UTF-8编码的文本
    // Rust 的核心语言中只有一种字符串类型：str，字符串 slice，它通常以被借用的形式出现，&str

    // 称作 String 的类型是由标准库提供的，而没有写进核心语言部分，
    // 它是可增长的、可变的、有所有权的、UTF-8 编码的字符串类型。

    // 新建字符串
    {
        let mut a = String::new(); // 使用类似Vector 的方法 新建一个空的字符串

        let data = "initial contents";
        let data:&str = "initial contents";

        let s = data.to_string();
        let s:String = data.to_string();

        // 该方法也可直接用于字符串字面值：
        let s = "initial contents".to_string();
        let s:String = "initial contents".to_string();

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
        let s2 = String::from("world");
        let s4 = String::from("!");
        // &s2 是 &String 可以被 强转（coerced）成 &str。
        // Rust 使用了一个被称为 解引用强制多态（deref coercion）的技术，你可以将其理解为它把 &s2 变成了 &s2[..]。
        let s3 = s1 + &s2 + &s4; // 注意 s1 被移动了，不能继续使用
//        println!("{}",s1);
        println!("{}",s2);
        println!("{}",s3);

        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");

        let s = format!("{}-{}-{}", s1, s2, s3);
        println!("{}",s)
    }
    // 字符串 slice
    {
        let hello = "Здравствуйте";

        let s = &hello[0..4]; // 每个 Unicode 标量值需要两个字节存储

        println!("{}",s)
    }
    // 遍历字符串
    {
        // 返回六个 char 类型的值
        for c in "नमस्ते".chars() {
            println!("{}", c);
        }

        // bytes 方法返回每一个原始字节
        for b in "नमस्ते".bytes() {
            println!("{}", b);
        }
    }












}

fn ch08_03_hash_maps(){
    //  哈希 map（hash map）。HashMap<K, V> 类型储存了一个键类型 K 对应一个值类型 V 的映射。
    // 它通过一个 哈希函数（hashing function）来实现映射，决定如何将键和值放入内存中。
    // 很多编程语言支持这种数据结构，不过通常有不同的名字：哈希、map、对象、哈希表或者关联数组，

    // 新建一个 hash map
    {
        use std::collections::HashMap;

        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);

        // 使用两个vector 生成 hashMap

        let teams  = vec![String::from("Blue"), String::from("Yellow")];
        let initial_scores = vec![10, 50];

        let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    }
    // 哈希 map 和所有权
    {

        let field_name = String::from("Favorite color");
        let field_value = String::from("Blue");

        let mut map = HashMap::new();
        map.insert(field_name, field_value);  // 移交所有权 虽然也能借用但是并不常用

        println!("{:?}",map);

    }

    // 访问哈希 map 中的值
    {

        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);

        let team_name = String::from("Blue");
        let score = scores.get(&team_name); // 返回的是 Option<&T> 类型
    }

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
        scores.insert(String::from("blue"),10);

        // 使用 entry 方法只在键没有对应一个值时插入
        scores.entry(String::from("Yellow")).or_insert(50);
        scores.entry(String::from("Blue")).or_insert(50);

        println!("{:?}", scores);
    }







}