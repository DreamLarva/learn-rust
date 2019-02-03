fn main() {
//    ch08_01_vectors();
    ch08_02_strings()
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








}