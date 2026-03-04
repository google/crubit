# The `Delete` trait

## Quick fix

If you've encountered this error when using Crubit's `virtual_unique_ptr<T>`,
note that `virtual_unique_ptr<T>` is only available when `T` is a C++ type with
a virtual destructor or which overrides operator delete.

When `T` does not have a virtual destructor and does not override operator
delete, use `cc_std::std::unique_ptr<T>` instead.

## Why

For types with virtual destructors or custom operator delete implementations,
C++'s `delete ptr;` operation does more than simply destruct the value and
free the memory associated with the target type. Instead, C++ will use the
custom `operator delete` defined by the target type.

This makes it possible to convert a `std::unique_ptr<DerivedClass>` into a
`std::unique_ptr<BaseClass>` so long as `BaseClass` has a virtual destructor.
When the `std::unique_ptr<BaseClass>` is dropped, the `operator delete` on the
`BaseClass` will invoke the virtual destructor, resulting in a call to
the destructor of `DerivedClass`. After that, C++ knows to free memory the
size of `DerivedClass` rather than memory the size of `BaseClass`.

In order to replicate this behavior in Rust, C++'s `unique_ptr<T>` has been
split into two types: `unique_ptr` and `virtual_unique_ptr`. `unique_ptr`
assumes that the target type is concrete, while `virtual_unique_ptr` supports
C++ types with a virtual destructor or custom `operator delete`. Such types
will automatically receive a Crubit-generated implementation of the `Delete`
trait.
