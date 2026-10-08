# Unknown target errors

When generating Rust bindings for a C++ library, Crubit needs to know which
Bazel target owns every header that defines a type used in the library's API.
That is how it decides, for example, that `absl::Span<T>` maps to the `Span`
type from the Rust bindings for `@abseil-cpp//absl/types:span`, or which
generated crate a struct should be imported from.

Crubit only knows about the public headers of **Crubit-enabled** targets that
your library depends on **directly** (plus those reachable through header-only
or Crubit-enabled intermediate libraries). If a type's header isn't one of
those, Crubit reports an error like:

```
crubit.rs/errors/unknown_target: Failed to complete template specialization type absl::Span<const std::string>:
the template is defined in
  third_party/absl/types/span.h
which is not a public header of any Crubit-enabled target that
  //my:lib
depends on directly. ...
```

or, for non-template types:

```
crubit.rs/errors/unknown_target: the type is defined in
  foo/bar.h
which is not a public header of any Crubit-enabled target that
  //my:lib
depends on directly. ...
```

Crubit can't tell which of the following causes applies, so check each.

## Cause 1: Crubit is not enabled on the defining library {#not-enabled}

The library that provides the header exists in your `deps`, but doesn't have
Crubit enabled. Enable Crubit on it (see [Enabling
Crubit](../cpp/index.md#enable)). If you can't, see [Unsupported
types](unsupported_type.md) for workarounds.

## Cause 2: the defining library is not a direct dependency {#transitive}

To keep builds fast, Crubit does not analyze every transitive dependency of a
C++ library. In particular, it does **not** look behind a `cc_library` that has
compiled sources (`.cc` files) and no Crubit `aspect_hints`. If your header uses
a type whose header is only reachable through such a library, Crubit can't
attribute the header to a target, even if the defining library has Crubit
enabled.

This is the same rule as
[include-what-you-use](https://google.github.io/styleguide/cppguide.html#Include_What_You_Use)
and `layering_check`: a header that names a type should `#include` the header
that defines it, and the target should depend directly on the library that
provides that header.

For example, the following fails even though `@abseil-cpp//absl/types:span` has
Crubit enabled:

```build {.bad}
cc_library(
    name = "middle",
    srcs = ["middle.cc"],
    hdrs = ["middle.h"],  # #includes "third_party/absl/types/span.h"
    deps = ["@abseil-cpp//absl/types:span"],
)

cc_library(
    name = "my_lib",
    hdrs = ["my_lib.h"],  # uses absl::Span, but only #includes "middle.h"
    aspect_hints = [":my_lib_rust.hint"],
    deps = [":middle"],  # :span is not a direct dependency
)
```

Fix it by adding a direct dependency on the library that provides the header,
and `#include`-ing the header directly:

```build {.good}
cc_library(
    name = "my_lib",
    hdrs = ["my_lib.h"],  # #includes "third_party/absl/types/span.h"
    aspect_hints = [":my_lib_rust.hint"],
    deps = [
        ":middle",
        "@abseil-cpp//absl/types:span",
    ],
)
```

## Cause 3: the header is private {#private}

Headers listed in `srcs` (rather than `hdrs`) of a `cc_library` are private to
that library, and Crubit does not attribute them to any target. Types defined in
private headers can't receive bindings. If you need bindings for such a type,
move its definition to a public header (in `hdrs`), or wrap it in a public type.
See [Unsupported types](unsupported_type.md#fix-wrapper).
