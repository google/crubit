# Crubit Features

> SUMMARY: To enable Crubit on a C++ target and make it usable from Rust, use
> `cc_library(..., aspect_hints=["//features:supported"])`.
> No additional work is necessary for Rust targets to make them usable from C++.
>
> The other features are more rarely useful, and mostly form a testing or
> release staging area for making changes to `:supported`.

C++/Rust interop is configured per target using "features": aspect hints which
are applied onto a build target in order to configure its automatically
generated bindings.

Features control which capabilities Crubit has. For example, support for move
constructors is gated behind the `:experimental` feature. If the class is in a
target configured to use `:experimental`, then the constructor will receive
bindings, otherwise it will not.

Features are also used for gradual rollouts of new capabilities, and to allow
for separate build modes for un-annotated code, code which opts into C++/Rust
interop, and internal test code.

## Using Features

To use a feature, add it to your target:

```python
# Enables the `:supported` feature on a C++ library `:example`.
cc_library(
    name = "example",
    hdrs = ["example.h"],
    aspect_hints = ["//features:supported"],
)
```

This causes the generated bindings for that target to use all the Crubit
capabilities in `:supported`.

## Features

### \<default\> {#default}

By default, Rust targets enable Crubit, and C++ targets do not.

Bindings are only generated for code without feature hints if this has minimal
or generally-agreed upon impositions on library owners.

Rust targets are well behaved, and are expected to support C++ callers and
integrate into the existing C++ codebase. For that reason, Crubit is
automatically enabled on all Rust targets. See crubit.rs/rust.

However, by default, Crubit does not run at all on C++ targets. On a
sufficiently "strange" target, Crubit can fail arbitrarily badly. As a
real-world example, some libraries in third_party have headers that do not even
parse, and which disable the header parsing check. See e.g. b/348557947. Crubit
cannot run on these without manual intervention.

In addition, unexpected Rust callers could frustrate C++ library owners, as Rust
(even via Crubit) constrains the C++ API changes users can make. (For example,
changing a type from trivial to nontrivial is potentially mostly nonbreaking in
C++, but a very major API break in Rust.) So, even if the technical ability were
there, enabling Crubit by default would still require conversation and
cooperation with the C++ ecosystem, and the resulting compromise might be
short of what you get with `:supported`.

### `"//features:supported"` {#supported}

The `supported` feature enables generally-available C++/Rust interop
capabilities, for libraries that specifically support FFI callers.

See crubit.rs/cpp for documentation on what this includes.

This is a no-op on Rust targets.

### `"//features:wrapper"` {#wrapper}

> WARNING: `wrapper` produces highly unstable bindings, and is by allowlist
> only, as it can and will break during crubit releases and when other libraries
> enable Crubit.
>
> The intention is that this is mainly for use in high-touch core libraries, in
> partnership with the Rust team.

The `wrapper` feature enables "comprehensive fallbacks" for wrapper libraries.
Instead of failing to generate bindings for a function if it uses types that it
doesn't know about, we replace them with the moral equivalent of a `void*` and
keep going, allowing at least *something* usable to be generated.

This also enables bindings for features that are not yet in `:supported`, for
early adoption (e.g. forward declarations), and for templates, which are not
currently implemented in a way that is usable broadly.

When bindings are generated for `:wrapper` that are not generated for
`:supported`, they are marked `pub(crate)` and only usable with
`additional_rust_srcs_for_crubit_bindings`.

See crubit.rs-comprehensive-fallbacks for more detail.

As a result of all of these, the `:wrapper` feature is very unstable. For
instance, if `absl::Cord` does not enable Crubit, or if Crubit is missing a
feature necessary to generate bindings for the type, then `Cord` pointers will
become opaque pointers: a `const absl::Cord*` parameter in C++ might be `*const
Incomplete<symbol!("absl::Cord"), ()>` in Rust. Then if `Cord` enables Crubit,
it becomes `*const absl::Cord` as you'd expect.

This is the only place where adding support for more features and more types is
a backwards-compatibility break. In order to keep the compatibility hazards
scaling at O(1), the breakage-prone APIs are `pub(crate)`, and `:wrapper` is
gated behind an allowlist. The primary intended user of the feature is the Rust
team itself, as part of producing bindings for high-touch core libraries.

### `"//features:experimental"` {#experimental}

WARNING: `experimental` has known compilation errors when used (e.g.
b/439435837), and is by allowlist only. As a general rule, you should prefer to
deploy a workaround rather than use `experimental`.

<!-- TODO(b/439435837): update to different bug link once fixed, or else change
to warn that it's easily broken / fragile and may have bugs. -->

The `experimental` feature enables internal-only experimental capabilities. This
is used for testing of upcoming features, and is also used by pilot partners who
are testing those features. This also includes everything in `:supported`.

`experimental` is visibility-restricted to only internal use and pilot partners.

### Other features {#other}

The other features float over time, as new features are taken from `:wrapper` or
`:experimental` and moved into a separate, individual feature, and then released
as part of `:supported` in a Bazel configuration change.

See crubit.rs/team/release for why features are released this way.

## How-to: Testing Features {#testing}

Crubit developers may want to temporarily enable features without respect to
visibility restrictions or level of support. The build flag:

`--//common/bazel_support:globally_enabled_features`

accepts a comma-separated list of unstable internal feature names (like
`supported`, `experimental`, or the special alias `all`) and overrides the
baseline feature set for all targets for the duration of the build. The flag's
default value is the empty list.
