# Thunks for class template member functions

## Problem definition

Given the C++ header below...

```cpp
#pragma clang lifetime_elision

template <typename T>
class MyTemplate {
 public:
  MyTemplate(T value) : value_(value) {}
  const T& GetValue() const;
 private:
  T value_;
};

using MyIntTemplate = MyTemplate<int>;
```

... Crubit will generate Rust bindings that can call into the
`MyTemplate<int>::GetValue()` member function. To support such calls, Crubit has
to generate a C++ *thunk* (to *instantiate* the class template and to provide a
symbol for a C-ABI-compatible function that Rust can call into):

```cpp
extern "C"  // <- C ABI
int const& __rust_thunk___ZNK10MyTemplateIiE8GetValueEv(
    const class MyTemplate<int>* __this) {
  return __this->GetValue();
}
```

There are other (non-`template`-related) scenarios that require generating
thunks (e.g. `inline` functions, or functions that use a custom calling
convention), but templates bring one extra requirement: a class template can
be defined in one header (say `my_template.h`) and used in *multiple* other
headers (e.g. `library_foo/template_user1.h` and
`library_bar/template_user2.h`). Because of this, the same thunk might need to
be present in *multiple* generated `..._rs_api_impl.cc` files (e.g. in
`library_foo_rs_api_impl.cc` and `library_bar_rs_api_impl.cc`). This may lead to
duplicate symbol errors from the linker:

```stderr
ld: error: duplicate symbol: __rust_thunk___ZNK10MyTemplateIiE8GetValueEv
```

## Implemented solution: Encoding target name in the thunk name

One solution is to give each of the generated thunks a unique,
target/library-specific name, e.g.:
`__rust_thunk___ZNK10MyTemplateIiE8GetValueEv__library_foo` (note the
`library_foo` suffix).

Pros:

-   **Minimal extra code complexity** (e.g. no need for templates-specific code
    in thunk-related code in `src_code_gen.rs`).
-   **Obviously correct behavior-wise** (e.g. since it is just like other thunks
    which we assume are implemented correctly).

Cons:

-   **Performance guarantees are unclear**. Binary size depends on link time
    optimization (LTO) recognizing that all the thunks are identical and
    deduplicating them.

    -   This seems to work in practice (at least for production binaries).
    -   Future work: add tests + consider asking LLVM to provide LTO guarantees

-   **Requires escaping Bazel target names** into valid C identifiers. See
    `ConvertToCcIdentifier(const BazelLabel&)` in `bazel_types.cc`.

## Alternative solutions

### Function template

An alternative solution would be to use a function template that we immediately
explicitly instantiate. These still generate the code we need, but their
duplicated symbol definitions (across multiple binding crates) won't cause an
ODR violation. It is expected that a single function template is instantiated
multiple times in multiple translation units, therefore the linker silently
merges these equivalent definitions.

Example:

```cpp
// Thunk is expressed as a function template:
template <typename = void>
__attribute__((__always_inline__)) int const&
__rust_thunk___ZNK10MyTemplateIiE8GetValueEv(
    const class MyTemplate<int>* __this) {
  return __this->GetValue();
}

// Explicit instantiation of the function template:
// (to generate a symbol that `..._rs_api.rs` can call into)
template int const& __rust_thunk___ZNK10MyTemplateIiE8GetValueEv(
    const class MyTemplate<int>* __this);
```

Pros:

-   **Naturally deduplicated** (just depending on what C++ already does for
    function templates).

Cons:

-   **Assumes a particular ABI** - a function template specialization uses the
    calling convention prescribed by the platform C++ ABI.  We know that
    [the Itanium ABI maps C++ sigatures to the C
    ABI](https://itanium-cxx-abi.github.io/cxx-abi/abi.html#functions) and
    therefore will be compatible with the calling convention expected by the
    generated `..._rs_api.rs`.  Further research is needed to investigate the
    guarantees offered by other platforms (e.g., the MSVC ABI).
-   **Requires extra complexity** to calculate the mangled name of the function
    template specialization.
    -   Crubit doesn’t have a `clang::FunctionDecl` corresponding to the
        function-template-based thunk, and therefore Crubit can’t use
        `clang::MangleContext::mangleName` to calculate the linkable/mangled
        name of the thunk.
    -   Reimplementing `clang::MangleContext::mangleName` in Crubit seems
        fragile. One risk is bugs in Crubit's code that would make it behave
        differently from Clang (e.g. code review of the initial prototype
        identified that
        [mangling compression](https://itanium-cxx-abi.github.io/cxx-abi/abi.html#mangling-compression)
        was missing). Another risk is having to implement not just
        `ItaniumMangleContext`, but also `MicrosoftMangleContext`.
    -   One idea to avoid reimpliementing mangling is to explicitly specify
        the name for the function template instantiation using
        `__asm__("abc")` (sadly this doesn't seem to work - it may be a
        Clang bug).

An abandoned prototype of this approach can be found in a (Google-internal)
cl/450495903.

### Explicit `linkonce_odr` attribute

Example:

```cpp
extern "C"
int const& __rust_thunk___ZNK10MyTemplateIiE8GetValueEv(
    const class MyTemplate<int>* __this)
    __attribute__((linkonce_odr))  // <- THIS IS THE PROPOSAL
{
  return __this->GetValue();
}
```

Pros:

-   All the "pros" of the "Encoding target name in the thunk name" approach
    (simplicity + correctness of behavior)
-   All the "pros" of the "Function template" template approach (deduplication)

Cons:

-   Requires changing Clang to support the new attribute (e.g. requires
    convincing the Clang community that this is a language extension that is
    worth supporting).
    **TODO(b/234889162)**: Send out a short RFC to gauge interest?

## Rejected solutions

-   [`selectany`](https://clang.llvm.org/docs/AttributeReference.html#selectany)
    doesn't work with functions, only data members. Furthermore, we need
    something that maps to `linkonce_odr`, and selectany maps only to
    `linkonce`.

-   `__attribute__((weak))` has the disadvantage that a weak definition can be
    overridden by a strong one. This rule makes weak definitions non-inlineable
    except in full-program LTO. C++ function template instead follows the ODR
    rule that says that all definitions must be equivalent, making them
    inlineable.
