import foo_cffi

if __name__ == '__main__':
    foo = foo_cffi.lib.foo_new()

    print(f'Hello world from Python! I have a value of {foo.bar}')
    foo_cffi.lib.foo_hello_world(foo)

    foo.bar = 3
    print(f'Hello world from Python! I now have a value of {foo.bar}')
    foo_cffi.lib.foo_hello_world(foo)

    foo_cffi.lib.foo_free(foo)
