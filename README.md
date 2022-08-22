# Rust CSV FFI - WIP
FFI bindings for my CSV library.

## About
This is a hobby project, mostly for learning how to work with FFI.
Currently only statically linking, but will be adding dynamic library later.

## Prerequisites
- Make
- GCC
- Rust Toolchain
- Valgrind

## Directory Structure
### bin
A place to hold your executables.

### examples
Examples you can run.

### include
Generated C header file (with C++ compatability).

### lib
Compiled library to be statically linked.

### src
Rust library source.

### target
Rust generated output.

## Sample Input - *./data/examples/input.csv*
```csv
Header 1, Header2, "Header 3", "Header4" , Header 5
0, 1, 2, 3, 4
5, 6, 7, 8, 9, 10, 11
1, 2, 3
```

## Building and Running
### C++
- `make example_build_cpp`
- `make example_run_cpp`
- `make example_valgrind_cpp`

## TODO
- Better documentation
- Better method to run examples (e.g. Docker containers)
- Windows support and documentation
- Shared library generation & adding bindings for other languages
