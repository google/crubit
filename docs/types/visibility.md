# Type visibility

In Crubit's `:wrapper` mode, `pub(crate)` types can be generated, which are
restricted to a specific library. This is generally a temporary state of
affairs: as a way of enabling types to be used for a specific library, without
exposing them everywhere, if their bindings are flawed or need work.

## Visibility errors

If the generated bindings for a type are `pub(crate)`, then bindings will not be
generated when the type is used outside of that library. For example, consider
the following library, which uses `:wrapper` mode:

```c++
struct WEIRD_EXPERIMENTAL_ATTRIBUTE SomeType {};
void Foo(SomeType);
```

If `SomeType` is `pub(crate)` because of its use of
`WEIRD_EXPERIMENTAL_ATTRIBUTE`, then functions, class members, constants, etc.
which use that type will only receive bindings in the same crate, and those
bindings will themselves be `pub(crate)`:

```rust
pub(crate) struct SomeType { ... }
pub(crate) fn Foo(...: SomeType) { ... }
```

If a different library uses the type, and defines a similar function `Bar`, then
it will not receive bindings at all, because the bindings for `Bar` are only
visible in the library where it was defined.

```c++
void Bar(SomeType);  // won't receive bindings: it's in a another library
```

This can dramatically reduce the set of bindings which are generated, and it is
for this reason that these `pub(crate)` type bindings are only used sparingly,
typically for early release of features that cannot yet be globally supported.
You should not rely on the `pub(crate)` status of a type!

### Fix

To work around this, you can wrap or hide the type as it is used in the public
API. For example, if you needed to accept a pointer to `X`, but `X` is
`pub(crate)`, you can accept a `void*` instead.
