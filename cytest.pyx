from cpython cimport PyUnicode_AsUTF8String, PyUnicode_InternFromString
cdef extern from "string.h":
    void *memcpy(void *dest, void *src, size_t n)
cdef extern from "stdio.h":
    int toupper(int c)
import numpy as np
cimport numpy as np
import cython


cdef extern from "rlib.h":
    char *my_upper(char *b)
    int init()


def call_init():
    return bool(init())


def call_rust_upper(unicode s):
    cdef unicode out
    return PyUnicode_InternFromString(my_upper(PyUnicode_AsUTF8String(s)))


@cython.boundscheck(False) # turn off bounds-checking for entire function
@cython.wraparound(False)  # turn off negative index wrapping for entire function
cdef do_array_upper(np.ndarray[np.int64_t, ndim=1] starts,
                    np.ndarray[np.int64_t, ndim=1] stops,
                    np.ndarray[np.uint8_t, ndim=1] content,
                    np.ndarray[np.uint8_t, ndim=1] out):
    cdef char* word
    cdef char* uword
    cdef int start, stop
    assert content.flags['C_CONTIGUOUS']
    assert out.flags['C_CONTIGUOUS']
    l = len(starts)
    cdef int i = 0
    while i < l:
        i += 1
        start = starts[i]
        stop = stops[i]
        word = <char*> (<void*> (content.data + start))
        uword = my_upper(word)
        memcpy((<void*> out.data) + start, <void *> uword, stop - start)


@cython.boundscheck(False) # turn off bounds-checking for entire function
@cython.wraparound(False)  # turn off negative index wrapping for entire function
cdef do_carray_upper(np.ndarray[np.int64_t, ndim=1] starts,
                     np.ndarray[np.int64_t, ndim=1] stops,
                     np.ndarray[np.uint8_t, ndim=1] content,
                     np.ndarray[np.uint8_t, ndim=1] out):
    cdef char* word
    cdef int start, stop
    cdef int l = starts.shape[0]
    cdef int i, j = 0
    while i < l:
        start = starts[i]
        stop = stops[i]
        word = <char*> (<void*> (content.data + start))
        while start <= stop:
            out.data[start] = <char>toupper(<int>content.data[start])
            start += 1
        i += 1


def array_upper(arr):
    out = arr.empty_like()
    #do_array_upper(arr.starts, arr.stops, arr.content, out.content)
    do_carray_upper(arr.starts, arr.stops, arr.content, out.content)
    return out