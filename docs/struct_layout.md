# Struct Layout

C++ (in the Itanium ABI) extends the C layout rules, and so `repr(C)` isn't
enough. This pages documents the tweaks to Rust structs to give them the same
layout as C++ structs.

In particular:

*   C++ classes and Rust structs must have the same alignment, so that
    references can be exchanged without violating the alignment rules. This is
    usually ensured by the regular `#[repr(C)]` layout algorithm, but sometimes
    the interop tool needs to generate explicit `#[repr(align(n))]` annotations.
*   C++ classes and Rust structs must have the same size, so that arrays of
    objects can be exchanged.
*   Public subobjects must have the same offsets in C++ and Rust versions of the
    structs.

## Non-field data

Rust bindings introduce a `__non_field_data: [MaybeUninit<u8>; N]` field to
cover data within the object that is not part of individual fields. This
includes:

*   Base classes.
*   VTable pointers.
*   Empty struct padding.

### Empty Structs

One notable special case of this is the empty struct padding. An empty struct or
class (e.g. `struct Empty{};`) has size `1`, while in Rust, it has size `0`. To
make the layout match up, bindings for empty structs will always enforce that
the struct has size of at least 1, via `__non_field_data`.

(In C++, different array elements are guaranteed to have different addresses,
and also, arrays are guaranteed to be contiguous. Therefore, no object in C++
can have size `0`. Rust, like C++, has only contiguous arrays, but unlike C++
Rust does not guarantee that distinct elements have distinct addresses.)

## Potentially-overlapping objects

In C++, in some circumstances, the requirement that objects do not overlap is
relaxed: base classes and `[[no_unique_address]]` member variables can have
subsequent objects live inside of their tail padding. The most famous instance
of this is the
[empty base class optimization (EBCO)](https://en.cppreference.com/w/cpp/language/ebo):
a base class with no data members is permitted to take up zero space inside of
derived classes.

NOTE: This has other, non-layout consequences for Rust: for example, it is not
safe to obtain two `&mut` references to overlapping objects, unless they are of
size `0`. (To prevent this, classes that might be base classes are always
[`!Unpin`](unpin).)

This is impossible to represent in a C-like struct. (Indeed, it's impossible to
represent even in a C++-like struct, before the introduction of
`[[no_unique_address]]`). Therefore, in Rust, we don't even try:
potentially-overlapping subobjects are replaced in the Rust layout by a
`[MaybeUninit<u8>; N]` field, where `N` is large enough to ensure that the next
subobject starts at the correct offset. The alignment of the struct is still
changed so that it matches the C++ alignment, but via `#[repr(align(n))]`
instead of by aligning the field.

### Example

For example, consider these two C++ classes:

```c++
// This is a class, instead of a struct, to ensure that it is not POD for the
// purpose of layout. (The Itanium ABI disables the overlapping subobject
// optimization for POD types.)
class A {
  int16_t x_;
  int8_t y_;
};

struct B final : A {
  int8_t z;
}
```

In memory, this may be laid out as so:

```
| x_ | x_ | y_ | z |
 <------------> <->
  A subobject  | B
<------------------>
  sizeof(A)
  (also sizeof(B))

```

The correct representation for `B`, in Rust, is something like this:

```rs
#[repr(C)]
#[repr(align(2))] // match the alignment of the int16_t variable.
struct B {
  // The We don't use a field of type `A`, because it would have a size of 4,
  // and Rust wouldn't permit `z` to live inside of it.
  // Nor do we align the array, for the same reason -- correct alignment must be
  // achieved via the repr(align(2)) at the top.
  __non_field_data : [MaybeUninit<u8>; 3];
  pub z: i8,
}
```
