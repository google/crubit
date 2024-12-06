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

As described in <internal link>/cpp/classes_and_structs#trivially_relocatable, types
cannot be passed by value in Rust unless they are Rust-movable, or, in C++
terminology, trivially relocatable.

This can happen for a couple of easily fixable reasons, described in
subsections:

*   The type defines a destructor or copy/move constructor / assignment
    operator. If it is in-principle still trivially relocatable, and these
    functions do not care about the address of the object in memory, then the
    type can be [annotated with `ABSL_ATTRIBUTE_TRIVIAL_ABI`](#trivial_abi)
*   The type has a field which is not rust-movable. In that case, the field can
    be [boxed in a pointer](#boxing).

There are *other* reasons a type can become non-trivially-relocatable, which do
not have these easy fixes described below. For example, virtual methods, or
non-trivially-relocatable base classes. For those, your only option is the hard
option of more radically restructuring your code to avoid those patterns.

### `ABSL_ATTRIBUTE_TRIVIAL_ABI` {#trivial_abi}

<internal link>/cpp/cookbook#trivial_abi

One of the ways a type can become non-trivially-relocatable is if it has a
copy/move constructor / assignment operator, or a destructor. In that case,
Clang will assume that it cannot be trivially relocated, **unless** it is
annotated with `ABSL_ATTRIBUTE_TRIVIAL_ABI`.

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

One of the ways a type can become non-trivially-relocatable is if it has a
field, where the type of that field is not trivially relocatable. There is no
way to override this: there is nothing a type can do to make itself trivially
relocatable if one subobject is not.

For example, consider a field like `std::string name;`. `std::string` defines a
custom destructor and copy / move constructor/assignment operator, in order to
correctly manage owned heap memory for the string. Because of this, it also is
not trivially relocatable / rust-movable. And, at the time of writing,
`std::string` currently cannot use `ABSL_ATTRIBUTE_TRIVIAL_ABI` in any STL
implementation. In the case of libstdc++, for example, `std::string` contains a
self-referential pointer: when the string is small enough, the `data()` pointer
refers to the inside of the string. Rust-moving it would cause the pointer to
refer back to the *old* object, which would cause undefined behavior.

If a struct or class contains a `std::string` as a subobject by value, or any
other non-trivially-relocatable object, then that struct or class is itself also
not trivially relocatable. (If you somehow were able to Rust-move the parent
object, this would also Rust-move the `string`, causing the very same issues.)

Instead, what you can do is change the type of the field, so that it doesn't
contain the problematic type *by value*. Instead, it can hold the
non-trivially-relocatable type by pointer.

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
`unique_ptr<T>` trivially relocatable (in C++) and Rust-movable (in Rust). In
fact, it is trivially relocatable even if `T` itself is not.

This means that if a particular field is making its parent type
non-trivially-relocatable, one fix is to wrap it in a `unique_ptr`:

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

BEST PRACTICE: This should only be used in codebases that do not use a trivially
relocatable `unique_ptr` or `unique_ptr` equivalent. Consider wrapping this in
an `ABSL_ATTRIBUTE_TRIVIAL_ABI` type which resembles `unique_ptr`, instead.

<section class="zippy" markdown="1">

When not using libc++'s unstable ABI, the most straightforward way to make a
field trivially relocatable is to instead use a **raw** pointer, and delete it
in the destructor (as if it were held by a `unique_ptr`).

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
