# C++/Rust Protobuf interop

WARNING: This page documents functionality that is currently internal to the
Google monorepo.

Protocol buffers are Google's language-neutral, platform-neutral, extensible
mechanism for serializing structured data. Once you define how you want your
data to be structured once, you can generate source code in a variety of
languages to manipulate and serialize/deserialize your structured data. Protobuf
messages are among the most common types at Google, appearing in vast majority
of APIs.

The usual way to passing data from one language to another using Protobufs is to
serialize a message in one language, and deserialize it in another. This
serialization/deserialization has costs which makes this approach unsuitable for
hot code paths.

To avoid those costs, we've intentionally designed C++ and Rust Protobuf message
types to have identical layouts. We avoid the need for
serialization/deserialization and instead we directly use the same message
object from both languages. Crubit automatically generates the zero-cost glue
code for us. For example, take this piece of a C++ header:

```c++
MyProto Foo();
```

This becomes available to Rust as:

```rust
pub fn Foo() -> MyProto {...}
```

(Specifically, Crubit will detect that this is a Protobuf message, and it will
convert from the C++ message type to the Rust message type.)

## Calling Rust APIs using Protobuf message types {#rust}

| Rust                          | C++                                 |
| :---------------------------- | :---------------------------------- |
| `Message`                     | `Message`                           |
| `MessageView`                 | `const Message*`                    |
| `MessageMut`                  | `Message*`                          |
| `&Message`                    | `const proto::Rust<Message>&`       |
| `*const Message`              | `const proto::Rust<Message>*`       |
| `Message` (as a struct field) | `proto::Rust<Message>`              |
| `Vec<Message>`                | `rs_std::Vec<proto::Rust<Message>>` |

Protocol buffers are supported by value, and using the `View` and `Mut` view
types, where they are mapped to C++ pointers.

When an owned Rust `Message` appears in a layout-compatible position (such as a
public struct field, behind a reference or pointer, or inside a generic
container like `Vec`), it maps to `proto::Rust<Message>` in C++, a non-null
handle providing `get()`, `operator->`, and `operator*` access to the underlying
C++ `Message`.

## Calling C++ APIs using Protobuf message types {#cpp}

### Passing by value

| C++       | Rust      |
| :-------- | :-------- |
| `Message` | `Message` |

When a C++ proto message is passed or returned by value, it is mapped directly
to the Rust message type, as you would expect.

### Passing by reference

| C++              | Rust          |
| :--------------- | :------------ |
| `const Message&` | `MessageView` |
| `Message&`       | `MessageMut`  |

C++ protobuf references map directly to safe Rust `MessageView` and `MessageMut`
types. This relies on lifetime annotations (`assume_lifetimes`) and
`//features:proto_references`, both of which are enabled by
default, so no extra configuration is needed beyond the usual
`rust_api_from_cpp` setup:

```starlark
load("//bazel:rust_api_from_cpp.bzl", "rust_api_from_cpp")

cc_library(
    name = "my_cpp_lib",
    hdrs = ["my_cpp_lib.h"],
    aspect_hints = [":my_cpp_lib_rust.hint"],
    deps = [":my_cc_proto"],
)

rust_api_from_cpp(
    name = "my_cpp_lib_rust",
    cpp_target = ":my_cpp_lib",
)
```

No `cpp_coerce()` (or deprecated `cpp_cast()`) or `unsafe` block is required
when calling C++ functions that take or return protobuf references:

```rust
// Pass an owned Rust proto using .as_view() / .as_mut(),
// or pass an existing MessageView / MessageMut directly:
my_cpp_lib::ProcessConstRef(my_proto.as_view());
my_cpp_lib::ProcessMutRef(my_proto.as_mut());
```

*(If `proto_references` is not enabled on a target with `assume_lifetimes`, C++
`const Message&` maps to `&Incomplete<symbol!("Message"), ...>` and `Message&`
maps to `Pin<&mut Incomplete<symbol!("Message"), ...>>`. In that case, you
should enable `proto_references` on the target.)*

### Passing by pointer

| C++              | Rust                                         |
| :--------------- | :------------------------------------------- |
| `const Message*` | `*const Incomplete<symbol!("Message"), ...>` |
| `Message*`       | `*mut Incomplete<symbol!("Message"), ...>`   |

When a C++ proto is passed or returned by raw pointer (or by reference on legacy
`no_assume_lifetimes` targets), the Rust type is a raw pointer to a forward
declaration (`Incomplete<...>`) of the C++ protocol buffer type. Raw pointers
cannot be mapped to `MessageView` or `MessageMut` because C++ pointers may be
null or lack lifetime guarantees, and the generated `.proto.h` header is not
processed directly by Crubit.

To convert between Rust protobuf view types and C++ `Incomplete` pointers, use
the `CppCoerce` (`cpp_coerce()`) and `UnsafeCppCoerce` (`unsafe_cpp_coerce()`)
traits from `forward_declare` (which replace the deprecated `cpp_cast()` and
`unsafe_cpp_cast()` methods):

*   **To convert a Rust `Proto` (or `ProtoView`) to a C++ `const Proto*`**: use
    `my_proto.as_view().cpp_coerce()` (or `my_view.cpp_coerce()`)

*   **To convert a Rust `Proto` (or `ProtoMut`) to a C++ `Proto*`**: use
    `my_proto.as_mut().cpp_coerce()` (or `my_mut.cpp_coerce()`)

*   **To convert a C++ `(const) Proto*` to a Rust `View`/`Mut`**: bind the
    pointer to a local variable and use `unsafe { my_ptr.unsafe_cpp_coerce() }`.

See support/forward_declare.rs for the definition of
`Incomplete`, `CppCoerce`, and `UnsafeCppCoerce`.
