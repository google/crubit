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

### Passing by pointer

| C++              | Rust                                         |
| :--------------- | :------------------------------------------- |
| `const Message*` | `*const Incomplete<symbol!("Message"), ...>` |
| `Message*`       | `*mut Incomplete<symbol!("Message"), ...>`   |

When a C++ proto is passed by pointer, the Rust type is a pointer to a forward
declaration of the C++ protocol buffer type.

*   **To convert a Rust `Proto` to a C++ `const Proto*`**: use
    `my_proto.as_view().cpp_cast()`

*   **To convert a Rust `Proto` to a C++ `Proto*`**: use
    `my_proto.as_mut().cpp_cast()`

*   **To convert a C++ `(const) Proto*` to a Rust `View`/`Mut`**: use `unsafe
    {my_ptr.unsafe_cpp_cast()}`.

### Passing by reference

| C++              | Rust                                            |
| :--------------- | :---------------------------------------------- |
| `const Message&` | `&Incomplete<symbol!("Message"), ...>`          |
| `Message&`       | `Pin<&mut Incomplete<symbol!("Message"), ...>>` |

When a C++ proto is passed by reference, Crubit uses lifetime inference
(`assume_lifetimes`, enabled by default in
`//features:supported`) to map the C++ reference to a Rust
reference to a forward declaration of the C++ protocol buffer type. Note that
because incomplete types are `!Unpin`, non-const references are wrapped in
`Pin<&mut ...>`.

Because `cpp_cast()` currently produces a raw pointer (`*const Incomplete` or
`*mut Incomplete`), to call a C++ API expecting `&Incomplete` or `Pin<&mut
Incomplete>`, you must dereference the pointer in an `unsafe` block:

*   **To convert a Rust `Proto` to a C++ `const Proto&`**: use
    `unsafe { &*my_proto.as_view().cpp_cast() }`

*   **To convert a Rust `Proto` to a C++ `Proto&`**: use
    `unsafe { Pin::new_unchecked(&mut *my_proto.as_mut().cpp_cast()) }`

*   **To convert a C++ `const Proto&` (`&Incomplete`) to a Rust `View`**: obtain a
    pointer with `std::ptr::from_ref` and call `unsafe_cpp_cast`:
    ```rust
    let ptr = std::ptr::from_ref(my_ref);
    let view: ProtoView = unsafe { ptr.unsafe_cpp_cast() };
    ```

*   **To convert a C++ `Proto&` (`Pin<&mut Incomplete>`) to a Rust `Mut`**:
    unpin the reference, obtain a mutable pointer with `std::ptr::from_mut`, and
    call `unsafe_cpp_cast` on `&mut ptr`:
    ```rust
    let mut ptr = std::ptr::from_mut(unsafe { Pin::into_inner_unchecked(my_pin) });
    let mut_msg: ProtoMut = unsafe { (&mut ptr).unsafe_cpp_cast() };
    ```

#### Returning references from C++

When a C++ API returns a reference:

*   **Tied to an input lifetime**: Crubit infers the lifetime and returns
    `::cref::CRef<'a, Incomplete>` for `const Message&` or
    `::cref::CMut<'a, Incomplete>` for `Message&`.
    *   To convert `CRef` to `ProtoView`:
        ```rust
        let ptr = CRef::as_ptr(cref);
        let view: ProtoView = unsafe { ptr.unsafe_cpp_cast() };
        ```
    *   To convert `CMut` to `ProtoMut`:
        ```rust
        let mut ptr = CMut::as_mut_ptr(cmut);
        let mut_msg: ProtoMut = unsafe { (&mut ptr).unsafe_cpp_cast() };
        ```
*   **Static or unbound reference**: If the return reference cannot be tied to an
    input parameter's lifetime, Crubit falls back to returning raw pointers
    (`*const Incomplete` and `*mut Incomplete`).

See support/forward_declare.rs for the definition of
`Incomplete`, `CppCast`, and `UnsafeCppCast`, and
support/cref.rs for `CRef` and `CMut`.
