<!-- <internal link> -->

# Unknown attribute errors

C++ and Rust allow the use of attributes to customize the behavior and semantics
of a piece of syntax. For example, the C++ `[[nodiscard]]` and Rust
`#[must_use]` attributes cause a compilation error if a return value is
discarded.

If Crubit encounters an attribute it is not aware of, it will fail to generate
bindings for the item that the attribute affects. This is both for reasons of
correctness, as well as forward-compatibility between Crubit releases.

## Correctness

While many attributes, like `[[nodiscard]]`, can be safely ignored, others
cannot be. For example, Crubit *must* correctly understand the
`[[no_unique_address]]`attribute to produce bindings, as it can alter the layout
of the type containing it. In Rust, Crubit *must* understand the `#[repr(...)]`
attribute, which changes type layout.

Simply ignoring these attributes would result in code which failed to compile,
or worse, which had undefined behavior at runtime.

Because Crubit does not know whether an attribute is like this in advance,
it conservatively must assume that an unknown attribute might impact
correctness, and will not generate bindings.

## Compatibility

Other attributes present a compatibility hazard. Take `[[nodiscard]]` and
`#[must_use]`. The bindings of a C++ `[[nodiscard]]` type or function should be
a Rust `#[must_use]` type or function, and <!-- disableFinding("vice versa") -->
vice versa.

If Crubit simply ignored attributes that are relevant for API surface but not
correctness, it would be a backwards-incompatible change to later stop ignoring
them.

## Fix

Crubit conservatively rejects items with unknown attributes. This can be
resolved in one of three ways:

1.  Remove the attribute from the declaration.

2.  File a crubit.rs-bug to add the attribute to Crubit.

3.  If the attribute does not impact the bindings (either their safety *or* the
    resulting API shape), add a `CRUBIT_UNSAFE_IGNORE_ATTR(...)` attribute
    containing the name of the unknown attribute.

    See support/annotations.h

    WARNING: `CRUBIT_UNSAFE_IGNORE_ATTR` can cause undefined behavior, or
    compilation failure, and can make code incompatible with future versions of
    Crubit. Only use this when the attribute has *no* impact on interface or
    semantics.
