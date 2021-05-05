import cffi
import os

"""Build foo Python bindings"""
print('Building foo Python bindings')
ffi = cffi.FFI()

prefix = os.environ['HOME'] + '/opt'
libdir = prefix + '/lib'

# Load our C definitions using a slightly-modified version of our original
# header to meet CFFI's current level of parsing support.

# NOTE: In a more robust implementation, dynamically remove the pre-processor
# directives, or use Cython to auto-generate wrapper Python modules.
thisdir = os.path.dirname(os.path.realpath(__file__))
foo_h = thisdir + '/foo.h'
with open(foo_h) as f:
    ffi.cdef(f.read())

ffi.set_source(
    "foo_cffi",
    # create bindings with the custom foo.h header
    '#include "foo.h"',
    # -lfoo
    libraries=['foo'],
    # -L${libdir}
    library_dirs=[libdir],
)

if __name__ == '__main__':
    ffi.compile(verbose=True)
