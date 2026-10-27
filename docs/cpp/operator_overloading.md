# Rust bindings for C++ operators and special member functions

Here we describe how Crubit bindings work with C++ special member functions and
operator overloading (e.g. the copy constructor, or `operator==`) and with
traits from the Rust standard library (e.g., the `Clone` or `PartialEq` traits).

Rust traits which return `Self` by value (e.g., `Default` or `Clone`) or use
mutable references (e.g., `AddAssign`) are only implemented in Rust bindings for
a given C++ type if that type is [is `Unpin`](../../unpin.md).

## Bidirectional map

The following special member functions and traits are mapped bidirectionally:

| C++                          | Rust      | Notes                                                                                                                      |
| :--------------------------- | :-------- | :------------------------------------------------------------------------------------------------------------------------- |
| Default constructor          | `Default` |                                                                                                                            |
| Trivial copy constructor     | `Copy`    | Rust bindings for C++ require that the C++ type is non-abstract and has a public, trivial copy constructor and destructor. |
| Non-trivial copy constructor | `Clone`   |                                                                                                                            |
| Destructor                   | `Drop`    |                                                                                                                            |

## One-way map of C++ special member functions into Rust traits

If the C++ type [is `Unpin`](../../unpin.md), then the C++ special member
functions below are mapped one-way into the corresponding Rust traits as
follows:

| C++ API                                         | Rust bindings | Notes                                                             |
| :---------------------------------------------- | :------------ | :---------------------------------------------------------------- |
| Constructor taking single parameter of type `T` | `From<T>`     | Regardless if the constructor is `explicit` in the C++ API or not |

The C++ binary operators below are mapped one-way into the corresponding Rust
traits as follows:

| C++ API       | Rust bindings                                                     |
| :------------ | :---------------------------------------------------------------- |
| `operator==`  | `PartialEq`                                                       |
| `operator<=>` | `PartialOrd` (and opt-in `Eq` + `Ord` for `std::strong_ordering`) |
| `operator<`   | `PartialOrd`                                                      |
| `operator>`   | `PartialOrd`                                                      |
| `operator<=`  | `PartialOrd`                                                      |
| `operator>=`  | `PartialOrd`                                                      |
| `operator+`   | `Add`                                                             |
| `operator-`   | `Sub`                                                             |
| `operator*`   | `Mul`                                                             |
| `operator/`   | `Div`                                                             |
| `operator%`   | `Rem`                                                             |
| `operator&`   | `BitAnd`                                                          |
| `operator\|`  | `BitOr`                                                           |
| `operator^`   | `BitXor`                                                          |
| `operator<<`  | `Shl`                                                             |
| `operator>>`  | `Shr`                                                             |
| `operator+=`  | `AddAssign`                                                       |
| `operator-=`  | `SubAssign`                                                       |
| `operator*=`  | `MulAssign`                                                       |
| `operator/=`  | `DivAssign`                                                       |
| `operator%=`  | `RemAssign`                                                       |
| `operator&=`  | `BitAndAssign`                                                    |
| `operator\|=` | `BitOrAssign`                                                     |
| `operator^=`  | `BitXorAssign`                                                    |
| `operator<<=` | `ShlAssign`                                                       |
| `operator>>=` | `ShrAssign`                                                       |

The C++ unary operators below are mapped one-way into the corresponding Rust
traits as follows:

| C++ API     | Rust bindings |
| :---------- | :------------ |
| `operator-` | `Neg`         |
| `operator!` | `Not`         |

### `PartialOrd`

Rust only permits a single `PartialOrd` impl for a given pair of operand types,
so Crubit picks one of the comparison operators (`<=>`, `<`, `>`, `<=`, `>=`) to
back the impl, in that order of preference. The remaining comparison operators
are not bound directly; they are served by the default `PartialOrd` methods,
which are derived from `partial_cmp`.

`operator<=>` (the "spaceship" operator) implements `partial_cmp` directly. It
must return `std::strong_ordering`, `std::weak_ordering`, or
`std::partial_ordering` (`std::partial_ordering::unordered` maps to `None`). For
the other operators, Crubit synthesizes `partial_cmp` from the operator together
with `operator==`.

A `PartialOrd` impl is only generated if `operator==` is also available for the
same operand types, and if the left- and right-hand operands have the same type.

### `Eq` and `Ord`

If `operator<=>` returns `std::strong_ordering` (and Crubit implements
`PartialOrd` with it), then Crubit also implements `Eq` and `Ord`. This lets you
use the type in a `BTreeMap` or `BTreeSet`, and sort it with `sort()`.

For now, this requires the `strong_ordering_traits` Crubit
feature ,
which is also part of `experimental`. This lets existing Crubit users migrate
gradually, if they have manual `Eq` or `Ord` impls that conflict with the
generated impls.

Crubit doesn't implement `Eq` and `Ord` for `std::weak_ordering` and
`std::partial_ordering`.

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
`S`), enable the `inherited_friend_operators` Crubit
feature
on the target that defines `S`. Only operators whose first parameter is (a
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

```live-snippet
cs/file:support/annotations.h symbol:CRUBIT_OVERRIDE_DISPLAY
```
