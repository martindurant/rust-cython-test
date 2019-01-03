from setuptools import setup, Extension

from Cython.Build import cythonize
from Cython.Distutils import build_ext
import numpy

ext = Extension(
    'cytest',
    sources=['cytest.pyx'],
    libraries=['rustcython_string'],
    library_dirs=['target/release'],
    include_dirs=['.']
)

extensions = [ext]

setup(
    name="cytest",
    ext_modules=cythonize(extensions, annotate=True),
    cmdclass={'build_ext': build_ext},
    include_dirs=[numpy.get_include()]
)

