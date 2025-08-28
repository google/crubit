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

As described in <internal link>/cpp/classes_and_structs#rust_movable, types cannot be
passed by value in Rust unless they are Rust-movable.

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

<internal link>/cpp/cookbook#trivial_abi

One of the ways a type can become non-Rust-movable is if it has a copy/move
constructor / assignment operator, or a destructor. In that case, Clang will
assume that it cannot be trivially relocated, **unless** it is annotated with
`ABSL_ATTRIBUTE_TRIVIAL_ABI`.

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

<internal link>/cpp/cookbook#boxing

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
`unique_ptr<T>` Rust-movable. In fact, it is Rust-movable even if `T` itself is
not.

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
  // boxed to make Person rust-movable: <internal link>/cpp/cookbook#boxing
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
  // Owned, boxed to make Person rust-movable: <internal link>/cpp/cookbook#boxing
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

<internal link>/cpp/cookbook#renaming

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

// For Rust callers: <internal link>/cpp/cookbook#renaming
inline void FooInt(int x) {return Foo(x);}
// For Rust callers: <internal link>/cpp/cookbook#renaming
inline void FooFloat(float x) {return FooFloat(x);}
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
