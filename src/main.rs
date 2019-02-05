use std::collections::HashMap;
use std::fs::{self,File};
use std::io::ErrorKind;
use std::io;
use std::io::Read;


fn main() {
//    ch08_01_vectors();
//    ch08_02_strings();
//    ch08_03_hash_maps();
//    ch09_01_unrecoverable_errors_with_panic();
//    ch09_02_recoverable_errors_with_result();
    ch09_03_to_panic_or_not_to_panic();
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
            *i += 50; // 为了修改可变引用所指向的值，在使用 += 运算符之前必须使用解引用运算符（*）获取 i 中的值
            println!("{}", i);
        }
    }

    // 使用枚举来存储多种类型
    // 枚举的成员都被定义为相同的枚举类型，所以当需要在 vector 中储存不同类型值时，我们可以定义并使用一个枚举！
    {
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

        row.pop(); // pop 弹出一个元素
    }
}

fn ch08_02_strings() {
    // 使用字符串存储 UTF-8编码的文本
    // Rust 的核心语言中只有一种字符串类型：str，字符串 slice，它通常以被借用的形式出现，&str

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
        let s2 = String::from("world");
        let s4 = String::from("!");
        // &s2 是 &String 可以被 强转（coerced）成 &str。
        // Rust 使用了一个被称为 解引用强制多态（deref coercion）的技术，你可以将其理解为它把 &s2 变成了 &s2[..]。
        let s3 = s1 + &s2 + &s4; // 注意 s1 被移动了，不能继续使用
//        println!("{}",s1);
        println!("{}", s2);
        println!("{}", s3);

        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");

        let s = format!("{}-{}-{}", s1, s2, s3);
        println!("{}", s)
    }
    // 字符串 slice
    {
        let hello = "Здравствуйте";

        let s = &hello[0..4]; // 每个 Unicode 标量值需要两个字节存储

        println!("{}", s)
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

fn ch08_03_hash_maps() {
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

        let teams = vec![String::from("Blue"), String::from("Yellow")];
        let initial_scores = vec![10, 50];

        let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    }
    // 哈希 map 和所有权
    {
        let field_name = String::from("Favorite color");
        let field_value = String::from("Blue");

        let mut map = HashMap::new();
        map.insert(field_name, field_value);  // 移交所有权 虽然也能借用但是并不常用

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
}

fn ch09_01_unrecoverable_errors_with_panic() {
    // !panic 与不可恢复错误
    {
        // panic!("crash and burn"); // 抛出错误
    }
}

fn ch09_02_recoverable_errors_with_result() {

    //    enum Result<T, E> {
//        Ok(T), // 成功的类型
//        Err(E), // 错误的类型
//    }

    /*{
        let f = File::open("hello.txt");
        let f = match f {
            Ok(file) => file,
            Err(error) => { // 匹配错误
                panic!("there is was a problem opening the file: {:?}",error) // 不能注释 否则不能编译

            }
        };
    }*/
    // 匹配不同的错误
    {
        let f = File::open("hello.txt");
        let f = match f {
            Ok(file) => file,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("Tired to create file but there is a problem: {:?}", e),
                }
                other_error => panic!("there was a problem opening the file: {:?}", other_error),
            }
        };
    }
    // 个更老练的 Rustacean 可能会这么写
    {
        let f = File::open("hello.txt").map_err(|error| {
            if error.kind() == ErrorKind::NotFound {
                File::create("hello.txt").unwrap_or_else(|error| {
                    panic!("Tried to create file but there was a problem: {:?}", error);
                })
            } else {
                panic!("There was a problem opening the file: {:?}", error);
            }
        });
    }

    // 失败时 panic 的简写 : unwrap 和 expect
    {
        // 如果 Result 值是成员 Ok，unwrap 会返回 Ok 中的值。
        // 如果 Result 是成员 Err，unwrap 会为我们调用 panic!。
//         let f = File::open("hello1.txt").unwrap();

        // 使用 expect 而不是 unwrap 并提供一个好的错误信息可以表明你的意图并更易于追踪 panic 的根源。
//        let f = File::open("hello1.txt").expect("Failed to open hello.txt");
    }

    // 传播错误
    // 当编写一个其实现会调用一些可能会失败的操作的函数时，除了在这个函数中处理错误外，还可以选择让调用者知道这个错误并决定该如何处理。
    // 这被称为 传播（propagating）错误，这样能更好的控制代码调用，因为比起你代码所拥有的上下文，调用者可能拥有更多信息或逻辑来决定应该如何处理错误。
    {
        fn read_username_from_file() -> Result<String, io::Error> {
            let f = File::open("hello.txt");

            let mut f = match f { // 如果没有文件就直接报错
                Ok(file) => file,
                Err(e) => return Err(e),
            };

            let mut s = String::new();
            // 有文件返回的内容
            match f.read_to_string(&mut s) { // 将读取到的内容 放到 字符串s中
                Ok(_) => Ok(s),
                Err(e) => Err(e),
            }
        }

        match read_username_from_file(){
            Ok(s) => println!("data is: {}",s),
            Err(e) => panic!("{:?}",e)
        }
    }

    // 传播错误的简写 : ?
    {
        // 在Result 值之后的 ?
        // 如果值是ok 就返回ok 中的值 继续执行
        // 如果是Err 就将Err 中的值作为整个函数的返回值 就像使用了return 的关键字一样
        fn read_username_from_file() -> Result<String, io::Error> {
            let mut f = File::open("hello.txt")?;
            let mut s = String::new();
            f.read_to_string(&mut s)?;
            Ok(s)
        }
        match read_username_from_file(){
            Ok(s) => println!("data is: {}",s),
            Err(e) => panic!("{:?}",e)
        }
    }
    // 进一步使用链式调用缩短代码 链式调用中也能够是用 ?
    {
        fn read_username_from_file() -> Result<String,io::Error>{
            let mut s = String::new();
            File::open("hello.txt")?.read_to_string(&mut s)?;
            Ok(s)
        }
    }
    {
        //  Rust 提供了名为 fs::read_to_string 的函数，它会打开文件、新建一个 String、读取文件的内容，并将内容放入 String，接着返回它。
        // 当然，这样做就没有展示所有这些错误处理的机会了
        fn read_username_from_file() -> Result<String, io::Error> {
            fs::read_to_string("hello.txt")
        }
    }

    // ? 只能被用于返回 Result 的函数
    {
        // 错误指出只能在返回 Result 的函数中使用 ?。在不返回 Result 的函数中，当调用其他返回 Result 的函数时，需要使用 match 或 Result 的方法之一来处理，而不能用 ? 将潜在的错误传播给代码调用方。
//        let f = File::open("hello.txt")?;
    }

}

fn ch09_03_to_panic_or_not_to_panic(){
    // 示例、代码原型和测试都非常适合 panic
    // 当你编写一个示例来展示一些概念时，在拥有健壮的错误处理代码的同时也会使得例子不那么明确。
    // 例如，调用一个类似 unwrap 这样可能 panic! 的方法可以被理解为一个你实际希望程序处理错误方式的占位符，它根据其余代码运行方式可能会各不相同。
    //
    //类似地，在我们准备好决定如何处理错误之前，unwrap和expect方法在原型设计时非常方便。
    // 当我们准备好让程序更加健壮时，它们会在代码中留下清晰的标记。
    //
    //如果方法调用在测试中失败了，我们希望这个测试都失败，即便这个方法并不是需要测试的功能。
    // 因为 panic! 是测试如何被标记为失败的，调用 unwrap 或 expect 就是应该发生的事情。

    // 当我们比编译器知道更多的情况
    // 当你有一些其他的逻辑来确保 Result 会是 Ok 值时，调用 unwrap 也是合适的，
    // 虽然编译器无法理解这种逻辑。你仍然需要处理一个 Result 值：即使在你的特定
    // 情况下逻辑上是不可能的，你所调用的任何操作仍然有可能失败。如果通过人工检
    // 查代码来确保永远也不会出现 Err 值，那么调用 unwrap 也是完全可以接受的，
    // 这里是一个例子：
    {
        use std::net::IpAddr;
                            // 硬编码的字符
        let home: IpAddr = "127.0.0.1".parse().unwrap(); // 我们知道绝对不会报错 但是编译器不知道仍然返回是Result
    }

    // 错误处理知道原则
    // 在当有可能会导致有害状态的情况下建议使用 panic! —— 在这里，有害状态是指当一些假设、
    // 保证、协议或不可变性被打破的状态，例如无效的值、自相矛盾的值或者被传递了不存在的值 ——
    // 外加如下几种情况：
    //  有害状态并不包含 预期 会偶尔发生的错误
    //  之后的代码的运行依赖于处于这种有害状态
    //  当没有可行的手段来将有害状态信息编码进所使用的类型中的情况

    // 如果别人调用你的代码并传递了一个没有意义的值，最好的情况也许就是 panic! 并警告使用你的
    // 库的人他的代码中有 bug 以便他能在开发时就修复它。类似的，panic! 通常适合调用不能够控制
    // 的外部代码时，这时无法修复其返回的无效状态。

    // 然而当错误预期会出现时，返回 Result 仍要比调用 panic! 更为合适。这样的例子包括解析器接
    // 收到错误数据，或者 HTTP 请求返回一个表明触发了限流的状态。在这些例子中，应该通过返回
    // Result 来表明失败预期是可能的，这样将有害状态向上传播，调用者就可以决定该如何处理这个问
    // 题。使用 panic! 来处理这些情况就不是最好的选择。

    // 当代码对值进行操作时，应该首先验证值是有效的，并在其无效时 panic!。这主要是出于安全的原
    // 因：尝试操作无效数据会暴露代码漏洞，这就是标准库在尝试越界访问数组时会 panic! 的主要原
    // 因：尝试访问不属于当前数据结构的内存是一个常见的安全隐患。函数通常都遵循 契约（contracts）
    // ：他们的行为只有在输入满足特定条件时才能得到保证。当违反契约时 panic 是有道理的，因为这通
    // 常代表调用方的 bug，而且这也不是那种你希望调用方必须处理的错误。事实上也没有合理的方式来
    // 恢复调用方的代码：调用方的 程序员 需要修复其代码。函数的契约，尤其是当违反它会造成 panic
    // 的契约，应该在函数的 API 文档中得到解释。

    // 虽然在所有函数中都拥有许多错误检查是冗长而烦人的。幸运的是，可以利用 Rust 的类型系统（以
    // 及编译器的类型检查）为你进行很多检查。如果函数有一个特定类型的参数，可以在知晓编译器已经
    // 确保其拥有一个有效值的前提下进行你的代码逻辑。例如，如果你使用了一个不同于 Option 的类型
    // ，而且程序期望它是 有值 的并且不是 空值。你的代码无需处理 Some 和 None 这两种情况，它只
    // 会有一种情况就是绝对会有一个值。尝试向函数传递空值的代码甚至根本不能编译，所以你的函数在
    // 运行时没有必要判空。另外一个例子是使用像 u32 这样的无符号整型，也会确保它永远不为负。

}

