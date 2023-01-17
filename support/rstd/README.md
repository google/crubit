# Crubit `rstd` library

Disclaimer: This project is experimental, under heavy development, and should
not be used yet.

This directory contains C++ libraries that help work with Rust types.  For
example, `rstd::Char` represents Rust's `char` type (a separate type from
C++'s `char32_t` is needed to detect certain invalid bit patterns
result in Undefined Behavior in Rust;  additionally `char32_t` takes
at least 32 bits, rather than exactly 32 bits).
