#![allow(unused_variables)] // 不对 未使用的变量 warning

use std::collections::*;
use std::fs::{self, File};
use std::io;
use std::io::ErrorKind;
use std::io::Read;
use std::ops::Add;

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
    // 根据旧值 更新一个值
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
