# rust-cython-string

**Summary**: Rust can be successfully called from python, but requires a little
finesse for arrays. Since Rust has UTF8 strings as first-class citizens, and a host of
methods on them in its standard library, I thought it might be the best way to
get around the frequent bottle-neck in python data-science: processing arrays of
strings.
I did not find the acceleration I was expecting, but this is probably don't to my 
ignorance (help appreciated!).

## Introduction
I have spent some months at the back end of 2018 teaching myself Rust, from the 
[official tutorial](https://doc.rust-lang.org/book/2018-edition/index.html) and 
[excercism](https://exercism.io/my/tracks/rust). For those that don't know, Rust is a relatively new
language with low-level (C-like) performance, but nice functional and iterator semantics,
as well as strict rules for memory ownership to prevent those all-too-common segfaults.
Yes, I know you can do UTF8 processing in C/C++ too...

This small effort is also influenced
by the attempt of [fletcher](https://github.com/xhochy/fletcher) to do array-string operations
by compiling with [numba](https://numba.pydata.org) - which suffers from having to re-implement
bytes-wise algorithms to simulate string operations, and has no UTF8 handling at all.

I started by adapting code from [this earlier example](https://github.com/synapticarbors/rust-cython-test),
which shows how to call a Rust-compiled function from cython. 
My version has been adapted to perform the operation `upper` on a single string or on
an array of strings.

Basic instructions for running the example:

```console
$ ./make
```

and the simplest function (note that inputs and outputs here are unicode - this is running on py3)

```python
>>> import cytest

>>> cytest.call_rust_upper('hello')
'HELLO'
```

This example is obviously not very useful, as we are creating a python object, and the
round-trip is about 6x slower than `'hello'.upper()`, but I wanted to demonstrate the capability.

### Build details

How it was done may be generally useful to others. Feel free to skip the details!

The Rust part is standard, with the following function (note the C semantics)
```rust
#[no_mangle]
pub extern "C" fn my_upper(b: *const i8) -> *const u8 {
    let s = unsafe { CStr::from_ptr(b) };
    s.to_str().unwrap().to_uppercase().as_ptr()
}
```

which takes a pointer, assumes it is a valid UTF8 string, calls uppper on it and returns a (C)
pointer to the result. Note that creating a local variable from a passed pointer is always unsafe,
but this is the only thing we can do unless we actually allocate and assign strings within Rust.

The cargo stub is straight-forward:

```toml
[lib]
name = "rustcython_string"
crate-type = ["dylib"]
```

On the cython side, we only need declare the function as external, but if we want to
work with the python object, we need to both be absle to get a pointer to the string out,
and construct a new instance with the result from Rust. This looks like:  

Cython
```python
from cpython cimport PyUnicode_AsUTF8String, PyUnicode_InternFromString

cdef extern:
    char *my_upper(char *b)

def call_rust_upper(unicode s):
    cdef unicode out
    return PyUnicode_InternFromString(my_upper(PyUnicode_AsUTF8String(s)))
```

To build it, you simply declare the dependant library
```python
ext = Extension(
    'cytest',
    sources=['cytest.pyx'],
    libraries=['rustcython_string'],
    library_dirs=['target/release'],
    include_dirs=['.']
)
```

... and make
```bash
cargo build --release
python setup.py build_ext --inplace
```

As noted, `cytest.call_rust_upper` works, but is slow compared to the builtin method, which is perhaps
not too surprising, as we are not skipping object instantiation, but we are adding external call cruft.

### arrays?

The point would be, to be fast by passing in (UTF8) bytes and takeing out bytes. Here, I used
[awkward array](https://github.com/scikit-hep/awkward-array/tree/master/awkward): a library for
traversing nested, variable-length data schema in numba loops and computing on the leaf data entries
without creating python instances, when run in a numba-jitted function. Well worth looking into. From
the point of view of this article, it just makes a bytes-type array from string data with offset start
and end arrays, so I am not really using its power here; this is rather like Arrow layout of variable-length
bytes data.

The function `do_carray_upper` in [the source](https://github.com/martindurant/rust-cython-test/blob/master/cytest.pyx)
iterates over the offsets and uses cython to make pointers to pass to Rust. Data is assigned back out
into another identical array using casting, and the whole loop shows no yellow in the cython annotation,
i.e., no python calls at all. The function, to my surprise, works!

![Annotation](https://raw.githubusercontent.com/martindurant/rust-cython-test/59686d0d6ea89c0f321a5113a19509984561d0eb/annot.pnghttps://raw.githubusercontent.com/martindurant/rust-cython-test/master/annot.png)

Timing:
```python
>>> arr = awkward.StringArray.fromiter(['hello', 'oi']*1000000)
>>> %timeit cytest.array_upper(arr)
135 ms
```
compares to (75ns per .upper()) * 200000strings = 150ms. To actually run the list comprehension
takes longer for building the list; but pandas takes even longer:
```python
>>> a = ['hello', 'oi']*1000000
>>> %timeit [s.upper() for s in a]
259 ms
>>> s = pandas.Series(a)
>>> %timeit s.str.upper()
488 ms
```

## Conclusion

I made Rust do something from python! I am hoping someone can tell me how I *should* have done this.
All the material is available on [github](https://github.com/martindurant/rust-cython-test), so a PR
would be perfect.

Maybe Rust is useful here, maybe not. For really solving the array-of-string processing bottleneck, I 
now thing the best solution will come from the addition of [strings to numba](http://numba.pydata.org/numba-doc/latest/release-notes.html#version-0-41-0),
which has now begun. I strongly prefer this to writing kernels in C-land, I really
want to do my development in python if I can.
If awkward-array were to become the way to compile arbitrary python functions to process
lists-of-strings in a parquet column, for example, all the better.