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
use std::{env, fs, process};
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

    let mut a = Box::new(1);
    println!("{a}");

    *a = 2;

    println!("{a}");

}
