# C++ bindings for Rust function pointer types

When used as function parameter types or function return types, Rust function
pointer types map into the corresponding C++ types as follows:

Rust                         | C++
---------------------------- | --------------------------------------
`extern "C" fn(i32)`         | `std::type_identity_t<void(int32_t)>&`
`Option<extern "C" fn(i32)>` | Not supported

In other scenarios Rust function pointer types map into the corresponding C++
types as follows:

Rust                         | C++
---------------------------- | --------------------------------------
`extern "C" fn(i32)`         | `std::type_identity_t<void(int32_t)>*`
`Option<extern "C" fn(i32)>` | Not supported

TODO: Link to the not-yet-written `references.md` with a longer explanation why
Rust references are sometimes bound to C++ references and sometimes to C++
pointers (C++ references can only be bound once; C++ temporaries).
