#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// Include foo bindings
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

fn main() {
    println!("Hello world from Rust!");
}