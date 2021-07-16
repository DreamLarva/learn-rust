// main.rs 一般使用自己项目的使用 只给自己运行生产用
#![allow(unused_variables)] // 不对 未使用的变量 warning

use std::collections::HashMap;
use std::fs::{self, File};
use std::io::ErrorKind;
use std::io::Read;
use std::ops::{Add, Index};
use std::{env, io, process};

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

// 引用当前的 crate 也就是 lib 中的内容, 使用的文件名就是 当前项目的名字
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

    // rust 中 文件夹 中的 mod.rs 相当于整个文件夹的索引(类似js 中的index.js)
    // ch07::main()

    // ch08_vec::ch08_01_vectors();
    ch08_string::ch08_02_strings();
    // ch08_hashmap::ch08_03_hash_maps();

    // ch09::ch09_01_unrecoverable_errors_with_panic();
    // ch09::ch09_02_recoverable_errors_with_result();
    // ch09::ch09_03_to_panic_or_not_to_panic();

    // ch10::ch10_01_syntax();
    // ch10::ch10_02_traits();
    // ch10::ch10_03_lifetime_syntax();

    // let args: Vec<_> = env::args().collect();
    // let config = Config::new(&args).unwrap_or_else(|err| {
    //     // eprintln! 宏来打印到标准错误流 否则如果输出到文件的话,会将错误信息输出到文件
    //     eprintln!("problem parsing arguments: {}", err);
    //     process::exit(1)
    // });
    // println!("Searching for {}", config.query);
    // println!("In file {}", config.filename);
    //
    // if let Err(e) = the_rust_programming_language::run(config) {
    //     // eprintln! 宏来打印到标准错误流
    //     eprintln!("Application error : {}", e);
    //     process::exit(1);
    // }

    // ch13::ch13_01_closures();
    // ch13::ch13_02_iterators();
}
