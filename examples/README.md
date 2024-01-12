# Examples of using Crubit

The `examples/` directory contains copy-pastable example snippets.
`examples/cpp/` contains example C++ header files, and how to call them from
Rust -- for example, `examples/cpp/function` shows how to call a C++ function
from Rust. `examples/rust` contains example Rust modules, and how to call them
from C++.

In every case, examples have the following files:

1.  The original Rust/C++ file that will be used, called either `example.h` or
    `example.rs`
2.  An example usage of that file via FFI using Crubit, called either `main.rs`
    or `main.cc`.
