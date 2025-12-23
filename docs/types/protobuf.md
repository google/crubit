<!-- <internal link> -->

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

| Rust          | C++              |
| :------------ | :--------------- |
| `Message`     | `Message`        |
| `MessageView` | `const Message*` |
| `MessageMut`  | `Message*`       |

Protocol buffers are supported by value, and using the `View` and `Mut` view
types, where they are mapped to C++ pointers.

See
cc_bindings_from_rs/test/bridging/protobuf/rust_lib.rs
for an example definition, and
cc_bindings_from_rs/test/bridging/protobuf/user_of_rust_lib.cc
for how to call it from Rust.

## Calling C++ APIs using Protobuf message types {#cpp}

Calling C++ APIs which use protobuf is slightly more difficult.

### Passing by value

| C++       | Rust      |
| :-------- | :-------- |
| `Message` | `Message` |

When a C++ proto message is passed or returned by value, it is mapped directly
to the Rust message type, as you would expect.

### Passing by reference

| C++                                | Rust                                         |
| :--------------------------------- | :------------------------------------------- |
| `const Message*`, `const Message&` | `*const Incomplete<symbol!("Message"), ...>` |
| `Message*`, `Message&`             | `*mut Incomplete<symbol!("Message"), ...>`   |

When a C++ proto is passed by pointer or by reference, the Rust type is a
pointer to a forward declaration of the C++ protocol buffer type.

In particular, C++ APIs are **not** exposed using the `View` or `Mut` types.

These are pointers because C++ APIs do not annotate ownership, lifetime, or
aliasing properties, and so these cannot be mapped to the distinct owned,
`View`, or `Mut` types of the Rust protobuf API. And these are forward-declared
because the C++ types do not have direct Rust bindings: the generated `.proto.h`
file does not get piped through Crubit.

  * **To convert a Rust `Proto` to a C++ `const Proto*`**: use
    `my_proto.as_view().cpp_cast()`

  * **To convert a Rust `Proto` to a C++ `Proto*`**: use
    `my_proto.as_mut().cpp_cast()`

  * **To convert a C++ `(const) Proto*` to a Rust `View`/`Mut`**: use `unsafe
    {my_ptr.unsafe_cpp_cast()}`.

See support/forward_declare.rs for the definition of
`Incomplete`, `CppCast`, and `UnsafeCppCast`.
