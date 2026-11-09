# Rust bindings for C++ static data members

Public static data members of a class receive bindings. Which kind depends on
the member's type and whether its value is a compile-time constant:

*   A member of integer, `bool`, C array of integer or `bool`, or
    `std::string_view`/`absl::string_view` type whose value is a compile-time
    constant (because it is `constexpr`, or `const` or `inline` with a constant
    initializer) becomes an associated `const` of the Rust struct. Its value is
    copied into the bindings, so Rust doesn't need to access the C++ object.
*   A member whose value is not a compile-time constant (for example, one that
    is only declared in the class and is defined in a `.cc` file) becomes a Rust
    `static` that refers to the C++ object, because its value is only known at
    run time. Rust has no associated statics, so it is placed in the snake-cased
    module that holds the class's other nested items. If it is mutable, it is a
    `static mut`, and, as with other C++ globals, accessing it is `unsafe`.

For example, given:

```c++
struct Counter {
  static constexpr int kMax = 10;
  static const bool kEnabled = true;
  static int count;  // Defined in counter.cc.
};
```

Crubit generates (simplified):

```rust
impl Counter {
    pub const kMax: ffi_11::c_int = ffi_11::new_c_int(10);
    pub const kEnabled: bool = true;
}

pub mod counter {
    unsafe extern "C" {
        pub static mut count: ffi_11::c_int;
    }
}
```

A member that is only declared in the class, and defined `constexpr` after it in
the header, is treated the same as one that is initialized in the class:

```c++
struct Limits {
  static const int kMax;
};
// `Limits::kMax` becomes a `const` in Rust.
inline constexpr int Limits::kMax = 10;
```

Some constants can only be written this way: a constant of the class's own type
(such as `static const Limits kDefault;`) can't be initialized in the class,
because the class is incomplete there.

Any other out-of-line definition (for example, a non-`constexpr` one, or one in
a `.cc` file) leaves the member a `static` that refers to the C++ object.

Compile-time constants of other types, such as floating-point or class types,
and static data members of class template specializations don't receive bindings
yet.
