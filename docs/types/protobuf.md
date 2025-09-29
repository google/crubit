<!-- <internal link> -->

# C++/Rust Protobuf interop

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
types to be ABI compatible. We avoid the need for serialization/deserialization
and instead we directly use the same message object from both languages. Crubit
automatically generates the zero-cost glue code for us. For example, take this
piece of a C++ header:

```c++
MyProto Foo(); 
```

This becomes available to Rust as:

```rust
pub fn Foo() -> MyProto {...}
```

(Specifically, Crubit will detect that this is a Protobuf message, and it will
convert from C++ message type to Rust message type.)

## Calling C++ APIs using Protobuf message types {#cpp}

TODO: more narrative documentation, inline example code.

First of all, add your `proto_library` target to the
[allowlist](http://<internal link>). See b/414381884 for more context
& information on when this allowlist will be removed.

<!-- Need to submit examples first, then docs, to get working previews. -->

See google_internal/protobuf/BUILD for examples of
calling various shapes of C++ APIs from Rust.

## Calling Rust APIs using Protobuf message types {#rust}

TODO: more narrative documentation, inline example code.

<!-- Need to submit examples first, then docs, to get working previews. -->

See
cc_bindings_from_rs/test/bridging/protobuf/rust_lib.rs
for an example definition, and
cc_bindings_from_rs/test/bridging/protobuf/user_of_rust_lib.cc
for how to call it from Rust.
