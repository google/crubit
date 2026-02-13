# Unsupported type errors

Sometimes, a type is not supported in Crubit. This presents a problem for all
functions that use that type, and so Crubit aims to actually cover *all* C++
types, even if in only minimal form. However, sometimes that aspiration is still
not met today, and functions like the following are not callable at all:

{{#tabs}}

{{#tab name="C++"}}

```c++
Unsupported DoSomething(Unsupported in_val, Unsupported& in_ref);
```

{{#endtab}}

{{#tab name="Rust"}}

```rust
pub fn DoSomething(in_val: Unsupported, in_ref: &mut Unsupported) -> Unsupported {...}
```

{{#endtab}}

{{#endtabs}}

This example will be used for reference below, though most of the advice also
applies to functions and types defined in Rust.

## Fix

There are roughly four approaches to solving this:

*   (Most preferred) if possible, convince Crubit to generate bindings.
*   Use wrapper types
*   (For C++ libraries only) Use forward declarations
*   (Dispreferred) Use void pointers

### (Preferred) Convince Crubit to generate bindings {#fix-preferred}

If Crubit is not enabled on the library defining `Unsupported`, first, enable
Crubit. (See crubit.rs/cpp/ and crubit.rs/rust.)

If Crubit is enabled on the target, the generated bindings will contain a
comment explaining the error. To
see the error message as a compilation error, apply the `CRUBIT_MUST_BIND`
Crubit attribute to the `Unsupported` type definition.

The error may describe some way to resolve the type. For example, if it has an
unrecognized attribute, see crubit.rs/errors/unknown_attribute.

However, ultimately, it may boil down to a bug or missing feature in Crubit, and
you may need to work around `Unsupported` not having a type. This can be done by
writing a wrapper around the `DoSomething` function which doesn't use that type.
Below are a handful of common approaches.

### Replace it with a wrapper type {#fix-wrapper}

Even if the type is not supported, a type *containing* that type will likely
work fine. Crubit relies on the abstract properties of the overall compound data
type, like its size and alignment, and does not need each field to individually
work with Crubit.

(In C++, you may also want to make the wrapper type Rust-movable for
convenience. See crubit.rs/cpp/cookbook#rust_movable)

{{#tabs}}

{{#tab name="C++"}}

To make the example C++ function `DoSomething` callable from Rust:

```c++
// mylib_ffi.h
#include "third_party/absl/base/attributes.h"
#include "support/annotations.h"
struct ABSL_ATTRIBUTE_TRIVIAL_ABI UnsupportedWrapper {
  Unsupported value;
};

CRUBIT_RUST_NAME("DoSomething")
UnsupportedWrapper DoSomethingRust(
    UnsupportedWrapper in_val,
    UnsupportedWrapper* in_ref);
```

```c++
// mylib_ffi.cc
#include "mylib_ffi.h"
#include "mylib.h"

UnsupportedWrapper DoSomethingRust(
    UnsupportedWrapper in_val,
    UnsupportedWrapper* in_ref) {
  return UnsupportedWrapper{
      DoSomething(std::move(*in_val.value), in_ref ? &in_ref->value : nullptr)
  };
}
```

{{#endtab}}

{{#tab name="Rust"}}

To make the example Rust function `DoSomething` callable from C++:

```rust
// mylib_ffi.rs
pub struct UnsupportedWrapper(Unsupported);

#[crubit_annotate::cpp_name("DoSomething")]
pub fn DoSomethingCpp(in_val: UnsupportedWrapper,
                      in_ref: &mut UnsupportedWrapper) -> UnsupportedWrapper {
    UnsupportedWrapper(DoSomething(in_val.0, &mut in_ref.0))
}
```

{{#endtab}}

{{#endtabs}}

### (C++) Replace it with a forward declaration in wrapper functions {#fix-forward_declare}

TODO(b/482061078): Support this workaround in Rust, as well.

NOTE: Crubit does this automatically in `wrapper` mode, and this may become the
default behavior for unsupported types in supported targets in a future release,
so that no additional work is necessary.

In an FFI-oriented header, you can define a replacement `DoSomething` function
that uses void a wrapper type, and which, at worst, heap-allocates when passing
or returning by value. However, you must also free the memory when complete.
Using a wrapper is likely to be more user-friendly.

To make the example C++ function `DoSomething` callable from Rust:

```c++
// mylib_ffi.h
#include "support/annotations.h"
class Unsupported;

CRUBIT_RUST_NAME("DoSomething")
Unsupported* DoSomethingRust(Unsupported* in_val, Unsupported* in_ref);
```

```c++
// mylib_ffi.cc
#include "mylib_ffi.h"
#include "third_party/absl/memory/memory.h"
#include "mylib.h"

Unsupported* DoSomethingRust(Unsupported* in_val, Unsupported* in_ref) {
  std::unique_ptr<Unsupported> in_val_owned = absl::WrapUnique(in_val);
  return new auto(DoSomething(std::move(*in_val_owned), in_ref));
}
```

Here, the wrapper uses raw pointers (in Rust) where C++ would use values, in
order to allow passing an unsupported type by value. However, by reference and
by pointer, there is no heap allocation.

### (Dispreferred) Replace it with a void pointer {#unsupported-type-void}

BEST PRACTICE: This is identical to the forward declaration solution when
applicable, but is more dangerous due to erasing the type and using `void`.

In an FFI-oriented header, you can define a replacement `DoSomething` function
that uses void pointers, and which, at worst, heap-allocates when passing or
returning by value. However, you must also free the memory when complete, this
is not type safe. Using a wrapper or a forward declaration is both safer and
more user-friendly.

{{#tabs}}

{{#tab name="C++"}}

To make the example C++ function `DoSomething` callable from Rust:

```c++ {.bad}
// mylib_ffi.h
#include "support/annotations.h"

CRUBIT_RUST_NAME("DoSomething")
void* DoSomethingRust(void* in_val, void* in_ref);
```

```c++ {.bad}
// mylib_ffi.cc
#include "mylib_ffi.h"
#include "third_party/absl/memory/memory.h"
#include "mylib.h"

void* DoSomethingRust(void* in_val, void* in_ref) {
  std::unique_ptr<Unsupported> in_val_owned = absl::WrapUnique(reinterpret_cast<Unsupported*>(in_val));
  return new auto(DoSomething(std::move(*in_val_owned), reinterpret_cast<Unsupported*>(in_ref)));
}
```

{{#endtab}}

{{#tab name="Rust"}}

To make the example Rust function `DoSomething` callable from C++:

```rust {.bad}
// mylib_ffi.rs

/// # Safety:
///
/// in_val must be an owned pointer to an Unsupported, in_ref must be a non-null
/// borrowed pointer to an Unsupported.
#[crubit_annotate::cpp_name("DoSomething")]
pub unsafe fn DoSomethingCpp(in_val: *mut c_void, in_ref: *mut c_void) -> *mut c_void {
    unsafe {
        DoSomething(
            (in_val as *mut Unsupported).read(),
            *(in_ref as *mut Unsupported)
        ) as *mut _ as *mut _
    }
}
```

{{#endtab}}

{{#endtabs}}
