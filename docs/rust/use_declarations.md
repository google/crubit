# C++ bindings for Rust `use` declarations

Crubit supports `use` declarations for functions and types, mapping them to
equivalent `using` declarations in C++.

**Limitations:**

*   The `use` declaration must refer to a function or type.
*   The `use` declaration must import exactly one entity per name. For example,
    `pub use m::x;` is supported if `x` refers to a function, or to a type, but
    not if it refers to *both* a function *and* a type.

## Example

Given the following Rust crate:

```live-snippet
cs/file:examples/rust/use_declaration/example.rs content:\bpub\ use\b
```

Crubit will generate the following bindings:

```live-snippet
cs/file:examples/rust/use_declaration/example_generated.h content:\busing\b[^;]*;
```

## Name collisions {#name-collisions}

Rust keeps types and values in separate namespaces, so one module can contain
both a type alias named `Foo` and a function named `Foo`. C++ does not allow a
`using` alias to share its name with another declaration in the same namespace.
When that would happen, Crubit gives the alias a different name in C++.

Only the declarations that become C++ `using` aliases are renamed:

*   `pub use` re-exports of types and traits.
*   `pub type` aliases.

Value declarations (functions, constants, modules, struct, traits, etc) will not
be renamed.

When a conflict occurs, the alias is renamed as follows:

2.  Append an underscore: `Foo` becomes `Foo_`.
3.  If that name is taken, or if the name already ends in an underscore, appends
    a number: `Foo_1`, then `Foo_2`, etc.

For example:

| The Rust module contains                               | C++ name of the alias |
| :----------------------------------------------------- | :-------------------- |
| `pub type Foo = i32;` and `pub fn Foo()`               | `Foo_`                |
| `pub use other::Foo;` (a type) and `pub fn Foo()`      | `Foo_`                |
| `pub type FOO = i32;` and `pub const FOO: i32`         | `FOO_`                |
| `pub type Foo = i32;`, `pub fn Foo()`, `pub fn Foo_()` | `Foo_1`               |
| `pub type Foo_ = i32;` and `pub fn Foo_()`             | `Foo_1`               |

A `pub type` alias with an explicit C++ name, set with
`#[crubit_annotate::cpp_name("...")]`, is never renamed. If the explicit name
collides with another declaration, Crubit does not generate the alias, and
reports the collision in a comment in the generated header.
