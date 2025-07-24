# Crubit `rs_std` library

This directory contains the `rs_std` C++ library that provides the following
APIs: - Manually authored APIs that help work with Rust builtin types. For
example, `rs_std::char` represents Rust's `char` type (a separate type from
C++'s `char32_t` is needed to detect certain invalid bit patterns that result in
Undefined Behavior in Rust; additionally `char32_t` takes at least 32 bits,
rather than exactly 32 bits). - (Not yet implemented) Automatically generated
C++ bindings for Rust standard library.
