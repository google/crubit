# C++ bindings for Rust traits

Rust traits and trait implementations are mapped into a template struct and
template struct specialization respectively.

Today only a subset of trait functionality is supported:

*   Generic traits are not supported.
*   Blanket Implementations are not supported.
*   Only associated functions of the trait receive bindings today.
*   The same constraints required for a top-level function to receive bindings
    apply to trait functions (each parameter and return type receives bindings
    etc.).

## Example.

Given the following Rust crate:

```
{{ #include ../../examples/rust/trait/example.rs }}
```
<!--  -->


Crubit will generate three bindings. A template struct for `MyTrait`:

```
{{ #include ../../examples/rust/trait/example_generated.h }}
```
<!--  class:MyTrait -->


A struct for `MyStruct`:

```
{{ #include ../../examples/rust/trait/example_generated.h }}
```
<!--  class:MyStruct -->


A template specialization for `impl MyTrait for MyStruct`:

```
{{ #include ../../examples/rust/trait/example_generated.h }}
```
<!--  class:impl -->


We can see an example of how we call our generated trait methods:

```
{{ #include ../../examples/rust/trait/main.cc }}
```
<!--  function:main -->


Each bound trait provides its implementations via the `impl` member, which
selects the generated specialization (if one exists). We can call our trait
method using the `Trait::impl<Args...>::trait_method(args...)` syntax mirroring
Rust's fully qualified method dispatch.

## What about when a trait isn't implemented?

Our example so far, rather conveniently, only calls trait implementations that
exist. But what about when a trait implementation doesn't exist? We're just as
free to write `MyTrait::impl<uint32_t>::add_with(...)` despite there being no
`impl MyTrait for i32` in Rust.

In such a situation, we generate a fresh instantiation of our `MyTrait` template
devoid of any methods. This will cause a compilation error. If we're writing our
own templated code and we want to check that our a template argument `T`
implements a trait, we can use `rs_std::where`.

We can write a templated function that requires our trait implementation:

```cpp
template <typename T>
  requires(rs_std::where_v<T, example_crate::MyTrait>)
uint32_t add_with_2(T const& self) {
  return example_crate::MyTrait::impl<T>::add_with(self, 2);
}
```

A constexpr expression is also available as `rs_std::where_v`.
