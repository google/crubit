# Rust bindings for C++ operators and special member functions

Here we describe how Crubit bindings work with C++ special member functions and
operator overloading (e.g. the copy constructor, or `operator==`) and with
traits from the Rust standard library (e.g., the `Clone` or `PartialEq` traits).

Rust traits which return `Self` by value (e.g., `Default` or `Clone`) or use
mutable references (e.g., `AddAssign`) are only implemented in Rust bindings for
a given C++ type if that type is [is `Unpin`](../../unpin.md).

## Bidirectional map

The following special member functions and traits are mapped bidirectionally:

C++                          | Rust
---------------------------- | ---------
Default constructor          | `Default`
Trivial copy constructor     | `Copy`
Non-trivial copy constructor | `Clone`
Destructor                   | `Drop`

Rust bindings for a C++ type only implement `Copy` if the C++ type is
non-abstract and has a public, trivial copy constructor and destructor.

## One-way map of C++ special member functions into Rust traits

If the C++ type [is `Unpin`](../../unpin.md), then the C++ special member
functions below are mapped one-way into the corresponding Rust traits as
follows:

C++ API                                   | Rust bindings
----------------------------------------- | -------------
Constructor taking a single `T` parameter | `From<T>`

This applies regardless of whether the constructor is `explicit` in the C++ API.

The C++ binary operators below are mapped one-way into the corresponding Rust
traits as follows:

C++ API       | Rust bindings
------------- | --------------
`operator==`  | `PartialEq`
`operator<`   | `PartialOrd`
`operator>`   | `PartialOrd`
`operator<=`  | `PartialOrd`
`operator>=`  | `PartialOrd`
`operator+`   | `Add`
`operator-`   | `Sub`
`operator*`   | `Mul`
`operator/`   | `Div`
`operator%`   | `Rem`
`operator&`   | `BitAnd`
`operator\|`  | `BitOr`
`operator^`   | `BitXor`
`operator<<`  | `Shl`
`operator>>`  | `Shr`
`operator+=`  | `AddAssign`
`operator-=`  | `SubAssign`
`operator*=`  | `MulAssign`
`operator/=`  | `DivAssign`
`operator%=`  | `RemAssign`
`operator&=`  | `BitAndAssign`
`operator\|=` | `BitOrAssign`
`operator^=`  | `BitXorAssign`
`operator<<=` | `ShlAssign`
`operator>>=` | `ShrAssign`

The C++ unary operators below are mapped one-way into the corresponding Rust
traits as follows:

C++ API     | Rust bindings
----------- | -------------
`operator-` | `Neg`
`operator!` | `Not`

### `PartialOrd`

Rust only permits a single `PartialOrd` impl for a given pair of operand types,
so Crubit picks one of the relational operators (`<`, `>`, `<=`, `>=`) to back
the impl, in that order of preference, and synthesizes `partial_cmp` from it
together with `operator==`. The remaining relational operators are not bound
directly; they are served by the default `PartialOrd` methods, which are derived
from `partial_cmp`.

A `PartialOrd` impl is only generated if `operator==` is also available for the
same operand types, and if the left- and right-hand operands have the same type.

The three-way comparison operator (`operator<=>`) is not yet supported. Types
that only declare `operator<=>` do not receive a `PartialOrd` impl: Crubit only
binds explicitly declared relational operators, not the rewritten candidates
that C++20 synthesizes from `<=>`.

### Operators inherited from base classes

C++ operators are sometimes defined as "hidden friends" of a base class, for
example by a CRTP mixin which implements comparison operators for the derived
class:

```c++
template <typename T>
struct EqualityMixin {
  friend bool operator==(const T& lhs, const T& rhs) { ... }
};

struct S final : EqualityMixin<S> { ... };
```

Argument-dependent lookup finds such operators for `S`, even though `S` doesn't
declare them itself. To map them into Rust traits as well (e.g. `PartialEq` for
`S`), enable the `inherited_friend_operators` Crubit feature
on
the target that defines `S`. Only operators whose first parameter is (a
reference to) `S` are mapped.

This is opt-in, because the generated trait implementations conflict with any
existing manual implementations of the same traits.

## One-way map into `Display`

By default: for a C++ type `T`, Crubit maps one-way the following C++ signatures
into the Rust trait `Display`:

*   `template <typename Sink> void AbslStringify(Sink&, const T&)`
*   `template <typename Sink> void AbslStringify(Sink&, T)`
*   `std::ostream& operator<<(std::ostream&, const T&)`
*   `std::ostream& operator<<(std::ostream&, T)`

Crubit prefers
[`AbslStringify`](https://abseil.io/docs/cpp/guides/abslstringify) but falls
back to `operator<<`.

The [attribute macro](customizing.md) `CRUBIT_OVERRIDE_DISPLAY` forces whether
to implement the `Display` binding. See the macro's documentation:

```
{{ #include ../../support/annotations.h }}
```
<!--  symbol:CRUBIT_OVERRIDE_DISPLAY -->

