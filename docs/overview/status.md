# Are we Crubit Yet?

NOTE: The bug links below, of the form `b/123456`, are for Google-internal
tracking purposes.

What follows is an overview of the major features Crubit does and does not
support. The list is necessarily incomplete, because there exist more features
and types than could be feasibly listed in anything readable, but it should give
a rough idea.

This page should evolve over time:

*   If the status of a given feature is not listed, and not clear based on what
    *is* here, we should add it.
*   Some features may not have bug IDs attached. If a feature is actively
    requested, it should be listed with a given bug that updates will be posted
    to.
*   This page may fall out of date, since the set of features supported by
    Crubit is documented in many places. Sorry! Please update it if you notice
    any problems.

## Types

See <internal link>/types for more details about types in general, including
explanations of what it means for a type to be ABI-compatible versus
layout-compatible.

Unless otherwise specified, the types below are supported and ABI-compatible
(see <internal link>/types/primitive, <internal link>/types/pointer):

*   integer types (except 128-bit integers)
*   floating point types
*   user-defined types
    *   These are either layout-compatible (usually) or ABI-compatible (rarely –
        if all member types are supported, and it's nonempty, and it uses no
        obscure attributes)
*   function pointers, where the parameters and return type are in this list and
    are ABI-compatible
*   `std::string_view` / `absl::string_view`
*   Bridged: `std::string`
*   Bridged: `&str`
*   Bridged: Rust tuples (e.g. `(i32, i64)`)
*   Bridged: `std::optional<T>`
*   Bridged: (allowlisted) protocol buffers
*   Bridged: `absl::Status`
*   raw pointers to any ABI-compatible or layout-compatible item in this list

We have *experimental* unreleased support for the following types:

*   (2025H2) b/362475441: references and pointers to `MaybeUninit<T>`, which are
    treated as `T`.

We have planned support for the following types:

*   (2025H2) b/271016831: layout-compatible `*const [T]`, `*mut [T]`
*   (2025H2) bridged `Option<T>`
*   (2025) b/356638830: layout-compatible `std::vector`
*   (2025) b/369994952: layout-compatible `std::unique_ptr`

The following types are **not** yet supported, among many others:

*   b/254507801: Rust `!`
*   b/260128806: Arrays (`std::array<T, N>`, `[T; N]`)
*   b/254094650: `i128` and `u128`
*   Rust `String`
*   `Result<T, E>`
*   b/254099023: `()` as anything but a return type.
*   b/213960614: `std::byte`

## C++

For C++ libraries, used from Rust, we have support for the following **language
features**, used in public interfaces:

*   rust-movable structs. (Either trivially copyable, or
    `[[clang::trivial_abi]]`)
*   rust-movable unions.
*   enums
*   type aliases
*   non-overloaded functions (which are **not** member functions)
    *   inline or non-inline
    *   extern "C" or non-extern "C"

We have *experimental* unreleased support for the following language features:

*   forward declarations
*   non-trivial types
*   b/356224404: non-overloaded member functions, (overloaded) constructors and
    assignment operators
*   templated types, bridged to a non-generic concrete type.
    *   e.g. `vector<int>` becomes `struct __crubit_mangled_vector_i`, not
        `struct vector<T>(...)`
    *   specialization
*   operator overloading
*   nullability annotations
*   lifetime annotations, mapped unsafely to references
*   Some object-orientation:
    *   types with **non-virtual** base classes
    *   upcasting
    *   downcasting
    *   inherited methods

The following features are **not** supported yet, among many others:

*   b/213280424: overloading
*   b/313733992: Object-Oriented Programming more generally
    *   e.g., cannot derive from a C++ class and override its virtual methods
*   *safe* support for references
*   template-generic bridging, so that a C++ template becomes a Rust generic
*   non-type `using` aliases
    *   using enum
    *   using namespace
*   constants
*   macros

## Rust

For Rust libraries, used from C++, we have support for the following language
features, used in public interfaces:

*   structs
*   `repr(C)` unions
*   opaque representations of other user-defined types
    *   enums
    *   non-repr(C) unions
*   aliases (via `use`, `type`)
*   functions and methods
*   references
*   specific known traits with equivalents in C++:
    *   `Clone`
    *   `Default`
    *   `Drop`
    *   `From`
*   simple `const` constants
*   Defining a C++ enum from Rust

We have *experimental* unreleased support for the following language features:

*   non-opaque enums
*   non-opaque non-`repr(C)` unions

The following features are **not** supported yet, among others:

*   traits and trait methods in general
*   defining C++ abstractions from Rust
    *   inheriting from a C++ class
    *   defining a C++ base class
*   statics and more complex `const` constants
*   macros

## Usage outside of Google

Crubit was initially written to take advantage of the superpowers that come with
a centrally controlled monorepo using a Bazel build system. However, this
presents a high barrier to entry: in order to use Crubit, you must satisfy all
of the preconditions.

In 2026, we are building Crubit up to be a tool shaped like OSS users
expect: an IDL-based FFI tool with Cargo integration, with _options_ for a better
experience in codebases with strong control over the build environment. (Though
for calling Rust from C++, we might stop short of an IDL, and instead rely on
compiler-synced binary releases, since there is only one compiler.)

In particular, this involves decomposing Crubit into a collection of parts that
can be used on their own, without needing to consume the whole:

* Reusable libraries that implement C++ functionality (e.g., forward declarations,
  nontrivial object semantics.)
* An IDL-based core, with optional compiler integration at the front-end.
* Support for building with Cargo, stable named versions of Clang or Rust, etc.

### Decoupling from the toolchain

By using an IDL as input, instead of a C++ compiler frontend, Crubit can be made
compatible with arbitrary C++ compilers: a human can write the IDL in a way that
is compatible with the compiler in question, even if Crubit does not integrate
with that compiler yet.

For the Rust compiler, however, there is only one. The main toolchain
integration hazard is that the compiler and its arguments must be exactly
matched with the version and arguments used to compile the Rust crate itself.
This can be resolved by using rmeta files as inputs, instead of source code.

> TODO:
>
> *   rs_bindings_from_idl and idl_from_cc exist, and Crubit can be used
>     with IDL inputs
> *   cc_bindings_from_rs can accept rmeta inputs

### Crate Ecosystem

> TODO:
>
> *   Crubit accepts pull requests and regularly reviews GitHub issues and PRs.
> *   A C++ stdlib crate exists in crates.io
> *   The Crubit `ctor` crate is either replaced with `pin-init`, the equivalent
>     standard library module, or else has a crate in crates.io with
>     documentation and an explanation of why to use it vs `pin-init`.
> *   For all other support libraries: they exist in crates.io and are
>     documented.

### Build System

We currently only support Bazel.

> TODO:
>
> *   cc_bindings_from_rs builds using Cargo
> *   rs_bindings_from_cc builds using Cargo
> *   idl_bindings_from_cc, rs_bindings_from_idl build using Cargo
> *   Crubit is usable as a Bazel dependency
> *   Crubit is usable as a Bazel dependency
> *   Crubit builds against public Rust and Clang releases
> *   Crubit binary releases
> *   (not planned) Buck2
> *   (not planned) CMake
