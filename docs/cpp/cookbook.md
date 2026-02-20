# C++ Bindings Cookbook

This document presents a collection of techniques for creating Rust bindings for
C++ libraries.

These techniques are often *workarounds* for gaps in what Crubit can do. Expect
the recommended practices to evolve over time, as Crubit's capabilities expand!

> BEST PRACTICE: The tips below describe deviations from typical C++ style. (If
> typical C++ style worked, you wouldn't need a cookbook.) When you deviate from
> typical C++ style, **document why**, and try to keep changes limited in scope,
> close to the interop boundary.
>
> If possible, solve the same problem while staying within more typical C++
> style. For example: you may be able to
> [add `ABSL_ATTRIBUTE_TRIVIAL_ABI`](#trivial_abi) to a type you control,
> instead of [boxing the type in a pointer](#boxing).

## Making types Rust-movable {#rust_movable}

As described in crubit.rs/cpp/classes_and_structs#rust_movable, types have a
more pleasant API when they are Rust-movable.

This can happen for a couple of easily fixable reasons, described in
subsections:

*   The type defines a destructor or copy/move constructor / assignment
    operator. If it is in-principle still Rust-movable, and these functions do
    not care about the address of the object in memory, then the type can be
    [annotated with `ABSL_ATTRIBUTE_TRIVIAL_ABI`](#trivial_abi)
*   The type has a field which is not rust-movable. In that case, the field can
    be [boxed in a pointer](#boxing).

There are *other* reasons a type can become non-Rust-movable, which do not have
these easy fixes described below. For example, virtual methods, or
non-Rust-movable base classes. For those, your only option is the hard option of
more radically restructuring your code to avoid those patterns.

### `ABSL_ATTRIBUTE_TRIVIAL_ABI` {#trivial_abi}

crubit.rs/cpp/cookbook#trivial_abi

One of the ways a type can become non-Rust-movable is if it has a copy/move
constructor / assignment operator, or a destructor. In that case, Clang will
assume that it cannot be trivially relocated, **unless** it is annotated with
[`ABSL_ATTRIBUTE_TRIVIAL_ABI`](https://github.com/abseil/abseil-cpp/blob/master/absl/base/attributes.h#:~:text=ABSL_ATTRIBUTE_TRIVIAL_ABI).

```c++ {.bad}
struct LogWhenDestroyed {
  ~LogWhenDestroyed() {
    std::cerr << "I was destroyed!\n";
  }
};
```

```c++ {.good}
struct ABSL_ATTRIBUTE_TRIVIAL_ABI LogWhenDestroyed {
  ~LogWhenDestroyed() {
    std::cerr << "I was destroyed!\n";
  }
};
```

> WARNING: Only use `ABSL_ATTRIBUTE_TRIVIAL_ABI` if changing the location of an
> object in memory is safe. In particular, if the object is self-referential,
> using `ABSL_ATTRIBUTE_TRIVIAL_ABI` will result in Undefined Behavior (UB).
>
> ```c++ {.bad}
> class SelfReferential {
>  public:
>   SelfReferential(const SelfReferential& other) : x(other.x), x_ptr(&x) {}
>  private:
>   int x = 0;
>   int* x_ptr = &x;
> }
> ```
>
> Types like this, if Rust-moved, will contain invalid pointers. Carefully
> review any code adding `ABSL_ATTRIBUTE_TRIVIAL_ABI`.

### Boxing in a pointer {#boxing}

crubit.rs/cpp/cookbook#boxing

One of the ways a type can become non-Rust-movable is if it has a field, where
the type of that field is not Rust-movable. There is no way to override this:
there is nothing a type can do to make itself Rust-movable if one subobject is
not.

For example, consider a field like `std::string name;`. `std::string` defines a
custom destructor and copy / move constructor/assignment operator, in order to
correctly manage owned heap memory for the string. Because of this, it also is
not Rust-movable. And, at the time of writing, `std::string` currently cannot
use `ABSL_ATTRIBUTE_TRIVIAL_ABI` in any STL implementation. In the case of
libstdc++, for example, `std::string` contains a self-referential pointer: when
the string is small enough, the `data()` pointer refers to the inside of the
string. Rust-moving it would cause the pointer to refer back to the *old*
object, which would cause undefined behavior.

If a struct or class contains a `std::string` as a subobject by value, or any
other non-Rust-movable object, then that struct or class is itself also not
Rust-movable. (If you somehow were able to Rust-move the parent object, this
would also Rust-move the `string`, causing the very same issues.)

Instead, what you can do is change the type of the field, so that it doesn't
contain the problematic type *by value*. Instead, it can hold the
non-Rust-movable type by pointer.

BEST PRACTICE: Except where necessary for better Rust interop, this is **not**
good C++ style. When you use this trick, document why, and try to limit it to
types close to the interop boundary. If possible, instead of boxing `T`, make
`T` itself rust-movable. (This is not easy for standard library types, but if
the type is under your control, it *may* be as easy as adding
`ABSL_ATTRIBUTE_TRIVIAL_ABI`.)

#### `unique_ptr` {#unique_ptr}

NOTE: The following is non-portable, and only works in libc++ with the unstable
ABI. If you aren't sure about whether you are using the unstable ABI, it is likely that you are not, but you might want to check in with your local toolchain maintainer.

If you tightly control your dependencies, you might be using
libc++'s unstable ABI. The unstable ABI, among other things, makes
`unique_ptr<T>` (and `shared_ptr<T>`) Rust-movable. In fact, it is Rust-movable
even if `T` itself is not.

This means that if a particular field is making its parent type
non-Rust-movable, one fix is to wrap it in a `unique_ptr`:

```c++ {.bad}
struct Person {
  std::string name;
  int age;
}
```

```c++ {.good}
struct Person {
  // boxed to make Person rust-movable: crubit.rs/cpp/cookbook#boxing
  std::unique_ptr<std::string> name;
  int age;
}
```

#### Raw pointers {#raw_ptr}

BEST PRACTICE: This should only be used in codebases that do not use a
Rust-movable `unique_ptr` or `unique_ptr` equivalent. Consider wrapping this in
an `ABSL_ATTRIBUTE_TRIVIAL_ABI` type which resembles `unique_ptr`, instead.

<section class="zippy" markdown="1">

When not using libc++'s unstable ABI, the most straightforward way to make a
field Rust-movable is to instead use a **raw** pointer, and delete it in the
destructor (as if it were held by a `unique_ptr`).

```c++ {.bad}
struct Person {
  std::string name;
  int age;
}
```

```c++ {.good}
struct ABSL_ATTRIBUTE_TRIVIAL_ABI Person {
  // Owned, boxed to make Person rust-movable: crubit.rs/cpp/cookbook#boxing
  std::string* name;
  int age;

  ~Person() {
    delete name;
  }
}
```

(Note the use of `ABSL_ATTRIBUTE_TRIVIAL_ABI`: because we added a destructor, we
also need to add `ABSL_ATTRIBUTE_TRIVIAL_ABI` to indicate that the destructor
does not care about the address of `Person`.)

</section>

## Renaming functions for Rust {#renaming}

crubit.rs/cpp/cookbook#renaming

Overloaded functions cannot be called from Rust (yet: b/213280424). To make them
available anyway, you can define new non-overloaded functions with different
names:

```c++ {.bad}
void Foo(int x);
void Foo(float x);
```

```c++ {.good}
void Foo(int x);
void Foo(float x);

// For Rust callers: crubit.rs/cpp/cookbook#renaming
inline void FooInt(int x) {return Foo(x);}
// For Rust callers: crubit.rs/cpp/cookbook#renaming
inline void FooFloat(float x) {return Foo(x);}
```

## Working around blocking bugs in Crubit {#blockers}

Crubit is still in development, and has bugs which can completely stop your work
if Crubit was in the critical path. These can take the form of parsing errors or
crashes when Crubit runs, or else generated bindings which do not compile.

The following workarounds can help get you moving again:

### Disable Crubit on a declaration {#disable-declaration}

If a declaration causes hard failures within Crubit, that declaration alone can
be disabled using the CRUBIT_DO_NOT_BIND attribute macro, defined in
support/annotations.h. This must be paired with an additional entry in
rs_bindings_from_cc/bazel_support/generate_bindings.bzl, recording the name of the item.

To mail the CL performing this change, use <internal link>_manage: add
`AUTO_MANAGE=testing:TGP` to the CL description.

NOTE: By disabling Crubit on this declaration, items which depend on it may
also, in turn, not receive bindings. For example, if it declares a type, then
functions which accept or return that type will also not receive bindings.

### Disable Crubit on a header {#disable-header}

If an entire header is giving problems (e.g. is unparseable), then it can be
removed from consideration by Crubit. Once disabled, Crubit will avoid reading
the header directly, although it is still included via `#include` preprocessor
directives.

Add the target name and header name to `public_headers_to_remove` in
rs_bindings_from_cc/bazel_support/rust_bindings_from_cc_aspect.bzl.
See the example in
rs_bindings_from_cc/test/disable/disable_header/.

To mail the CL performing this change, use <internal link>_manage: add
`AUTO_MANAGE=testing:TGP` to the CL description.

> NOTE: By disabling Crubit on this header, items which are defined in that
> header will not receive bindings. For example, this means that functions which
> use types defined in that header will also not get bindings, even if the
> function was defined in a header that was not disabled.
>
> When possible, it's preferable to use a smaller fix. For example, if the same
> header is owned by two targets, it's preferable to move the header into a
> third target, depended on by both. That way, functions which use types defined
> in that header will still get bindings, in both targets.

## Making backwards-compatible changes to C++ code with Rust callers {#compatibility}

Because Rust does not have all the features C++ does, some changes that do not
"usually" break a C++ caller can break a Rust caller completely. For example,
most C++ callers will not be broken by adding a new overload to an overload set,
but that does break Rust callers, because Rust doesn't have overloading.

Most of the time, you can think of Rust users (and Crubit) as the most
pathological possible C++ caller, relying on all the details of your API.
(Though there are some things that are *truly* compatibility breaking.)

The following attempts to be an exhaustive list of the things you might expect
to be able to do if you only have C++ callers, but cannot do with Rust callers.

### Making a type non-Rust-movable {#compatibility-non_movable}

If a type is made non-Rust-movable, when it was previously Rust movable, this
will break all but the most trivial of Rust callers.

Specifically, the following operations are compatibility-breaking if the type
was Rust-movable:

* Adding a virtual method.
* Adding a copy constructor, move constructor, or destructor, without specifying `ABSL_ATTRIBUTE_TRIVIAL_ABI`.
* Adding a non-Rust-movable field (such as `std::string`).

```c++ {.no-copy}
struct Before {
  int x;
};
struct After {
  ~After() {}

  int x;
  std::string y;
};

Before ReturnBefore();
After ReturnAfter();
```

```rust {.no-copy}
#[derive(Copy, Clone)]
struct Before {...}
struct After {...}

pub fn ReturnBefore() -> Before {...}
pub fn ReturnAfter() -> Ctor![After] {...}
```

**Fix:** Preserve Rust-movability, as described above in
[Making types Rust-movable](#rust_movable)

```c++ {.good, .no-copy}
class ABSL_ATTRIBUTE_TRIVIAL_ABI CompatibleAfter {
  ~After() {}

  int x;
  std::unique_ptr<std::string> y;
};
```

### Adding or removing virtual destructors {#compatibility-polymorphic}

If a type's destructor is made virtual, or was previously virtual and now
becomes nonvirtual, this changes how Rust callers invoke destruction. For
example, `unique_ptr<T>` becomes `unique_ptr_dyn<T>` if `T` has a virtual
destructor.

```c++ {.no-copy}
class Before {};
class After {~After(){}};

std::unique_ptr<Before> ReturnBefore();
std::unique_ptr<After> ReturnAfter();
```

```rust {.no-copy}
pub fn ReturnBefore() -> cc_std::std::unique_ptr<Before> {...}
pub fn ReturnAfter() -> cc_std::std::unique_ptr_dyn<After> {...}

let _: cc_std::std::unique_ptr<_> = ReturnAfter();  // error[E0308]: mismatched types
```

**Fix:** Migrate callers that use heap allocated types such as `unique_ptr`.

```rust {.good .no-copy}
let _: cc_std::std::unique_ptr_dyn<_> = ReturnAfter();
```

### Adding an overload {#compatibility-overload}

Except for constructors and operators, adding an overload to a C++ function will
cause it to no longer receive bindings, unless the overloads are given distinct
names.

```c++ {.no-copy}
void Before();

void After();
void After(int);
```

```rust {.no-copy}
pub fn Before() {...}
// No bindings for After.
```

**Fix:** use `CRUBIT_RUST_NAME("NewName")` to specify a per-overload name.

```c++ {.good .no-copy}
void CompatibleAfter();

CRUBIT_RUST_NAME("CompatibleAfterInt")
void CompatibleAfter(int);
```

```rust {.good .no-copy}
pub fn CompatibleAfter() {...}
pub fn CompatibleAfterInt(_: c_int) {...}
```

### Changing types with implicit conversions {#compatibility-conversions}

In C++, it is often (but not always) compatible to change from, for example,
accepting an `int32_t` to a `int64_t`, or similar. This is not a
compatible change in Rust, because implicit conversions do not exist.

```c++ {.no-copy}
void Before(int32_t);
void After(int64_t);
```

```rust {.no-copy}
pub fn Before(_: i32) {...}
pub fn After(_: i64) {...}

let x: i32 = ...;
After(x);  // error[E0308]: mismatched types
```

**Fix:** migrate callers.

```rust {.good .no-copy}
After(x.into())
```

### Unsupported features {#compatibility-unsupported}

Use of other unsupported features of C++, such as `decltype` or various
unsupported clang attributes, will also cause breakages for Rust callers.

This is an open-ended failure category, and includes, at minimum:

* Changing from a concrete function to a template function
* Changing from an ordinary class to a type alias to a template specialization
* Unrecognized attributes
* Types that do not have Crubit support yet

For instance:

```c++ {.no-copy}
int Before(int x) {...}

[[clang::very_interesting_advanced_attribute]]
auto After(int x) -> decltype(x + std::declval<int>()) {...}
```

```rust {.no-copy}
pub fn Before(_: c_int) -> c_int {...}
// No bindings for After
```

**Fix:** avoid the use of these features, or wrap with Rust-only functions that
avoid the use of these features.

```c++ {.good .no-copy}
int CompatibleAfter(int x) {return After(x);}
```

```rust {.good .no-copy}
pub fn After(_: c_int) -> c_int {...}
```
