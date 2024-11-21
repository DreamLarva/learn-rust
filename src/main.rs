// main.rs 一般使用自己项目的使用 只给自己运行生产用
#![allow(unused_variables)] // 不对 未使用的变量 warning
// #![feature(trace_macros)]
// #![feature(log_syntax)]
// use std::collections::HashMap;
// use std::fs::{self, File, OpenOptions};
// use std::io::Read;
// use std::io::{ErrorKind, Write};
// use std::ops::{Add, Index};
// use std::{env, io, process};

// extern crate core;

mod ch01;
mod ch02;
mod ch03;
mod ch04;
mod ch05;
mod ch06;
mod ch07;
mod ch08_hashmap;
mod ch08_string;
mod ch08_vec;
mod ch09;
mod ch10;
mod ch11_01;
mod ch11_02;
mod ch11_03;
mod ch12;
mod ch13;
mod ch14;
mod ch15;
mod ch15_02;
mod ch15_03;
mod ch15_04;
mod ch16;
mod ch17;
mod ch18;
mod ch19;
mod ch19_06;
mod ch20;
mod ch15_05;
// mod macro_book1;
// mod macro_book2;
// mod macro_book3;
// mod macro_book4;
// mod macro_book5_scope1;
// mod macro_book5_scope2;
// mod macro_book5_scope3;
// mod macro_book5_scope4;
// mod macro_book5_scope5;
// mod macro_book5_scope6;
// mod macro_book5_scope7;
// mod macro_book5_scope8;
// mod macro_book6_import_export1;
// mod macro_book6_import_export2;
// mod macro_book7_example;

use hex::hex;

// 引用当前的 crate 也就是 lib 中的内容, 使用的文件名就是 当前项目的名字
use std::cell::{Ref, RefCell};
use std::collections::{hash_map, HashMap};
use std::fmt::{Debug, Display};
use std::io::Write;
use std::rc::Rc;
use std::slice::from_raw_parts;
use std::str::from_utf8_unchecked;
use std::{env, fs, io, process};
use std::hash::Hash;
use std::num::ParseIntError;
use the_rust_programming_language::Config;

pub fn main() {
    // ch01::main();

    // ch02::ch02_00_guessing_game_tutorial()

    // ch03::ch03_02_data_types();
    // ch03::ch03_03_how_functions_work();
    // ch03::ch03_05_control_flow();

    // ch04::ch04_01_what_is_ownership();
    // ch04::ch04_02_references_and_borrowing();
    // ch04::ch04_03_slices();

    // ch05::ch05_01_defining_structs();
    // ch05::ch05_02_example_structs();
    // ch05::ch05_03_method_syntax();

    // ch06::ch06_01_defining_an_enum();
    // ch06::ch06_02_match();
    // ch06::ch06_03_if_let();
    // ch06::ch06_03_matches();

    // rust 中 文件夹 中的 mod.rs 相当于整个文件夹的索引(类似js 中的index.js)
    // ch07::main()

    // ch08_vec::ch08_01_vectors();
    // ch08_string::ch08_02_strings();
    // ch08_hashmap::ch08_03_hash_maps();

    // ch09::ch09_01_unrecoverable_errors_with_panic();
    // ch09::ch09_02_recoverable_errors_with_result();
    // ch09::ch09_03_to_panic_or_not_to_panic();

    // ch10::ch10_01_syntax();
    // ch10::ch10_02_traits();
    // println!("imply Summary for Vec, {}", vec![1].summarize()); // 没有实现 Summary

    // ch10::ch10_03_lifetime_syntax();

    // ch13::ch13_01_closures();
    // ch13::ch13_02_iterators();

    // 用迭代器改进后
    // let config = Config::new(env::args()).unwrap_or_else(|err| {
    //     // eprintln! 宏来打印到标准错误流 否则如果输出到文件的话,会将错误信息输出到文件
    //     eprintln!("problem parsing arguments: {}", err);
    //     process::exit(1)
    // });
    // println!("Searching for {}", config.query);
    // println!("In file {}", config.filename);

    // if let Err(e) = the_rust_programming_language::run(config) {
    //     // eprintln! 宏来打印到标准错误流
    //     eprintln!("Application error : {}", e);
    //     process::exit(1);
    // }

    // ch15::ch15_01_box();
    // ch15::ch15_02_deref();
    // ch15::ch15_03_drop();
    // ch15::ch15_04_rc();
    // ch15_02::main();
    // ch15_03::main()
    // ch15_04::main();

    // ch16::ch16_01_threads();
    // ch16::ch16_02_message_passing();
    // ch16::ch16_03_shared_state();

    // ch17::ch17_01whatis_oo();
    // ch17::ch17_02_trait_objects();
    // ch17::ch17_03_oo_design_patterns();

    // ch18::ch18_03_pattern_syntax();

    // ch19::ch19_01_unsafe_rust()
    // ch19::ch19_03_advanced_traits()
    // ch19::ch19_04_advanced_types();
    // ch19::ch19_05_advanced_functions_and_closures();

    // hex!(1 + 2, 3 + 4, 5 + 6);
    // macro_book1::main();
    // macro_book2::main();
    // macro_book3::main();

    // X!();
    // Y!();

    // macro_book7_example::main();

    // ch20::main();

    // println!("{:?}", m);

    use glob::glob;
    use glob::glob_with;
    use glob::MatchOptions;

    let from = "/Users/yangjiaqi/Desktop/project/rust/my-learn-rust/src/**/*.*";

    let options = MatchOptions {
        case_sensitive: false,
        require_literal_separator: false,
        require_literal_leading_dot: false,
    };
    for entry in glob_with(from, options).unwrap() {
        if let Ok(path) = entry {
            println!("{:?}", path.display())
        }
    }


    fn groupBy<T, V: Eq + Hash>(vec: Vec<T>, fun: &dyn Fn(&T) -> V) -> HashMap<V, Vec<T>> {
        let mut groups = HashMap::new();
        for v in vec.into_iter() {
            groups.entry(fun(&v)).or_insert(Vec::new()).push(v)
        }

        groups
    }

    let a = groupBy(vec![11, 12, 2, 1, 2, 1, 2, 1231], &|v| v.to_string());
    println!("{:?}", a);

    fn groupBy2<T, V, P>(vec: &Vec<T>, mut fun: P) -> HashMap<V, Vec<&T>>
    where
        V: Eq + Hash,
        P: FnMut(&T) -> V,
    {
        let mut groups = HashMap::new();
        for v in vec.iter() {
            groups.entry(fun(&v)).or_insert(Vec::new()).push(v)
        }
        groups
    }

    let mut b = vec![11, 12, 2, 1, 2, 1, 2, 1231];
    let a = groupBy2(&b, |v| {
        (v % 10)
    });


    println!("{:?}", a);
    println!("{:?}", b);
    b.push(21);
    println!("{:?}", b);


    fn test(n: i32) -> Box<dyn Display> {
        if n > 0 {
            Box::new(1i32)
        } else {
            Box::new(1.1)
        }
    }

    fn example(condition: bool, vec: Vec<i32>) -> Box<dyn Iterator<Item=i32>> {
        let iter = vec.into_iter();
        if condition {
            // Has type:
            // Box<Map<IntoIter<i32>, Fn(i32) -> i32>>
            // But is cast to:
            // Box<dyn Iterator<Item = i32>>
            Box::new(iter.map(|n| n * 2))
        } else {
            // Has type:
            // Box<Filter<IntoIter<i32>, Fn(&i32) -> bool>>
            // But is cast to:
            // Box<dyn Iterator<Item = i32>>
            Box::new(iter.filter(|&n| n >= 2))
        }
    }

    example(true, vec![1, 2, 4]);
    example(false, vec![1, 2, 4]);


    use std::convert::{TryFrom, TryInto};
    use std::error;
    use std::fmt;

    struct Point {
        x: i32,
        y: i32,
    }

    #[derive(Debug)]
    struct OutOfBounds;

    impl fmt::Display for OutOfBounds {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "out of bounds")
        }
    }

    impl error::Error for OutOfBounds {}

    impl TryFrom<(i32, i32)> for Point {
        type Error = OutOfBounds;
        fn try_from((x, y): (i32, i32)) -> Result<Self, Self::Error> {
            if x.abs() > 1000 || y.abs() > 1000 {
                return Err(OutOfBounds);
            }
            Ok(Point { x, y })
        }
    }

    struct Triangle {
        p1: Point,
        p2: Point,
        p3: Point,
    }

    impl<P> TryFrom<[P; 3]> for Triangle
    where
        P: TryInto<Point>,
    {
        type Error = P::Error;
        fn try_from([p1, p2, p3]: [P; 3]) -> Result<Self, Self::Error> {
            Ok(Triangle {
                p1: p1.try_into()?,
                p2: p2.try_into()?,
                p3: p3.try_into()?,
            })
        }
    }

    fn example1() -> Result<Triangle, OutOfBounds> {
        let t: Triangle = [(0, 0), (1, 1), (2, 2)].try_into()?;
        Ok(t)
    }
}