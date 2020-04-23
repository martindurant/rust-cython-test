import ctypes as ct
import numpy as np
from numpy.ctypeslib import ndpointer
import sys
sys.path.append('/Users/mdurant/code/awkward-1.0')
import awkward1

lib = ct.cdll.LoadLibrary('/Users/mdurant/code/rust-cython-test/target/release/librustcython_string.dylib')

arr = awkward1.Array(['hi' * 10] * 10000)

offsets = np.asarray(arr.layout.offsets)
indata = np.asarray(arr.layout.content).view('uint8')
outdata = np.empty_like(indata)

# UTF8, maybe variable length
lib.ak_upper.argtypes = [ndpointer(dtype='uint8'),
                         ndpointer(dtype='int64'),
                         ndpointer(dtype='uint8'),
                         ct.c_size_t,
                         ct.c_size_t]

lib.ak_upper(indata, offsets, outdata, len(offsets), len(indata))  # 3.5ms

# bytes, maybe variable length
lib.ak_bupper.argtypes = [ndpointer(dtype='uint8'),
                          ndpointer(dtype='uint8'),
                          ct.c_size_t]

lib.ak_bupper(indata, outdata, len(indata))  # 18us

# equality, utf or bytes
lib.ak_eq.argtypes = [ndpointer(dtype='uint8'),
                      ndpointer(dtype='int64'),
                      ct.c_size_t,
                      ndpointer(dtype='uint8'),
                      ndpointer(dtype='int64'),
                      ct.c_size_t,
                      ct.c_size_t,
                      ndpointer(dtype='bool')]

# 1-byte fixed width
data = np.asarray(arr, dtype="S")
np.char.upper(data)  # 1ms

# 4-byte fixed width unicode
data = np.asarray(arr, dtype="U")
np.char.upper(data)  # 2.7ms


# equality
out = np.empty(len(arr), dtype='bool')
lib.ak_eq(indata, offsets, len(indata), indata, offsets, len(indata), len(arr), out)  # 120us
