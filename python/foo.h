/* This file defines the shared bindings between C and the Python FFI module
 * taken from `sample-ffi/c/foo.h` */

struct Foo {
	int bar;
};

/**
 * @brief Returns a pointer to an allocated instance of Foo.
 * @return Pointer to a new Foo instance.
 */
struct Foo *foo_new();

/**
 * @brief Frees a Foo pointer (nothing happens if f is NULL)
 * @param f A Foo pointer (nullable)
 */
void foo_free(struct Foo *f);

/**
 * @brief Prints "hello, world! I have a value of %d", f->bar.
 * @param f A Foo pointer (not nullable)
 */
void foo_hello_world(struct Foo *f);
