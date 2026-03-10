# C++ bindings for Rust traits

Crubit allows C++ callers to invoke Rust trait implementations. For each `trait
SomeTrait`, Crubit generates a matching C++ `struct MyTrait`. To call a trait
method `SomeTrait::some_fn` implemented by `SomeType`, C++ callers can use
`SomeTrait::impl<SomeType>::some_fn(args...)`.

## Example

Given the following Rust crate:

```
{{ #include ../../examples/rust/trait/example.rs }}
```
<!--  -->


You can call the trait from C++ using the following code:

```
{{ #include ../../examples/rust/trait/main.cc }}
```
<!--  function:main -->


Each trait has an associated `impl` member which is generic upon the `Self` type
of the trait implementation, and which provides all of the associated types,
consts, and functions.

## Checking if a trait is implemented

C++ users can check if a trait implementation is available using
`rs_std::where_v<T, SomeTrait>`. This is useful when writing templated C++
functions, as it can be used with `requires` (or `enable_if`) to specify that a
particular trait is implemented by a type parameter. For example:

```
{{ #include ../../examples/rust/trait/main.cc }}
```
<!--  function:add_with_two -->


## Limitations

Some trait implementations will not receive bindings:

*   Trait implementations with generic parameters (e.g. `impl<T> ...`) will not
    receive bindings.
    *   For example, an implementation `impl<U> TwoArgs<i32, U> for MyStruct`
        will not receive bindings because the `impl` has a type parameter (`U`).
    *   This also means that blanket impls (e.g. `impl<T> Trait for T`) are not
        supported.
*   Traits with `const` parameters (e.g. `trait T<const V: usize>`) do not yet
    not receive bindings.
*   Trait methods will not receive bindings if their parameter or return types
    are not yet supported by Crubit.

## How does it work?

Under the hood, Crubit will generate three bindings from the example Rust crate.
A template specialization for `impl MyTrait for MyStruct`:

```
{{ #include ../../examples/rust/trait/example_generated.h }}
```
<!--  class:impl -->


Our generated `rs_std::impl` specialization is an implementation detail of
binding generation. It holds the actual thunks that call into rust to implement
our trait, but they should be accessed through our template struct `MyTrait`:

```
{{ #include ../../examples/rust/trait/example_generated.h }}
```
<!--  class:MyTrait -->


Each struct generated for a Rust trait has the `impl` member that provides
access to the generated `rs_std::impl`. This is the preferred way to reference
the generated `rs_std::impl`.

Finally, a struct for `MyStruct`:

```
{{ #include ../../examples/rust/trait/example_generated.h }}
```
<!--  class:MyStruct -->


Our struct is generated normally and exists purely so we have something to
implement our trait for.
