#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// Include foo bindings
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

fn main() {
    // Show how Rust-safe bindings have been created for Foo
    let foo = Foo { bar: 12 };
    println!("Hello world from Rust! I have a C-compatible struct of {:?}", &foo);

    // Rust does not consider pointers safe to the official language, so
    // `unsafe` is used. It is possible to make convert Foo types to pointers.
    unsafe {
        // Create an empty Foo instance with a `Foo:bar` value of 0.
        let foo = foo_new();
        foo_hello_world(foo);

        // Dereference the pointer, alter `Foo:bar`, and print again.
        (*foo).bar = 4;
        foo_hello_world(foo);

        // Cleanup the underlying C memory, as Rust does not use `malloc()` and
        // `free()`
        foo_free(foo);
    }
}
