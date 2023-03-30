# Lifetime Annotations for C++

[TOC]

Summary: We propose a scheme for annotating lifetimes for references and
pointers in C++.

Note: This is a living document that is intended to always reflect the most
current semantics and syntax of the lifetime annotations.

## Introduction {#introduction}

This document proposes an attribute-based annotation scheme for C++ that
describes object lifetime contracts. Lifetime annotations serve the following
goals:

*   They allow relatively cheap, scalable, local static analysis to find many
    common cases of heap-use-after-free and stack-use-after-return bugs.
*   They allow other static analysis algorithms to be less conservative in their
    modeling of the C++ object graph and potential mutations done to it.
*   They serve as documentation of an API’s lifetime contract, which is often
    not described in the prose documentation of the API.
*   They enable better C++/Rust and C++/Swift interoperability.

The annotation scheme is inspired by Rust lifetimes, but it is adapted to C++ so
that it can be incrementally rolled out to existing C++ codebases. Furthermore,
the annotations can be automatically added to an existing codebase by a tool
that [infers the annotations](lifetimes_static_analysis.md) based on the current
behavior of each function’s implementation.

While the annotation scheme can express a large subset of Rust’s lifetime
semantics, we have omitted some constructs that we do not expect to be necessary
for our purposes. For example,
[lifetime bounds](https://doc.rust-lang.org/reference/trait-bounds.html#lifetime-bounds)
(e.g. `'a: 'b` or `T: 'a`) may be needed rarely enough that we can do without
them, and
[higher-ranked trait bounds](https://doc.rust-lang.org/reference/trait-bounds.html#higher-ranked-trait-bounds)
(e.g. `where for<'a> F: Fn(&'a i32)`) are possible only for function types,
which is what they are usually needed for.

We are aware of two existing schemes for annotating lifetimes and flagging
lifetime violations in C++; we describe them in the sections
“[Alternative considered: `[[clang::lifetimebound]]`](#alternative-considered-clang-lifetimebound)”
and
“[Alternative considered: P1179 / `-Wdangling-gsl`](#alternative-considered-p1179-wdangling-gsl)”
below. Both of these schemes have limitations that make them unsuitable for our
purposes. We plan to enable our lifetime analysis to understand the existing
annotations by translating them into our annotation syntax internally (where
possible).

## Proposal {#proposal}

### Examples {#examples}

To give a feel for how the annotations work in practice, we will first show some
examples.

Here is a simple example:

```c++
const std::string& [[lifetime(a)]] smaller(
    const std::string& [[lifetime(a)]] s1,
    const std::string& [[lifetime(a)]] s2) {
  if (s1 < s2) {
    return s1;
  } else {
    return s2;
  }
}
```

The annotation states that both s1 and `s2` may be referred to by the return
value of the function. This implies that the lifetime of the return value is the
shorter of the lifetimes of `s1` and `s2`. In Rust, this example would be
expressed as follows:

```rust
pub fn smaller<'a>(s1: &'a String, s2: &'a String) -> &'a String;
```

Note how the syntax is broadly similar. The main difference is that, unlike in
Rust, our proposal does not require lifetimes to be declared.

A lifetime annotation placed after a member function refers to the lifetime of
the object the member function is called on:

```c++
struct string {
  // The returned pointer should not outlive ``*this``.
  const char *[[lifetime(a)]] data() const [[lifetime(a)]];
};
```

Similar to Rust, `[[lifetime(static)]]` is used to denote a static lifetime. A
common pattern is for a class to have a static function returning a reference to
some default value:

```c++
class Options final {
 public:
  // ...
  static const Options &[[lifetime(static)]] DefaultOptions();
  // ...
};
```

The attribute can be applied to references that appear inside a more complex
type expression. For example:

```c++
const std::vector<const A *[[lifetime(static)]]> &[[lifetime(static)]]
get_static_as();
```

This expresses that both the reference to the vector and the pointers to the
`A`s contained inside it have static lifetimes.

This roughly corresponds to the following in Rust (with the difference that,
unlike C++ pointers, Rust references cannot be null):

```rust
fn get_static_as() -> &'static CxxVector<&'static A>;
```

### Lifetimes {#lifetimes}

Lifetimes are associated with certain types that we call *reference-like types*.
A reference-like type is one of the following:

*   A pointer (except pointers to functions and pointers to members)
*   A reference (except references to functions)
*   A user-defined type that has been annotated as having lifetime parameters.
    (We will explain user-defined reference-like types in detail in a later
    section.)

The reason that pointers to functions and references to functions do not have
lifetimes is to be consistent with Rust, where `fn` types do not have lifetimes
either. In C++, the function that a pointer or reference refers to almost always
exists for the duration of the program execution. There are some exceptions,
such as functions created by a JIT compiler or functions in plugins loaded and
unloaded at runtime. Such functions may be destroyed before the program exits,
but we consider them to be unusual enough that we don't support annotating their
lifetimes.

Pointers to members don't have lifetimes because they aren't pointers in the
narrower sense. A pointer to member doesn't refer to a specific object in
memory; rather, it can be used to refer to a specific member of any object of a
given type. In implementation terms, a pointer to member is not an address but
an offset.

Lifetimes are annotated using the new attribute `lifetime`[^1]. The attribute
takes one or several lifetime names as arguments.
[Appendix A](#appendix-a-lifetime-attribute-specification) contains a formal
description of the attribute syntax.

For brevity, lifetimes may be implicitly inferred in some situations; this is
referred to as *lifetime elision*, and we describe the specific rules for this
later.

There are two lifetime names with special meaning:

*   `static`: A lifetime that lasts for the duration of the program.
*   `unsafe`: A lifetime that cannot otherwise be represented correctly using
    lifetime annotations. We will discuss the semantics of an unsafe lifetime in
    more detail below.

In addition, there are two types of lifetimes that cannot be named in a lifetime
attribute but that are implicitly associated with reference-like types in
certain situations:

*   *Local lifetime*: The lifetime of a pointer to a variable with automatic
    storage duration.
*   *Unknown lifetime*: A lifetime that has not been annotated and cannot be
    implicitly inferred. \
    The concept of unknown lifetimes is important because it allows us to
    migrate a codebase to lifetime annotations incrementally. Tools that verify
    lifetime correctness should assume that operations involving unknown
    lifetimes are lifetime-correct; this avoids generating large numbers of
    nuisance errors for code that has not been annotated yet. Note that this
    makes unknown lifetimes fundamentally different from unsafe lifetimes.

We call static, unsafe, local, and unknown lifetimes *constant lifetimes*. We
call all other lifetimes *variable lifetimes*; this reflects the fact that they
may be substituted by other lifetimes.

The `lifetime` attribute can be applied to reference-like types in function
signatures, variable declarations (including member variable declarations),
alias declarations, and to user-defined reference-like types when referring to
static members of such types. The sections below give details on how the
attribute can be applied to these constructs and what the semantics are in each
case.

Note that, unlike in Rust, lifetimes are not part of the type. For the purposes
of C++ semantics (e.g. function overloading), two types that differ only in
their lifetime annotations are considered the same type. This is by design: We
don’t want to change the semantics of existing code by adding lifetimes, and
this is one of the reasons we have chosen to use C++ attributes; the C++
standard allows compilers to ignore attributes they don’t know, which implies
that they have no effect on the C++ semantics.

### Lifetime-correctness {#lifetime-correctness}

The implementation of a function must be lifetime-correct. This section explains
what that means.

Most expressions propagate lifetimes in ways that are straightforward. We will
therefore explain lifetime-correctness rules only for those cases that are
non-trivial.

Dereferencing a pointer or accessing the value referred to by a reference is
lifetime-correct in exactly the following cases:

*   If its lifetime is static or a variable lifetime
*   If its lifetime is local and the access happens during the lifetime of the
    corresponding local variable.

Dereferencing a pointer with unknown lifetime or accessing the value referred to
by a reference with unknown lifetime is not lifetime-correct, but tools should
not emit lifetime verification errors in these cases.

`operator new` returns a pointer with unsafe lifetime. `operator delete` takes a
pointer parameter that has unsafe lifetime.

Initializing or assigning an object of reference-like type with another object
is always correct if the lifetimes of the two objects are the same.

In addition, there are a number of cases where it is permissible to initialize
or assign an object of reference-like type with another object that has
different lifetimes. We call such an operation a lifetime conversion.

To define lifetime correctness of conversions, we first need to define what it
means for one lifetime to *outlive* another:

*   Any lifetime outlives itself.
*   The `static` lifetime outlives any variable or local lifetime.
*   Any variable lifetime outlives any local lifetime.
*   A local lifetime `local1` outlives another local lifetime `local2` if the
    object associated with `local1` outlives the object associated with `local2`
    according to C++’s lifetime rules.
*   The unsafe lifetime does not outlive any lifetime except itself, and no
    other lifetime outlives the unsafe lifetime.
*   The unknown lifetime does not outlive any lifetime except itself, and no
    other lifetime outlives the unknown lifetime. However, tools should not emit
    lifetime verification errors for lifetime conversions involving unknown
    lifetimes.

Note that no variable lifetime `a` outlives any other variable lifetime `b`; our
annotation scheme does not permit specifying lifetime bounds between lifetimes
in the way that
[Rust does](https://doc.rust-lang.org/reference/trait-bounds.html#lifetime-bounds).

Here are the rules for the correctness of lifetime conversions:

*   Lifetime-converting a non-const pointer of type `T_from
    *[[lifetime(l_from)]]` to type `T_to *[[lifetime(l_to)]]` is
    lifetime-correct if and only if
    *   `l_from` outlives `l_to`, and
    *   any lifetimes in `T_from` and `T_to` are identical.
*   Lifetime-converting a const pointer of type `T_from * const
    [[lifetime(l_from)]]` to type `T_to * const [[lifetime(l_to)]]` is
    lifetime-correct if and only if
    *   `l_from` outlives `l_to`, and
    *   converting `T_from` to `T_to` is lifetime-correct.
*   The rules for converting references are analogous to those for converting
    pointers.
*   An object of a class `T` with lifetime parameters may not be converted to an
    object of the same class `T` but with different lifetime parameters; see
    also the sections on [variance](#variance) and
    [special member functions](#special-member-functions).

We will describe the lifetime-correctness rules for certain other constructs in
the specific sections that deal with those constructs below.

### `lifetime_cast` {#lifetime_cast}

To permit building safe abstractions on top of APIs that use unsafe lifetimes,
we provide a way to cast unsafe lifetimes to safe lifetimes and vice versa using
a function template called `lifetime_cast`[^2]. A `lifetime_cast` is similar to
C++ cast operations such as `const_cast` and `static_cast` but may only be used
to change lifetimes.

Obviously, code that uses `lifetime_cast` must guarantee that the operation is
actually lifetime-correct, i.e. that there is no risk of a use-after-free. Like
unsafe code in Rust, uses of `lifetime_cast` should therefore be carefully
reviewed and constrained to small parts of the codebase.

`lifetime_cast` is a function template defined suitably such that the call
`lifetime_cast<T>(e)` evaluates to `e` and does not perform any copy or move
operations. Tools will assume that the lifetimes of the result are those
specified in the template argument for `T`. Apart from lifetime attributes, `T`
must be the same as `decltype(e)`.

A typical use case for `lifetime_cast` would be building a container such as
`std::vector` on top of raw memory allocation primitives such as `operator new`.
For example, one of the constructors for a vector might look like this:

```c++

template <class T>
void vector<T>::vector(size_t size) [[lifetime(a)]]
:   size_(size), capacity_(size), data_(lifetime_cast<T *[[lifetime(a)]]>(new
    T[size])) {}
```

### Concise syntax using macros {#concise-syntax-using-macros}

Even with lifetime elision, there is a potential concern that the annotations
will introduce excessive clutter. A lifetime in Rust typically requires only two
characters, e.g. `'a`. In contrast, the attribute proposed above,
`[[lifetime(a)]]`, requires at least 15 characters, or more if the attribute is
scoped inside a namespace.

To reduce verbosity, we suggest providing a macro with a short name that expands
to the actual lifetime attribute. The single-character macro name “`$`” is not
in widespread use in many codebases[^3]; a codebase maintainer would obviously
want to consider carefully what to use it for, but we think lifetimes could be a
worthwhile use. In addition to a general <code>$(<em>lifetime</em>)</code>
macro, we could also define lifetime macros <code>$a</code> through
<code>$z</code> to allow an even more concise annotation. As an example, this is
what the <code>smaller()</code> example from the beginning would look like with
this concise syntax:

```c++
const std::string &$a smaller(
    const std::string &$a s1,
    const std::string &$a s2);
```

For a more extensive example, see
[appendix B](#appendix-b-std-string_view-with-lifetime-annotations), which shows
what `std::string_view` would look like with these annotations.

Every codebase can of course define its own macro shortcuts that work within the
context of the codebase. A more traditional and still concise macro name would
be `LT`, with additional macros `LT_A` through `LT_Z` for concise single-letter
lifetimes.

For brevity, in the examples that follow, we will use the `$` convention.

### Pointers and References {#pointers-and-references}

As already noted, pointers and references can be annotated with a lifetime,
which specifies the lifetime of the object the pointer or reference refers to
(the *pointee*). The lifetime of the pointee must outlive the lifetime of the
pointer or reference itself.

For example, let’s look at the example of a double pointer `int * $a * $b`. The
annotation `$b` on the outer pointer specifies the lifetime of the inner pointer
of type `int *`; the annotation `$a` on the inner pointer specifies the lifetime
of the int. When these lifetime variables are substituted with constant
lifetimes, the lifetime substituted for `$a` must outlive the lifetime
substituted for `$b`. This ensures that the `int` lives for at least as long as
the `int *` pointer that refers to it.

### Functions {#functions}

Lifetime attributes may be placed in the parameter types and return type of a
function or function type. In addition, for non-static member functions, a
lifetime attribute may be placed after the function declaration to describe the
lifetime of the object the member function is called on, i.e. the lifetime of
the implicit `this` parameter.

If a translation unit contains multiple declarations of the same function
(including its definition), the lifetime attributes in all declarations must be
the same.

As in Rust, a function is considered to be parameterized by the lifetimes that
appear in its signature. To express this, a `lifetime_param` attribute
containing the variable lifetime parameters may be placed in front of the
function definition, like this:

```c++
[[lifetime_param(a)]]
int *$a ReturnPtr(int *$a p) {
  return p;
}
```

However, for brevity, this `lifetime_param` attribute may and should be left out
in most cases. The exception to this is when the signature of the function
contains a function type that itself contains lifetimes; in this case, a
`lifetime_param` attribute must be added to disambiguate whether the lifetime
should be considered a parameter of the function type or the function. For
example:

```c++
// Lifetime $a is a parameter of the function type int*(int*).
void AddCallback(std::function<int *$a(int *$a) [[lifetime_param(a)]]> f);

// Lifetime $a is a parameter of the function AddCallback().
[[lifetime_param(a)]]
void AddCallback(std::function<int *$a(int *$a)> f, int *$a p);
```

Lifetime parameters on function types are analogous to
[higher-ranked trait bounds](https://doc.rust-lang.org/reference/trait-bounds.html#higher-ranked-trait-bounds)
in Rust; unlike Rust, however, we only allow this concept in the context of
function types, which is where it is typically required.

TODO: Show an example where we're passing a pointer to a local variable into the
callback and discuss how this is allowed in the HRTB case but not the other
case.

Lifetime-converting a function pointer `from` to a function pointer `to` of the
same type but with different lifetimes is lifetime-correct if `from` has either
the same lifetimes as `to` or lifetimes that are more permissive. This means
that we must be able to substitute the lifetime parameters of `from` with
lifetime parameters of `to` such that:

*   Every parameter of `to` is lifetime-convertible to the corresponding
    parameter of `from`. (Note the direction of the conversion, which is
    reversed from what one might initially expect. The idea is that `from` needs
    to be able to stand in for `to`, so we need to be able to convert the
    parameters of `to` to the parameters of `from`.[^4])

*   The return type of `from` is lifetime-convertible to `to`.

Similarly, a virtual member function `Derived::f` that overrides a base class
function `Base::f` must have either the same lifetimes or lifetimes that are
more permissive. This means that there we must be able to substitute the
lifetime parameters of `Derived::f` with lifetime parameters of `Base::f` such
that:

*   Every parameter of `Base::f` is lifetime-convertible to the corresponding
    parameter of `Derived::f`.
*   The return type of `Derived::f` is lifetime-convertible to `Base::f`.

A function call is lifetime-correct if the lifetime parameters of the callee can
be substituted by lifetimes from the caller in such a way that converting all
arguments to the respective parameter lifetimes is lifetime-correct. If no such
substitution can be found, the function call is not lifetime-correct.

Here is an example that illustrates how this works:

```c++
void copy_ptr(int *$x from, int *$x *$y to) {
  *to = from;
}

int *$a return_ptr(int *$a p) {
  int* copy;
  copy_ptr(p, &copy);
  return copy;
}
```

First of all, the `copy` pointer is inferred to have lifetime `$a` because it is
used in the return statement. Let’s use the name `$local1` for the lifetime of
the `copy` variable itself.

Now let’s look at the call to `copy_ptr`. If we make the substitutions `$x` =
`$a` and `$y` = `$local1`, we see that the lifetimes of the arguments are
identical to those of the parameters, so it is trivially correct to
lifetime-convert them.

Assume now that `return_ptr` had been declared with different lifetimes for its
parameter and return type:

```c++
int *$a return_ptr(int *$b p) {
  int* copy;
  copy_ptr(p, &copy);  // Error, not lifetime-correct.
  return copy;
}
```

Again, the `copy` pointer has lifetime `$a`. If we choose the substitution
`$x` = `$a`, we can lifetime-convert the second argument but not the first
argument (we need an i`nt *$a` but we have an `int *$b`). If we choose `$x` =
`$b`, we can lifetime-convert the first argument but not the second argument (we
need an `int *$b *` but we have an `int *$a *`).

Because there is no substitution we can make for `$x` that allows a
lifetime-correct conversion of the arguments of `copy_ptr` to the respective
parameter lifetimes, the call is not lifetime-correct.

### Lifetime elision {#lifetime-elision}

As in Rust, to avoid unnecessary annotation clutter, we allow lifetime
annotations to be elided (omitted) from a function signature when they conform
to certain regular patterns. Lifetime elision is merely a shorthand for these
regular lifetime patterns. Elided lifetimes are treated exactly as if they had
been spelled out explicitly; in particular, they are subject to lifetime
verification, so they are just as safe as explicitly annotated lifetimes.

We adopt the same
[lifetime elision rules](https://doc.rust-lang.org/reference/lifetime-elision.html)
as Rust. We will expand on the rationale for this below, but first let us
present the rules.

We call lifetimes on parameters *input lifetimes* and lifetimes on return values
*output lifetimes*. There are three rules:

1.  Each input lifetime that is elided (i.e. not stated explicitly) becomes a
    distinct lifetime.
2.  If there is exactly one input lifetime (whether stated explicitly or
    elided), that lifetime is assigned to all elided output lifetimes.
3.  If there are multiple input lifetimes but one of them applies to the
    implicit `this` parameter, that lifetime is assigned to all elided output
    lifetimes.

If a function signature contains a function type (in a parameter or the return
value), lifetime elision is performed separately for any lifetimes that occur in
this function type, independent of the lifetimes in the surrounding function
signature. Any elided lifetimes within the function type become lifetime
parameters of the function type. See also the discussion of lifetime parameters
on function types in [this section](#pointers-and-references).

Lifetime elision rules have two requirements:

1.  They need to be easy for a programmer to remember and apply.
2.  They should be applicable to as many functions as possible, i.e. they should
    maximize the percentage of functions whose lifetime semantics correspond to
    the elided lifetimes. Put differently, they should minimize the percentage
    of functions which need explicit, non-elided lifetimes.

There is some alignment between these requirements, but some tension too.
Working out what the best set of rules is likely requires quite a bit of
testing. Instead of doing this, we have for the time being adopted the same set
of rules that Rust uses, which presumably have a lot of collective experience
embedded in them. The underlying assumption is that Rust and C++ functions do
similar things with lifetimes in their interfaces; this assumption seems
passable, though surely not perfect. An added benefit of using the Rust rules is
that programmers using both languages don't need to keep two sets of rules in
their head.

Once we have static analysis tooling that can run on real-world codebases, we
may do some tweaking of the lifetime elision rules, but there would need to be
clear benefits to justify giving up commonality with Rust.

Introducing lifetimes to a codebase will have to happen incrementally, and this
requires some additional considerations. During the transition, there will be
some files that have not yet been annotated, and we may indeed decide to exclude
some parts of the code base from annotation permanently. Lifetime elision should
not be applied to files that have not been annotated or verified for lifetime
correctness; instead, the lifetimes should be assumed to be unknown, as
described [above](#lifetimes).

We propose using a pragma or suitable comment string to mark source files where
lifetime elision is allowed, e.g.:

```c++
#pragma lifetime_elision
```

### Static member variables and non-member variables {#static-member-variables-and-non-member-variables}

Static member variable declarations and non-member variable declarations need
not contain lifetime attributes but may do so for clarity.

In general, it may not even be possible to annotate a local variable correctly
with the current lifetime annotation syntax. This happens when a local variable
may refer to objects of different, unrelated lifetimes. Such a situation is
entirely permissible; lifetime inference and verification tools need to deal
with this by using a richer internal representation for the lifetimes of local
variables.

If a variable has static storage duration, all lifetimes in its type are
implicitly assumed to be static. Any manual annotations that are present may
only specify the lifetimes `static` or `unsafe`.

Taking the address of a static member variable or non-member variable yields a
pointer with a lifetime that depends on the variable’s storage duration. If the
variable has static storage duration, the pointer has `static` lifetime. If the
variable has automatic storage duration, the pointer has a local lifetime.

### Classes and non-static member variables {#classes-and-non-static-member-variables}

A class may be annotated with one or several lifetime parameters by placing the
new attribute `lifetime_param` in the class declaration, and a class annotated
in this way is considered to be a reference-like type. All declarations of a
class must be annotated with the same lifetime parameters. (See
[appendix A](#appendix-a-lifetime-attribute-specification) for a formal
description of the attribute syntax.)

When lifetime parameters are substituted with constant lifetime arguments, all
of these lifetime arguments must outlive the lifetime of the object they are
applied to. This is analogous to the corresponding rule for pointers and
references.

Lifetime parameters are necessary when an object of the class contains
references to data that has a different lifetime than the object itself; the
standard C++ types
<code>[std::string_view](https://en.cppreference.com/w/cpp/string/basic_string_view)</code>
and <code>[std::span](https://en.cppreference.com/w/cpp/container/span)</code>
are examples of this.

The lifetime parameters may be used in the declarations of non-static member
functions and non-static member variables of the class.

As an example, here is how parts of `std::string_view` might be annotated[^5]:

```c++
class [[lifetime_param(a)]] string_view {
  string_view(const char *$a data, size_type len)
      : ptr_(data), len_(len) {}


  const char *$a data() const { return ptr_; }

  string_view $a substr(size_t pos, size_t count) const;

private:
  const char *$a ptr_;
  size_t len_;
};
```

All reference-like types in the declaration of a non-static member variable must
be annotated with the lifetimes `static`, `unsafe`, or one of the lifetime
parameters of the class.

If a class contains owning pointers to manually allocated memory, these pointers
will typically be annotated with an `unsafe` lifetime. Collection types such as
`std::vector` are examples of this. Member functions that provide access to the
owned memory will typically perform a `lifetime_cast` to the lifetime of the
owning object. For example, `std::vector::at()` has the lifetime signature `T&
$a std::vector<T>::at(size_type) $a`.

A class is not required to use any of its lifetime parameters; it may declare
lifetime parameters solely for the purpose of associating a lifetime with
objects of the class.

#### Derived classes {#derived-classes}

Derived classes inherit the lifetime parameters of their base classes. It is not
permissible to add lifetime parameters to a derived class; in other words, all
lifetime parameters need to be declared on the base class. If a derived class
has multiple base classes, only one of these base classes may declare lifetime
parameters.

TODO: Having a derived class “silently” inherit the lifetime parameters of its
base classes isn’t great because it doesn't make the lifetime parameters of the
derived class visible at the place where it is defined. We should instead
consider requiring the lifetime parameters to be re-declared.

The motivation for this rule is to cover the case where a call to a virtual
member function in the base class may access member variables of reference-like
type in a derived class. A similar situation exists when casting a pointer from
the base class to the derived class. In both cases, we want all lifetimes that
are relevant to the derived class to be known on the base class.

### Special member functions {#special-member-functions}

Special member functions can be annotated with lifetimes just like other member
functions, but they deserve special attention because they can be implicitly
declared and because they are central to the semantics of C++ value types.

The default constructor and destructor are trivial as they only take a single
reference-like parameter, the implicit `this` parameter, so we will not discuss
them further.

The lifetimes in the copy and move operations for a type `A` without lifetime
parameters are as follows (using `$s` and `$o` as mnemonics for “self” and
“other”):

```c++
A(const A& $o) $s;
A(A&& $o) $s;
A& $s operator=(const A& $o) $s;
A& $s operator=(A&& $o) $s;
```

Conveniently, these are the lifetimes that are implied by lifetime elision, so
they would be omitted in practice.

The implication of these lifetimes is that it is possible to move or assign an
object of type `A` to another object with a different lifetime.

The situation is slightly more complicated for a type with lifetime parameters.
As an example, consider the following class:

```c++
struct [[lifetime_param(p)]] B {
  int* $p p;
};
```

(The special member functions are implicitly defaulted.)

The lifetimes of the special member functions on `B` are as follows:

```c++
B(const B $p & $o) $s;
B(B $p && $o) $s;
B& $s operator=(const B $p & $o) $s;
B& $s operator=(B $p && $o) $s;
```

Note that while the lifetimes of the “self” and “other” objects themselves are
different, their lifetime parameters are the same. This implies that the copy
and move operations cannot extend the lifetime of `B::p`.

The lifetimes above are not the same as those implied by lifetime elision.
Classes with lifetime parameters that use the defaulted copy and move operations
need to add explicitly defaulted definitions for these operations.

### Alias declarations {#alias-declarations}

Alias declarations can declare lifetime parameters in a similar way to classes.
These lifetime parameters can then be used on the right-hand side of the alias
declaration. In addition, any alias declaration, whether it has lifetime
parameters or not, can use the lifetimes `static` and `unsafe` on its right-hand
side.

If an alias declaration is contained inside a class, its right-hand side may
*not* use any lifetime parameters of that class. This is because, in general, an
instance of the alias type has no connection to an instance of the class.

Here is an example for an alias declaration with lifetime parameters, again
using `std::string_view`:

```c++
class [[lifetime_param(a)]] string_view {
public:
  // ...
  using const_iterator [[lifetime_param(i)]] = const char *$i;
  const_iterator $a begin() const;
  const_iterator $a end() const;
  // ...
};
```

Note that the `lifetime_param` attribute comes after the type alias name,
whereas in a class declaration it comes before the class name. This may seem
inconsistent, but the placement is dictated by the C++ grammar.

So far, we have pretended that `string_view` is a class, but it is in fact
itself an alias declaration for `basic_string_view<char>`, and this alias
declaration therefore has a lifetime parameter:

```c++
template <class T> class [[lifetime_param(a)]] basic_string_view {
  // ...
};

using string_view [[lifetime_param(a)]] = basic_string_view<char> $a;
```

The interpretation of this is that `string_view` is a type with a lifetime
parameter `a`, and that this lifetime parameter should be forwarded to the
lifetime parameter of `basic_string_view<char>`.

### Templates {#templates}

A function template or class template may be annotated with `lifetime`
attributes and, in the case of class templates, a `lifetime_param` attribute,
just like a non-template function or class.

Explicit template instantiations may not contain `lifetime` attributes.

Lifetime-correctness of a template may, in general, depend on the template
arguments. A template is lifetime-correct if there exists at least one set of
arguments for which no specialization exists, that do not result in substitution
failure, and for which the specialized template is lifetime-correct.

In general, therefore, lifetimes can only be inferred and verified on a template
instantiation. This implies that inference and verification may need to be done
multiple times if the same template instantiation is used in multiple
translation units. This is slightly unfortunate, but there does not seem to be a
good way around it, and it mirrors the fact that such a template instantiation
is also compiled multiple times.

To the extent that it is possible to infer and verify lifetimes on the template
itself, independent of the template arguments, tooling should do this. In other
words, a lifetime-correctness error should be flagged if there is no set of
template arguments for which the specialized template is lifetime-correct.
Lifetimes should be inferred if they are correct for any template arguments for
which no specialization exists and which do not result in a substitution
failure.

Partial template specializations should be treated the same way as primary
template definitions: Tooling should infer and verify lifetimes on the partial
specialization to the extent that this can be done independent of the template
arguments.

Full template specializations should be treated the same way as non-template
functions and classes: Lifetimes should be inferred and verified on the full
template specialization.

When analyzing code that uses a template for which partial or full
specializations exist, tooling must of course make sure to refer to the correct
specialization.

#### Function templates {#function-templates}

A function template’s type arguments as well as other dependent types may, in
general, be reference-like types. Therefore, when a function template
instantiation is used (either by calling it or by taking its address), tooling
should do the following:

*   Verify the lifetime-correctness of the function template instantiation.
*   Infer lifetimes for all reference-like types in the signature of the
    function template instantiation, except for reference-like types that occur
    in the function template itself and are already annotated with lifetimes
    there.

The lifetimes inferred for the specialized function template should be used when
inferring and verifying lifetimes of functions that use the specialized function
template.

#### Class templates {#class-templates}

The type arguments to a class template may be reference-like types. A class
template that is specialized with reference-like types in this way is itself
considered to be a reference-like type. The specialized class template has a
lifetime parameter for each reference-like type that occurs in the template
arguments; these lifetime parameters are in addition to any lifetime parameters
that are annotated on the class template itself using the `lifetime_param`
attribute. The lifetime parameters associated with a template argument are
implicitly propagated to all uses of that argument in the class template.

TODO: Add a discussion of template template arguments

Lifetimes are assigned to a specialized class template’s lifetime arguments as
for any other reference-like type, i.e. depending on the context in which the
specialized class template is used they may be explicitly annotated, implied by
lifetime elision, or inferred. However, there is a syntactical difference: When
lifetimes are explicitly annotated, they are placed in the template arguments
instead of after the type, as they would be for other lifetime parameters. For
example, here a function that takes a vector of pointers and returns an element
of the vector:

```c++
int* $a get_ith(const std::vector<int* $a>& $b v, size_t i) {
  return v[i];
}
```

TODO: Discuss dependent types.

A member function of a specialized class template need not be lifetime-correct
for all possible assignments of the lifetime parameters associated with the
template arguments.

Instead, we only require that every ODR-use of a member function of a
specialized class template is lifetime-correct for the lifetimes assigned to the
lifetime parameters for that particular use.

A (slightly contrived) example will help to illustrate why these rules are
written the way they are.

```c++
template <class From, class To>
struct Convert {
  To convert(From from) { return from; }
};

void constify(int* [[lifetime(a)]] p,
              const int *[[lifetime(a)]] *[[lifetime(b)]] pp) {
  Convert<int*, const int*> c;
  *pp = c.convert(p);
}
```

The specialized class template `Convert<int*, const int*>` has two lifetime
parameters: one lifetime parameter (which we will call `x`) for the `int*`
template argument, and one lifetime parameter (which we will call `y`) for the
`const int*` template argument.

The `Convert::convert()` member function is not lifetime-correct if we consider
`x` and `y` to be arbitrary variable lifetimes, as it is not lifetime-correct to
lifetime-convert an `int *`with lifetime `x` to a `const int *` with lifetime
`y`.

However, for the use of Convert in the declaration of `c`, we infer that both
`x` and `y` should be substituted by the lifetime `a`. `Convert::convert()` is
lifetime-correct when x and y are substituted in this way.

TODO: Do we need to make this distinction between lifetime parameters and the
lifetimes they are substituted with, or can we make the substitution directly?

### Variance {#variance}

[As in Rust](https://doc.rust-lang.org/nightly/nomicon/subtyping.html), we need
to establish some variance rules for type and lifetime parameters, but the
specific rules differ slightly from Rust.

*   Const references and pointers `const T &` and `const T *` are covariant with
    respect to `T`.
*   Non-const references and pointers `T &` and `T *` are invariant with respect
    to `T`.
*   Class templates are invariant with respect to their type parameters
    (including lifetimes contained in them).
*   All lifetime-parameterized types (classes and alias declarations) are
    invariant with respect to their lifetime parameters.

The last two rules differ from Rust, which
[infers](https://rustc-dev-guide.rust-lang.org/variance.html) the variance of
type and lifetime parameters on user-defined types. Unlike Rust generics, C++
class templates are invariant with respect to their type parameters[^6], and we
want to be consistent with this.

Regarding lifetime parameters on types, we restrict ourselves to invariance for
simplicity. Rust infers the variance of lifetime parameters from the way they
are used in the definition of the type, but in C++, this is impossible to do, at
least on a single-translation-unit basis, as a lifetime-parameterized class may
only be forward-declared in the current translation unit. For simplicity, and
consistency with template parameters, we have therefore decided that lifetime
parameters will always be invariant, as we expect this to be sufficient in
practice. If this turns out to be too limiting, we may need to provide a way of
annotating the variance of lifetime parameters.

## Alternative considered: `[[clang::lifetimebound]]` {#alternative-considered-clang-lifetimebound}

Clang already provides a limited ability to annotate lifetimes with the
[`[[clang::lifetimebound]]` attribute](https://clang.llvm.org/docs/AttributeReference.html#lifetimebound)[^7].
Quoting from the documentation:

> The `lifetimebound` attribute on a function parameter or implicit object
> parameter indicates that objects that are referred to by that parameter may
> also be referred to by the return value of the annotated function (or, for a
> parameter of a constructor, by the value of the constructed object).

If the lifetime annotation is applied to aggregates (arrays and simple structs),
those aggregates are considered to refer to any pointers or references
transitively contained within them.

Here, again, is the `smaller()` example, but annotated with
`[[clang::lifetimebound]]`:

```c++
const std::string& smaller(
    const std::string& s1 [[clang::lifetimebound]],
    const std::string& s2 [[clang::lifetimebound]]);
```

The attribute may also be applied to a member function to indicate that the
lifetime of the return value corresponds to the lifetime of the object. Here is
an example from the `[[clang::lifetimebound]]` documentation:

```c++
struct string {
  // The returned pointer should not outlive ``*this``.
  const char *data() const [[clang::lifetimebound]];
};
```

This is an example of the very common case where a member function returns a
pointer or reference to part of the object, or to another object owned by it.

The `[[clang::lifetimebound]]` attribute provides a way to express lifetimes in
many common scenarios, but it does have its limitations:

*   There is no way to differentiate between different lifetimes.
*   There is no way to annotate a static lifetime.
*   The attribute attaches to function parameters and always implicitly refers
    to the outermost reference-like type[^8]; it is not possible to attach it to
    part of a type (e.g. to the `T *` in a `const std::vector<T *> &`).

*   The single lifetime is implicitly applied to the outermost reference-like
    type in the function’s return type (or the value of the constructed object,
    in the case of a constructor). Again, it is not possible to associate the
    lifetime with inner reference types in the return value (e.g. the `T *` in
    `const std::vector<T *> &`).

*   The lifetime of a constructor parameter can be associated with the lifetime
    of the object being constructed, i.e. with the lifetime of the `this`
    pointer, but this isn’t possible in other member functions. In other words,
    a member function cannot associate the lifetime of a parameter with the
    lifetime of the object the member function is called on.

*   There is no way to add a lifetime parameter to a struct.

## Alternative considered: P1179 / `-Wdangling-gsl` {#alternative-considered-p1179-wdangling-gsl}

The WG21 proposal
[P1179](https://www.open-std.org/jtc1/sc22/wg21/docs/papers/2019/p1179r1.pdf)
describes a static analysis that aims to prevent many common types of
use-after-free. It uses an attribute-based annotation scheme to describe the
lifetime contracts of functions and to annotate user-defined types containing
indirections.

Preliminary implementations of this scheme exist in MSVC and a
[fork](https://github.com/mgehre/llvm-project) of Clang. In addition, Clang
trunk implements statement-local warnings inspired by the scheme, which are
enabled by the on-by-default flag `-Wdangling-gsl`.

The scheme has both advantages and disadvantages compared to the scheme proposed
here:

*   **Advantages**
    *   Can express independent pre- and postconditions for lifetimes, e.g. to
        annotate `std::swap(ptr1, ptr2)`, where the lifetimes of the pointers
        after the call are swapped compared to before the call.
    *   Can diagnose some cases of iterator invalidation.
*   **Disadvantages**
    *   User-defined types can only be annotated as having one of a class of
        fairly specific lifetime semantics (“SharedOwner”, “Owner”, “Pointer”);
        arbitrary annotation of classes with lifetime parameters is not
        possible.
    *   Cannot refer to lifetimes of pointers in template arguments, e.g. no way
        to express `int *$a return_first(const vector<int *$a> &$b v);`
    *   Annotations can be verbose and syntactically removed from the objects
        they refer to.

We believe the limitations of this scheme will restrict its usefulness in the
use cases we are interested in. A more in-depth comparison of P1179 with our
proposed scheme can be found
[here](https://discourse.llvm.org/t/rfc-lifetime-annotations-for-c/61377#lifetime-safety-preventing-common-dangling-wg21-proposal-p1179-wdangling-gsl-18).

## Appendix A: Lifetime attribute specification {#appendix-a-lifetime-attribute-specification}

This appendix describes where lifetime attributes may appear and what arguments
they can take.

### Temporary syntax {#temporary-syntax}

We are currently still experimenting with the exact syntax and semantics for the
lifetime annotations. While we are doing so, we will use the general-purpose
`annotate` and
<code>[annotate_type](https://discourse.llvm.org/t/rfc-new-attribute-annotate-type-iteration-2/61378)</code>
attributes as stand-ins for the new attributes proposed below.

### Attribute definitions {#attribute-definitions}

We introduce two new attributes, `lifetime` and `lifetime_param`. In practice,
these would be scoped to a namespace (probably `clang`), but for ease of
exposition, we assume they are in the global namespace.

#### Attribute `lifetime_param` {#attribute-lifetime_param}

This attribute may be applied to the following:

*   **A class definition** (more formally, it may appear in the
    attribute-specifier-seq of a
    [class-head](https://eel.is/c++draft/class.pre#nt:class-head))
*   **An
    [alias-declaration](http://eel.is/c++draft/dcl.pre#nt:alias-declaration)**
    (specifically, the attribute-specifier-seq following the identifier)

The attribute takes one or more arguments. Each of these arguments must be an
identifier[^9]; each argument defines a lifetime parameter for the corresponding
class.

If the class definition or alias declaration is nested within a class that
itself has a `lifetime_param` attribute, none of the lifetime parameter names of
the outer class may be used as lifetime parameter names on the nested class
definition or alias declaration.

#### Attribute `lifetime` {#attribute-lifetime}

This attribute may be applied to the following:

*   **Types and pointer operators in a function declaration, member function
    declaration, non-static member variable declaration, or alias declaration**
    \
    More formally, within the return type, the
    [trailing-return-type](https://eel.is/c++draft/dcl.dcl#nt:trailing-return-type)
    or the
    [parameter-declaration-clause](https://eel.is/c++draft/dcl.dcl#nt:parameter-declaration-clause)
    of a function declaration or member function declaration, within a
    non-static member variable declaration, or within the
    [defining-type-id](http://eel.is/c++draft/dcl.name#nt:defining-type-id) of
    an alias declaration the attribute may appear:
    *   In the attribute-specifier-seq of a
        [decl-specifier-seq](https://eel.is/c++draft/dcl.spec.general#nt:decl-specifier-seq)
    *   In the attribute-specifier-seq of a
        [type-specifier-seq](https://eel.is/c++draft/dcl.dcl#nt:type-specifier-seq)
    *   In the attribute-specifier-seq of a
        [ptr-operator](https://eel.is/c++draft/dcl.dcl#nt:ptr-operator) (both
        within a [declarator](https://eel.is/c++draft/dcl.dcl#nt:declarator) and
        an
        [abstract-declarator](https://eel.is/c++draft/dcl.dcl#nt:abstract-declarator))
*   **A non-static member function declaration** \
    More formally, within a
    [member-declarator](https://eel.is/c++draft/class.mem.general#nt:member-declarator)
    for a non-static member function, the attribute may appear in the
    attribute-specifier-seq of the
    [parameters-and-qualifiers](https://eel.is/c++draft/dcl.decl.general#nt:parameters-and-qualifiers).

The attribute takes one or more arguments, each of which must be an identifier
or the keyword `static`. We call these identifiers *lifetime names*.

In addition, the following constraints apply:

*   When the `lifetime` attribute is applied to a **type**, the type must be a
    class type or alias declaration whose definition contains a `lifetime_param`
    attribute.

    The `lifetime` attribute must have the same number of arguments as the
    `lifetime_param` attribute on the corresponding class or alias declaration.
    (These arguments define lifetime parameters for the object instance.)

*   When the lifetime attribute is applied to a **pointer operator**, it must
    take exactly one argument. (This defines a lifetime for the object
    referenced by the pointer operator.).

*   When the `lifetime` attribute is applied to a **non-static member function
    declaration**, it must take exactly one argument. (This defines a lifetime
    for the implicit object parameter).

*   Every lifetime name that appears in a **function’s return value** must
    either be `static` or also appear either in

    *   the function’s parameter list, or
    *   the `lifetime` attribute for the implicit object parameter (in the case
        of a non-static member function), or
    *   the `lifetime_param` attribute of the class (in the case of a non-static
        member function).

*   For every **constructor of a class that has a `lifetime_param` attribute**,
    every lifetime name that appears in the `lifetime_param` attribute must
    appear in the constructor’s parameter list.

*   Every lifetime name that appears in a **non-static member variable
    declaration** must either be `static` or one of the lifetime parameters
    declared in a `lifetime_param` attribute on the class containing the member
    variable declaration.

*   Every lifetime name that appears in the defining-type-id of an **alias
    declaration** must either be `static` or one of the lifetime parameters
    declared in a `lifetime_param` attribute on the alias declaration. Note that
    if the alias declaration is nested within a class that also has lifetime
    parameters, those lifetime parameters may *not* appear in the
    defining-type-id of the alias declaration.

## Appendix B: <code>std::string_view</code> with lifetime annotations {#appendix-b-std-string_view-with-lifetime-annotations}

To illustrate how lifetime annotations work on a larger code sample, here is an
annotated version of interesting parts of `std::string_view`. To keep the code
clear, we have omitted basic\_string\_view and simply stamped out `string_view`
for the template arguments used in its definition.

```c++
// Lifetime "s" is mnemonic for "lifetime parameter of string_view"
class LIFETIME_PARAM(s) string_view {
public:
  using const_pointer LIFETIME_PARAM(iter_lifetime) = const char *$(iter_lifetime);
  using const_reference LIFETIME_PARAM(iter_lifetime) = const char &$(iter_lifetime);
  using const_iterator LIFETIME_PARAM(iter_lifetime) = const char *$(iter_lifetime);
  using iterator LIFETIME_PARAM(iter_lifetime) = const_iterator $(iter_lifetime);
  using const_reverse_iterator LIFETIME_PARAM(iter_lifetime) =
      std::reverse_iterator<const_iterator $(iter_lifetime)>;
  using reverse_iterator LIFETIME_PARAM(iter_lifetime) =
      const_reverse_iterator $(iter_lifetime);

  using size_type = size_t;

  static constexpr size_type npos = static_cast<size_type>(-1);

  constexpr string_view() noexcept;
  constexpr string_view(const string_view $s & other) noexcept = default;
  constexpr string_view(const char* $s data, size_type len);

  constexpr const_iterator $s begin() const noexcept;
  constexpr const_iterator $s end() const noexcept;
  constexpr const_reverse_iterator $s rbegin() const noexcept;
  constexpr const_reverse_iterator $s rend() const noexcept;

  constexpr const_reference $s front() const;
  constexpr const_reference $s back() const;

  constexpr const_pointer $s data() const noexcept;

  constexpr const_reference $s operator[](size_type i) const;
  constexpr const_reference $s at(size_type i) const;

  // The annotation cannot express that the lifetime parameter of `this` and
  // `other` are swapped after the call, so we have to be overly restrictive and
  // require `this` and `other` to have the same lifetime parameter.
  constexpr void swap(string_view $s & other) noexcept;

  // Output buffer may have a different lifetime than this string view's data.
  size_type copy(char* buf, size_type n, size_type pos = 0) const;

  // Returned substring has the same lifetime parameter as this `string_view`.
  constexpr string_view $s substr(size_type pos = 0, size_type n = npos) const;

  // `string_view` to compare against does not need to share the same lifetime.
  constexpr int compare(string_view x) const noexcept;

private:
  const char* $s ptr_;
  size_type length_;
};
```

<!-- Footnotes themselves at the bottom. -->

## Notes

[^1]: The attribute will be scoped to some suitable namespace, but for ease of
    exposition we assume here that it is placed in the global namespace.
[^2]: `lifetime_cast` will be placed in a suitable namespace, but for ease of
    exposition, we assume here that it is in the global namespace.
[^3]: “`$`” is not part of the standard set of characters allowed in C++
    identifiers (including macro names), but the C++ standard permits
    implementations to allow additional implementation-defined characters, and
    gcc, Clang, and MSVC allow `$` as an implementation-defined character.
[^4]: More formally, this is because function types are contravariant in their
    parameter types.
[^5]: For simplicity, we are showing `std::string_view` as if it was a
    non-template type.
[^6]: Unless converting constructors and conversion constructors are used to
    simulate variance.
[^7]: This attribute is inspired by the C++ Standards Committee paper
    [P0936R0](http://www.open-std.org/jtc1/sc22/wg21/docs/papers/2018/p0936r0.pdf).
[^8]: Quoting Richard Smith: "The Clang attribute behaves as if each type has
    exactly one associated lifetime, and the attribute says in which cases the
    outermost lifetime of a parameter matches the outermost lifetime of the
    return value.”
[^9]: Note that this automatically disallows the special lifetime name `static`,
    which is allowed in `lifetime` attributes. We make no other constraints on
    identifiers, but codebases that want to use the lifetime annotations for
    C++ / Rust interop may want to enforce a rule that prohibits invalid Rust
    identifiers (e.g. Rust keywords) in the `lifetime_param` and `lifetime`
    attributes..
