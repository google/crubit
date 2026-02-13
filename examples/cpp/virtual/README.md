# Overriding virtual methods in Rust

This directory contains a minimal example of overriding a virtual method, while
calling a different method on the base class.

The overall process uses two directions of interop, and looks something like:

1.  Define a base class (here `ExampleBase`), with some virtual methods.

    Here, this is defined in `base.h`.

2.  In Rust, define a concrete type which can implement those methods, and
    potentially receives a pointer to `ExampleBase` for calling back into it.

    Here, this is defined in `definition.rs`.

3.  In C++, define a subclass of `ExampleBase` which contains the Rust type, and
    calls back into it.

    Here, this is defined in `example.h`.

This is a bit of a worst-case fallback: for arbitrarily complex features, with
no current stable Crubit support, one can write some glue code in C++ that
bridges the gap. Virtual dispatch is one of the most complicated cases, as it
inherently involves bidirectional interop.

(A better story and automation is planned for this, but not before 2028.)
