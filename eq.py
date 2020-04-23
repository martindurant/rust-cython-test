import numba
import numpy as np
import awkward1 as ak


arr = ak.Array(['hi' * 10]*500000)
arr2 = arr.copy()


@numba.njit()
def _equal(cone, oone, ctwo, otwo):
    lengths1 = oone[1:] - oone[:-1]
    lengths2 = otwo[1:] - otwo[:-1]
    out = lengths1 == lengths2
    for i in range(len(oone) - 1):
        if out[i] is False:
            continue
        for j, k in zip(range(oone[i], oone[i+1]), range(otwo[i], otwo[i+1])):
            if cone[j] != ctwo[k]:
                out[i] = False
                break
    return out


def equal(one, two):
    cone = np.asarray(one.layout.content)
    oone = np.asarray(one.layout.offsets)
    ctwo = np.asarray(two.layout.content)
    otwo = np.asarray(two.layout.offsets)
    return ak.highlevel.Array(ak.layout.NumpyArray(_equal(cone, oone, ctwo, otwo)))


