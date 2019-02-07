use std::collections::HashMap;
use std::fs::{self,File};
use std::io::ErrorKind;
use std::io;
use std::io::Read;

mod ch08;
mod ch09;

fn main() {
    ch08::ch08_01_vectors();
    ch08::ch08_02_strings();
    ch08::ch08_03_hash_maps();

    ch09::ch09_01_unrecoverable_errors_with_panic();
    ch09::ch09_02_recoverable_errors_with_result();
    ch09::ch09_03_to_panic_or_not_to_panic();
}



