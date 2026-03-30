# Rust bindings for C++ operators and special member functions

Here we describe how Crubit bindings work with C++ special member functions and
operator overloading (e.g. the copy constructor, or `operator==`) and with
traits from the Rust standard library (e.g., the `Clone` or `PartialEq` traits).

Rust traits which return `Self` by value (e.g., `Default` or `Clone`) or use
mutable references (e.g., `AddAssign`) are only implemented in Rust bindings for
a given C++ type if that type is [is `Unpin`](../../unpin.md).

## Bidirectional map

The following special member functions and traits are mapped bidirectionally:

| C++                          | Rust      | Notes                             |
| ---------------------------- | --------- | --------------------------------- |
| Default constructor          | `Default` |                                   |
| Trivial copy constructor     | `Copy`    | Rust bindings for C++ require     |
:                              :           : that the C++ type is non-abstract :
:                              :           : and has a public, trivial copy    :
:                              :           : constructor and destructor.       :
| Non-trivial copy constructor | `Clone`   |                                   |
| Destructor                   | `Drop`    |                                   |

## One-way map of C++ special member functions into Rust traits

If the C++ type [is `Unpin`](../../unpin.md), then the C++ special member
functions below are mapped one-way into the corresponding Rust traits as
follows:

| C++ API                   | Rust bindings | Notes                            |
| ------------------------- | ------------- | -------------------------------- |
| Constructor taking single | `From<T>`     | Regardless if the constructor is |
: parameter of type `T`     :               : `explicit` in the C++ API or not :

The C++ binary operators below are mapped one-way into the corresponding Rust
traits as follows:

C++ API       | Rust bindings
------------- | --------------
`operator==`  | `PartialEq`
`operator<`   | `PartialOrd`
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

