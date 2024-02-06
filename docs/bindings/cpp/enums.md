# Rust bindings for C++ `enum`s

For the following C++ header:

```live-snippet
cs/file:examples/cpp/enum/example.h class:Color
```

Crubit will generate the following bindings:

```live-snippet
cs/file:examples/cpp/enum/example_generated.rs content:^([^/\n])([^!\n]|$)[^\n]*
```

<!-- the explanation below is based on b/208944813#comment1 -->

A C++ `enum` is translated into a set of `const` items in Rust, because this
most accurately represents the fact that C++ enumerations are non-exhaustive
(i.e. in C++ any in-range value can be cast to the enumeration, even if it
wasn't listed in the `enum` declaration). In other words, C++ behavior doesn't
match Rust `enum`s where "a discriminant in an enum not included in the type
definition" is
[listed](https://doc.rust-lang.org/reference/behavior-considered-undefined.html)
as a potential source of Undefined Behavior.
