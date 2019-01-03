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
