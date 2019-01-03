# rust-cython-test
Code originally taken from [this earlier example](https://github.com/synapticarbors/rust-cython-test).

Here is a simple example of calling a Rust string functions sing [Cython](http://cython.org/). 
There is a simple function for one string, and two versions of arrayed calls.

Basic instructions for running the example:

```
./make
```

Then in python:

```
>>> import cytest

>>> cytest.call_rust_upper('hello')
'HELLO'
```

This example is obviously not very useful, but I wanted to demonstrate the capability.

### Build details

Include file:
```
const char *my_upper(char *b);
int init();
```

Setup file:
```python
ext = Extension(
    'cytest',
    sources=['cytest.pyx'],
    libraries=['rustcython_string'],
    library_dirs=['target/release'],
    include_dirs=['.']
)
```

Make file:
```bash
cargo build --release
python setup.py build_ext --inplace
```

Cargo file:
```toml
[lib]
name = "rustcython_string"
crate-type = ["dylib"]
```

Cython
```python
from cpython cimport PyUnicode_AsUTF8String, PyUnicode_InternFromString

cdef extern from "rlib.h":
    char *my_upper(char *b)
    int init()

def call_rust_upper(unicode s):
    cdef unicode out
    return PyUnicode_InternFromString(my_upper(PyUnicode_AsUTF8String(s)))
```