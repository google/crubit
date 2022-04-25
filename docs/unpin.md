# `Unpin` for C++ Types

SUMMARY: A C++ type is `Unpin` if it is trivially relocatable (e.g., a trivial
type, or a nontrivial type which is `[[clang::trivial_abi]]`), and is `final`.
Any such type can be used by value or plain reference/pointer in interop, all
non-`Unpin` types must instead be used behind pinned pointers and references.

A C++ type `T` is `Unpin` (always safe to manipulate through `&mut T`) if it is
known to be a **trivially relocatable type** (move+destroy is logically
equivalent to `memcpy`+release) with **insignificant padding** (it does not
matter if the padding is included in that `memcpy`).

`Unpin` C++ types can be used like any other normal Rust type: they are always
safe to access by reference or by value. Non-`Unpin` types, in contrast, can
only be accessed behind pins such as `Pin<&mut T>`, or `Pin<Box<T>>`, because it
may not be safe to directly mutate. These types are never used directly by value
in Rust, because value-like assignment has incorrect semantics: it fails to run
C++ special members for non-trivially-relocatable types, it can overwrite
padding for types with significant padding.

## Trivially Relocatable Types

In C++, moving a value between locations in memory involves executing code to
either initialize (move-construct) or overwrite (move-assign) the new location.
The old location still exists, but is in a moved-from state, and must still be
destroyed to release resources.

(For example, `std::string x = std::move(y);` will run the move constructor, so
that `x` contains the same value that `y` used to have before the move. The
variable `y` will still be a valid string, but might be empty, or might contain
some garbage value. The destructors for both `x` and `y` will run when they go
out of scope.)

Rust does not have move constructors or move assignment. In fact, there is no
way to customize what happens during moving or assignment: in Rust, moving or
swapping an object means changing its location in memory, as if by `memcpy`
without running the destructor logic in the old location. Another way of looking
at it is that it's as if an object moved around in memory over time: it is
constructed in one place, and then further operations and eventual destruction
might happen in other places. We call such a Rust-like move a "trivial
relocation" operation.

Despite C++ moves using explicit construction and destruction calls, many C++
types could also have used the Rust movement model. We call such types
**trivially relocatable** types.

For example, a C++ `std::unique_ptr`, implemented in the obvious way, is
trivially relocatable: its actual location in memory does not matter. In
contrast, a self-referential type is not trivially relocatable, because to
relocate it, you must also update the pointer it has to itself. This is done
inside the move constructor in C++, but cannot be done in the Rust model, where
the move operation is not customizable.

For more background, see
[P1144](http://www.open-std.org/jtc1/sc22/wg21/docs/papers/2020/p1144r5.html).

### Which types are trivially relocatable?

For the purpose of Rust/C++ interop, we define a type to be trivially
relocatable if, and only if, it is "trivial for calls" in Clang. That is,
either:

1.  It is actually
    [trivial](https://en.cppreference.com/w/cpp/named_req/TrivialType), **or**
2.  It uses
    [`[[clang::trivial_abi]]`](https://clang.llvm.org/docs/AttributeReference.html#trivial-abi)
    to make itself trivial for calls

This definition is conservative: some types that could be considered trivially
relocatable are not trivial for calls. (For example, `std::unique_ptr` uses
`[[clang::trivial_abi]]` only in the unstable libc++ ABI; the stable libc++ ABI
predates this attribute, and adding it now is ABI-breaking.)

This definition is, however, sound: all types which are trivial for calls are
trivially relocatable, because a type which is trivial for calls is
trivially-relocated when passed by value as a function argument.

### Expanding trivial relocatability

We are working to extend libc++ and Clang to trivially relocate these types in
even more circumstances, which would make `[[clang::trivial_abi]]` more
compelling and more widely used, enhancing both performance and
Rust-compatibility for our C++ core libraries.

*   [[clang] Mark `trivial_abi` types as "trivially relocatable".](https://reviews.llvm.org/D114732)
*   [Use trivial relocation operations in std::vector, by porting D67524 and
    part of D61761 to work on top of the changes in
    D114732.](https://reviews.llvm.org/D119385)

A future change to C++ or Clang in the vein of
[P1144](http://www.open-std.org/jtc1/sc22/wg21/docs/papers/2020/p1144r5.html)
could make types trivially relocatable without requiring ABI changes as
`[[clang::trivial_abi]]` does, although in the short term this doesn't seem very
likely.

## Insignificant Padding

If a type has padding, then even if the type is trivially relocatable and
therefore safe to write as if by `memcpy`, **Rust will `memcpy` an incorrect
number of bytes**: Rust will include the padding, though C++ would not.
Trivially relocatable types where the padding potentially has semantic meaning
can still be handled by value, but are `!Unpin`, and all mutable references Rust
receives from C++ must be `Pin<&mut T>`. Only trivially relocatable types where
the padding has no significance can be `Unpin` and safe to deal with via `&mut`.

Significant padding occurs via inheritance -- derived types may reuse the
padding for other objects -- and from the `[[no_unique_address]]` attribute
(which declares the padding to be reusable).

For the purposes of C++/Rust interop, `[[no_unique_address]]` is an unsafe
feature, and any type which cannot be inherited from (via e.g. `final`) is
considered to have insignificant padding.

### When is padding significant?

In C++, if you take a mutable reference to a base class subobject, and pass it
around, this is ultimately pretty safe. If you assign to it, it is a bit bad --
it will assign to only the base class subobject (if it's nonvirtual), not just
the subclass -- but it's possible for this to make sense, and if it were truly
dangerous they'd probably have deleted assignment or not inherited from the base
class.

In Rust, this is *extremely dangerous*, because the size of the base class
subobject can extend to include fields from the derived class. For example, take
this class hierarchy:

```c++
class Base {
  int64_t x_;
  int32_t y_;
  /* ...methods... */
};

class Derived : public Base {
  int32_t size_;
  char* data_;
  /* ...methods... */
};
```

Here we have a class `Derived` with some string data, which inherits from
`Base`. But something unfortunate happens: because `Base` has an extra 32 bits
of tail padding, and is not POD for the purpose of layout, the `size_` member of
the derived class is stored inside the tail padding for `Base`. This is allowed
by the C++ standard, and actually taken advantage of in the Itanium ABI.

In C++, this presents no problems, as C++ assignment doesn't do something like
`memcpy sizeof(x) bytes`, even when the class is trivially assignable. It only
copies the real data size, excluding padding. And so this code will not
accidentally overwrite the `size_` field:

```c++
Derived& d = ...;
Base& b1 = d;
Base& b2 = ...;
std::swap(b1, b2);
```

But the seemingly equivalent Rust code absolutely will:

```rs
let d : &mut Derived = ...;
let b1 : &mut Base = d.into();
let b2 : &mut Base = ...;
// This overwrites size_ from the derived class with uninitialized memory from
// b2.
std::mem::swap(b1, b2); // Catastrophically bad.
```

As a consequence, types like `Base` should not be exposed as `&mut` references:
they might refer to a base class subobject, in which case assignment in Rust
will do the wrong thing. Even if they are trivially relocatable and assignment
is equivalent to a `memcpy`, Rust will memcpy the wrong number of bytes.

### Gaps

#### `[[no_unique_address]]`

The exact same behavior can occur with `[[no_unique_address]]`. There are three
options:

1.  Live with the unsafety of `[[no_unique_address]]`, and make it buyer beware.
    This is similar to how we treat packed struct fields.

2.  Forbid `[[no_unique_address]]` in the C++ style guide, except for zero-sized
    types (which we can probably handle fine).

3.  Switch approaches: rather than only allowing it for `final` classes and the
    like, only allow it for classes whose data size is guaranteed to be the same
    as their stride, possibly using something like a `[[pod_layout]]` attribute.

For now, we take approach #1: `[[no_unique_address]]` is considered an unsafe
feature, which can render padding significant on any type which has padding.

#### Lambdas

TODO: implement this.

Lambdas are class types, are not `final`, and cannot be marked `final`. Most
likely, we need to simply pretend that they are `final` -- it is not very useful
to inherit from a lambda, and this should not break people in practice.

### How common is this?

Only ~4% of classes at Google are base
classes to some other type.

This means the number of classes that *should* be pinned due to potentially
significant padding is low, and the number of classes that *should* be marked
final is high. Mixed blessings: more boilerplate in C++, but less annoyance in
Rust, as the vast majority of classes can be marked `final` via LSC.

However, 4% doesn't quite seem small enough that we can pretend the issue
doesn't exist.
