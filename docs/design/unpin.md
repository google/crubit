# `Unpin` for C++ Types

SUMMARY: A C++ type is `Unpin` if it is trivially relocatable (e.g., a trivial
type, or a nontrivial type which is `[[clang::trivial_abi]]`). Any such type can
be used by value or plain reference/pointer in interop, all non-`Unpin` types
must instead be used behind pinned pointers and references.

A C++ type `T` is `Unpin` if it is known to be a **trivially relocatable type**
(move+destroy is logically equivalent to `memcpy`+release).

`Unpin` C++ types can be used like any other normal Rust type: they are always
safe to access by reference or by value. Non-`Unpin` types, in contrast, can
only be accessed behind pins such as `Pin<&mut T>`, or `Pin<Box<T>>`, because it
may not be safe to directly mutate. These types are never used directly by value
in Rust, because value-like assignment has incorrect semantics: it fails to run
C++ special members for non-trivially-relocatable types.

Note that not every object with an `Unpin` type is actually safe to hold in a
mutable reference. Objects with live aliases still must not be used with `&mut`,
and "potentially overlapping objects" can produce unexpected behavior in Rust.
(See [Reference Safety](#reference_safety).)

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

## Reference Safety

Not every object with an `Unpin` type can actually safely be pointed to by a
Rust reference.

### Conventional aliasing

If a C++ reference mutably aliases, it is unsafe to pass to Rust as a Rust
reference. Do not under any circumstance create aliasing Rust references, the
behavior of doing so is undefined.

For example:

```rust
pub fn foo(_: &mut i32, _: &mut i32) {}
```

It is Undefined Behavior to, in C++, call `foo(x, x)`.

### Tail padding

In C++, tail padding is not part of the object, and the space in the tail
padding can be taken up by other unrelated objects. Avoid creating a Rust
reference to a base class, or to a `[[no_unique_address]]` field, as these are
"potentially overlapping". This can cause surprising behavior, or unintended
aliasing and undefined behavior.

Consider the following struct:

```c++
struct A {};
struct B {
    [[no_unique_address]] A field_1_;
    char field_2_;

    A& field_1() { return field_1_; }
    char& field_2() { return field_2_; }
};
```

Here, while `sizeof(A)` is `1`, it has no data, only tail padding. A C++
assignment to `field_1_` will not write anything. And so C++ can store an
unrelated object inside of the tail padding. `[[no_unique_address]]` marks the
tail padding as available for use. `field_2_` may actually be stored inside the
tail padding of `field_1_`, and the `sizeof(B)` may also be `1`.

(Base classes also allow their tail padding to be reused, and the same example
works with `struct B : A`.)

```c++
static_assert(sizeof(A) == sizeof(B));
static_assert(offsetof(B, field_1) == offsetof(B, field_2));
```

Rust does not work this way. In Rust, tail padding *is* part of the object. Rust
references refer to the full span of the pointed-to object, including that tail
padding. And so a Rust reference to `field_1_` would encompass `field_2_` by
accident.

This means that the following code has undefined behavior via conventional
aliasing, despite looking fairly innocent:

```c++
B b = ...;
// Rust: pub fn foo(_: &mut A, _: &mut u8)
foo(b.field_1, b.field_2); // C++
```

And the following Rust code would perform unintended mutations to `field_2`:

```rust
let mut b1: B = ...;
let mut b2: B = ...;
// This actually swaps field_2!
std::mem::swap(&mut b1.field_1(), &mut b2.field_1());
```

### C++20

In C++17 and earlier, there was only one way to create a potentially-overlapping
object: inheritance (["EBO"](https://en.cppreference.com/w/cpp/language/ebo)).
Making inheritable types non-`Unpin` could have removed or mitigated the risk of
overlapping objects in C++17 and below.

However, as of C++20, **any** object can alias another in the tail padding.
C++20 introduced `[[no_unique_address]]`, which makes tail padding available for
reuse for any type. Since `[[no_unique_address]]` may be used fairly extensively
in library code (it has no negative effects in C++), we can't assume that it
does not exist.

In modern C++, `final` types are not much safer than other types. One must be
careful **when creating Rust references**, to ensure that those Rust references
do not contain data in their tail padding, or otherwise alias, and there is no
way to guarantee this at the type level.
