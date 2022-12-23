use std::{
    thread::{self},
    time::Duration,
};

use human_time_macros::elapsed;

#[elapsed]
fn main() {
    thread::sleep(Duration::from_secs(1));
    foo();
}

#[elapsed]
fn foo() -> i32 {
    1_i32
}
