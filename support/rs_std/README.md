# Crubit `rs_std` library

This directory contains the `rs_std` C++ library that provides the following
APIs: - Manually authored APIs that help work with Rust builtin types. For
example, `rs_std::char` represents Rust's `char` type (a separate type from
C++'s `char32_t` is needed to detect certain invalid bit patterns that result in
Undefined Behavior in Rust; additionally `char32_t` takes at least 32 bits,
rather than exactly 32 bits). - (Not yet implemented) Automatically generated
C++ bindings for Rust standard library.

## Versioning

Changes to this directory take effect instantly. To make version-dependent
changes, pair them with a feature flag in
`crubit/support/internal/cpp_config.h`, and guard with an `#ifdef`.

For example, consider `char`: a new pending release might, say, change it to be
64 bits wide. This can be released as so:

```c++
// rs_std/char.h
#include "crubit/support/internal/cpp_config.h"
#ifdef CRUBIT_INTERNAL_VERY_WIDE_CHAR
std::uint64_t value_ = '\0';
#else
std::uint32_t value_ = '\0';
#endif
```

```c++
// cpp_config.h
...
#define CRUBIT_INTERNAL_VERY_WIDE_CHAR
...
```

The old release, which has the *old* copy of `cpp_config.h`, will not define
`CRUBIT_INTERNAL_VERY_WIDE_CHAR`, and will use the previous layout.

Crubit when built at head, and for the next release, will use the *new* copy of
`cpp_config.h`, which defines the macro, and obtains the new struct definition.

Even though the source code is live-at-head, changes like this will be
release-gated through the crate features, which are not.
