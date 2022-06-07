# Static Analysis for C++ Lifetimes

[TOC]

Summary: We describe a static analysis that infers lifetimes in C++ function
signatures.

NOTE: This document describes the approach we are currently pursuing but it is
a) incomplete, and b) out of date. It has become clear that we are still making
changes to the static analysis frequently enough that it does not seem worth
updating a document in parallel with those changes. Once the static analysis
appears reasonably stable, we plan to update this document to describe it.

## Introduction

Lifetime analysis has two goals:

*   Infer lifetime annotations to put in C++ function signatures, using the
    attributes described in this doc.

    TODO: Add a link once we have uploaded a Markdown version of the lifetime
    annotation specification.
*   Verify lifetime-correctness of function bodies.

To infer and verify lifetimes, we perform a
[pointer analysis](https://en.wikipedia.org/wiki/Pointer_analysis)[^1]. For each
pointer or other reference-like type, a pointer analysis determines a points-to
set consisting of the storage locations it may point to.

There are different approaches to pointer analysis that can be classified
according to various properties. The pointer analysis we perform here has the
following properties:

*   *Intraprocedural, context-insensitive*. We analyze each function
    individually and do not take into account how it is called from different
    callsites.
*   *Array-insensitive*. We treat all elements in an array containing a
    reference-like type as having the same lifetime.
*   *Field-insensitive*. We treat member variables of reference-like type as
    having the same lifetime as the object they are contained in (unless they
    carry a lifetime annotation).
*   *Flow-sensitive*. When analyzing a function, we take statement ordering and
    control flow into account. We believe flow sensitivity is important to avoid
    inferring overly restrictive lifetimes and emitting false positive errors.

The pointer analysis we perform is relatively coarse-grained in that we do not
distinguish between different storage locations with the same lifetime;
equivalently, we can say that we identify a storage location merely by its
lifetime.

A points-to set is therefore just a set of lifetimes; a reference-like object is
also simply identified by its lifetime. The state that is tracked during the
analysis is therefore just a mapping from a lifetime (identifying the
reference-like object) to a set of lifetimes (identifying the storage locations
it may point to).

This coarse-grained approach simplifies the analysis and is sufficient for our
purposes because we are only attempting to infer and verify statements about
lifetimes.

## Analysis of a translation unit

We analyze all functions in a translation unit for which we have a definition.

We attempt to analyze all of these functions in topological order so that
callees are analyzed before callers. Where recursion makes this impossible, we
analyze the functions that take part in the recursive cycle in arbitrary order.
We accept that this may make it impossible to infer lifetimes for functions in a
recursive cycle.

## Analysis of a function

As explained in the introduction, we identify an object (often called a storage
location in pointer analysis) merely by its lifetime.

A **points-to set** is therefore simply a set of lifetimes. It represents the
set of objects that a reference-like type or glvalue can be referencing at some
point of execution of the program. We will sometimes refer to the objects in a
points-to set as **pointees**.

We associate each local variable in the function with a different local
lifetime. This serves two purposes: a) It reflects the fact that local variables
do, in general, have different lifetimes, and this is important for lifetime
verification. b) It allows us to associate a different points-to set with
different local variables of reference-like type, and this is required to make
the analysis precise enough.

We perform a data-flow analysis using the
[Clang dataflow framework](https://github.com/llvm/llvm-project/tree/main/clang/include/clang/Analysis/FlowSensitive)
([documentation](https://clang.llvm.org/docs/DataFlowAnalysisIntro.html))
to propagate points-to sets through the function. After the analysis is
complete, we produce lifetime annotations from the points-to sets; if these
lifetime annotations are different from existing annotations (ignoring pure
renamings), we output the new annotations as suggested edits.

The data-flow analysis tracks the following state:

*   For each reference-like object (identified by its lifetime), the points-to
    set of that reference-like object
*   For each expression of reference-like type, the points-to set of the
    expression
*   For each glvalue expression, the points-to set representing the glvalue’s
    referent
*   If the function’s return type is a reference or pointer type, a points-to
    set for the return value

The join operation on points-to sets means taking the union of the two sets.

The initial state for the data flow analysis is produced as follows:

*   Associate each parameter of reference-like type with a points-to set
    containing a new unique regular lifetime representing the pointee.
*   If the pointee is itself of reference-like type, recursively associate that
    pointee with a points-to set containing a new regular lifetime, and so on.

During the analysis, we propagate points-to sets through expressions and update
the points-to sets of reference-like objects.

After the analysis is complete, we obtain lifetime annotations by examining the
points-to sets of all parameters of reference-like type and the return value (if
applicable), descending into pointees that are themselves of reference-like
type.

For every points-to set, we look at the set of lifetimes of its pointees. If
there are multiple lifetimes, they are substituted by a single lifetime. This
lifetime then becomes the lifetime of the corresponding reference or pointer
type in the signature.

Here are some examples:

```c++
void foo(int* from, int** to) {
  // from_pointee (int): '1
  // to_pointee (int *): '2
  // to_pointee_pointee (int): '3
  // from: { from_pointee }
  // to: { to_pointee }
  // to_pointee: { to_pointee_pointee }

  *to = from;
  // to_pointee: { from, to_pointee_pointee }
}
```

TODO: Explain. Also talk about why, after the assignment `*to = from`, we keep
`to_pointee_pointee` in the points-to set and how we can, in some cases,
eliminate it. (Distinguish between scalar and aggregate pointees -- the latter
are arrays, for example. We can only delete existing pointees if *to has a
single pointee and it's scalar.)

```c++
int* target(int* p1, int* p2) {
  // p1_pointee (int): '1
  // p1: { p1_pointee }
  // p2_pointee (int): '2
  // p2: { p2_pointee }
  int** pp;
  if (foo()) {
    pp = &p1;  // pp: { p1 }
  } else {
    pp = &p2;  // pp: { p2 }
  }
  // pp: { p1, p2 }
  int local = 42;
  *pp = &local;  // glvalue on left side is { p1, p2 }, so:
                 // p1: { p1_pointee, local }
                 // p2: { p2_pointee, local }
  return p1; // rval: { p1_pointee, local }
}
```

TODO: Explain. Also mention how this is an example where we have two pointees on the left hand side, so we can't eliminate existing pointees from `p1` and `p2`.

### Function calls

Here is how we handle function calls:

1.  **Create a mapping from callee lifetimes to points-to sets.** For each
    variable lifetime that occurs in the callee's parameter list, find the union
    of all points-to sets in those argument positions to yield a mapping from
    lifetimes to points-to sets.
2.  **Propagate points-to sets to output parameters.** For each lifetime `'l` in
    an invariant argument position, replace the argument's existing points-to
    set with the points-to set established for `'l` in Step 1.
3.  **Step 3: Determine points-to set for the return value.** If the return
    value is of reference-like type with lifetime `'l`, find the points-to set
    established for `'l` in Step 1; this becomes the points-to set for the call
    expression's value.

If the `'static` lifetime occurs in output parameters (i.e. in invariant
position) or in the return value, the callee may be returning references to
pointees that do not occur as inputs to the callee. Therefore, when we encounter
the 'static lifetime in these positions, we create new pointees for the
corresponding outputs.

Here is an example of how this works:

```c++
void copy_ptr(int *'x from, int *'x *'y to) {
  *to = from;
}

int * get_lesser_of(int * arg1, int * arg2) {
  // arg1_pointee (int): '1
  // arg2_pointee (int): '2
  // arg1: { arg1_pointee }
  // arg2: { arg2_pointee }
  int* result = arg2;
    // result: { arg2_pointee }
  if (*arg1 < *arg2) {
    copy_ptr(arg1, &result);
      // &result: { result }
      // 'x pointees: { arg1_pointee, arg2_pointee }
      // result: { arg1_pointee, arg2_pointee }
  }
  return result;
}
```

TODO: Continue exposition

### Virtual member functions

Inferring lifetimes for virtual member functions is complicated by two factors:

*   The lifetimes of the base class member function
    are constrained by the lifetimes of all of its overrides.
*   The definitions of the overrides and the base class function (if it is not
    pure virtual) are typically contained in different translation units, and we
    plan to analyze each translation unit individually.

TODO: Add links to the lifetime annotation specification, which has additional
discussion of both of the points above.

We will describe an approach that can infer and update lifetimes for virtual
member functions progressively, as each translation unit is processed.

If a translation unit contains definitions for multiple overrides, or if it
contains the definition of the the base class function and at least one
override, we analyze these definitions in topological order from base class to
more derived class.

If the definitions are contained in different translation units, we effectively
process them in the same order because we analyze dependencies of a library
before analyzing the library itself, and libraries containing derived classes
generally depend on the library containing the base class.

TODO: The description above implicitly assumes we're talking about the initial
change where we add lifetimes across the codebase. Discuss also how this applies
when people are editing code.

When we encounter the definition of a virtual member function (whether it is the
base class implementation or an override), we first perform lifetime inference
on its implementation, as for any other function, and update the declaration of
the member function in its containing class.

If the function is an override, call it `Derived::f`, we then update the
lifetimes of every base class function `Base::f` that it overrides. (There may
be several if there is a chain of overrides.) We do so as follows:

*   **If the declaration of `Base::f` does not yet contain any lifetime
    annotations**, annotate it with the lifetimes of `Derived::f`. Because we
    process base class functions before derived class functions, this case can
    only occur if `Base::f` is pure virtual.
*   **If the existing lifetimes of `Base::f` are more permissive than the
    lifetimes inferred for `Derived::f`**, perform lifetime substitutions on the
    lifetimes of `Base::f` until they are at most as permissive as those of
    `Derived::f`.
*   **If the existing lifetimes of `Base::f` at most as permissive as the
    lifetimes inferred for `Derived::f`**, do nothing.

TODO: Can we ever get caught in a situation where neither the second nor the
third point above applies? I think we'll always be able to restrict the
lifetimes of `Base::f` until they're compatible with `Derived::f`, but this
needs a formal argument.

TODO: Discuss how the lifetime changes affect callers – may need to process them
again.

TODO: Show an example

### Templates

Templates pose a specific challenge to lifetime analysis:

*   Reference-like types may occur in the template itself as well as in template
    arguments and dependent types.
*   For reference-like types that occur in the template, we wish to infer and
    check lifetimes on the template itself to the greatest extent possible. This
    reflects the fact that, even though C++ templates are not really generics,
    they are often used as if they were. However, the semantics of C++ templates
    pose two difficulties here:
    *   Templates may be specialized, and we must be careful not to apply the
        lifetimes inferred on the primary template to the specialization.
    *   The inferred lifetimes and the lifetime correctness of a template may,
        in general, depend on the template arguments, even if the template
        arguments and dependent types do not contain any reference-like types.
        We show an example of this below.

The lifetime annotation specification defines what the semantics of lifetimes on
templates should be but does not say how they should be implemented. That is the
purpose of this section.

TODO: Add a link to the lifetime annotation specification.

#### Example scenarios

Before we discuss generally how we will analyze templates, let us look at some
scenarios that may occur.

As an example of why we want to be able to analyze templates themselves, let’s
take a look at part of a simplified implementation of `std::vector`:

```c++
template <class T>
class vector {
public:
  vector(const vector& other);

  T* $a begin() $a { return data_; }
  T* $a end() $a { return data_ + size_; }

private:
  T* data_;
  size_t size_;
};
```

We should be able to infer the lifetimes of `begin()` and `end()` from the
template itself. These member functions operate only on pointers to `T`, and the
lifetime behavior of a pointer to `T` is independent of the type `T` itself[^3].

On the other hand, we cannot infer the lifetimes of the copy constructor. It
calls the copy constructor of `T`, and as explained in the lifetime annotation
specification, copy and move operations can have two
different lifetime signatures.

TODO: Add a link to relevant section of the lifetime annotation specification.

Here is another example of how lifetimes can depend on a template argument:

```c++
template <int i>
int* return_ith(int* i0, int* i1) {
  if (i == 0) {
    return i0;
  } else {
    return i1;
  }
}
```

This example is contrived, but it is certainly not implausible that a trait
argument could affect behavior in a similar way.

While these examples do show the limitations of lifetime analysis on templates,
we likely won’t need to do anything subtle to detect them within the analysis.
In the case of the copy constructor of `vector`, we will notice when calling the
copy constructor of `T` that we’re doing member lookup on a dependent type and
that we can’t continue the analysis. In the case of `return_ith()`, we will be
able to analyze the function, but we will conclude that the lifetimes of all
pointers involved are the same. This is more restrictive than the result we
would obtain if we analyzed a template instantiation, but this limitation may be
acceptable.

#### General approach

The constraints described above imply that lifetime analysis of templates need
to proceed in two phases:

*   **Analysis of the template itself**. We first attempt to infer lifetimes on
    the template itself, as well as any partial or full specializations, to the
    extent that the lifetimes do not depend on template arguments. If the
    inferred lifetimes are different from the function’s current (possibly
    elided) lifetimes, we generate a corresponding annotation. If we cannot
    infer lifetimes for the function, we annotate all lifetimes on the function
    as unsafe. This is required to distinguish this case from the situation
    where we *were* able to infer lifetimes and those lifetimes are elided.

    TODO: Is there any alternative to marking the lifetimes unsafe? This isn't
    what we usually use unsafe lifetimes for, but I also don't really want to
    invent yet another syntax.

    Performing lifetime analysis on the template itself, rather than only on
    instantiations, serves two purposes: a) It documents the lifetimes in the
    code, and b) it saves us from having to analyze every instantiation in cases
    where the lifetimes don’t depend on template arguments.

*   **Analysis of template instantiations**. In the following situations, we
    infer lifetimes on a function template instantiation or member function of a
    class template instantiation that is called in the translation unit we are
    analyzing:

    *   If the template itself contains reference-like types but does not
        provide lifetimes for these.
    *   If the template arguments contain reference-like types.

    We use the inferred lifetimes when performing lifetime analysis on the
    callers of these functions, but we obviously cannot produce annotations for
    these inferred lifetimes.

As discussed in the lifetime annotation specification, any lifetimes in a
template argument should be propagated to all uses of the argument. Clang does
not provide a built-in mechanism for this, so this needs to be done in the
lifetime analysis code.

TODO: Add a link to relevant section of the lifetime annotation specification.

### Verifying lifetime correctness

TODO

### Generating error messages

If we detect that there is a lifetime error – either because a function is
returning a reference to a local or because there is a lifetime error inside the
function – we want to produce an easily comprehensible error message that
explains the error.

TODO: Explain how

## Alternative considered

We previously considered an alternative approach that built a set of constraints
between lifetimes involved in the function. Unfortunately, this approach
produced wrong results on some fairly simple examples involving variable
overwrites. A coworker identified a way to extend the approach in a way that
overcame many of these limitations, but this extension introduced additional
complexity. In the end, we decided that the approach based on points-to-sets was
the simpler alternative.

## Differences from Rust

### The exclusivity rule

The borrow checker in Rust, in addition to checking lifetimes, also enforces the
exclusivity rule: at any given time the program may have either one mutable
reference or any number of immutable references to the same storage location.

The exclusivity rule protects against certain kinds of memory safety errors. For
example, if it was applied to C++, it would catch the use after free here:

```c++
int test() {
  std::vector<int> xs;
  xs.push_back(10);
  const int &x0 = xs[0]; // `x0` borrows `xs` here.
  xs.push_back(20); // exclusivity error: `xs` is mutably borrowed here,
                    // overlapping with the `x0` borrow.
  return x0; // `xs` is borrowed by `x0` at least until here
             // because `x0` is used here.
}
```

Most C++ iterator invalidation bugs could be prevented by enforcing exclusivity:
while there are outstanding iterators that borrow the container, the container
can't be mutated.

From our experience porting woff2 from C++ to Rust, adjusting existing code to
follow the exclusivity rule is one of the most difficult steps in porting.
Therefore, it makes sense to separate rolling out lifetime checking from
exclusivity checking. Lifetime checks without exclusivity checks don't guarantee
memory safety, but they catch memory safety issues on their own, and should not
require many adjustments to C++ code.

Exclusivity checking could be rolled out in an optional second step. This would
not only provide additional memory safety to the C++ code but would facilitate a
manual or automatic conversion of C++ code to Rust.

### Spatial memory safety

Lifetime verification does not establish spatial memory safety, that is, it does
not prove that all accesses are in bounds. Rust collections perform these checks
at runtime.

<!-- Footnotes themselves at the bottom. -->

## Notes

[^1]: The following documents provide more background material:
    [doc 1](https://groups.seas.harvard.edu/courses/cs252/2011sp/slides/Lec06-PointerAnalysis.pdf),
    [doc 2](https://yanniss.github.io/points-to-tutorial15.pdf).
[^2]: Unless converting constructors and conversion constructors are used to
    simulate variance.
[^3]: Note, however, that Clang is currently very conservative in assigning
    types to type-dependent expressions.
