<!--* css: "//depot/docs/includes/featuretable.css" *-->

# Limits of Crubit

Crubit aims to make Rust a viable option for engineering teams with large,
pre-existing C++ codebases. There are limits to how well C++ and Rust can
interoperate. Tradeoffs still exist, and there can be tension between competing
goals. And the tools we have for interop, including Crubit, are many years away
from feature-completeness.

Crubit tries to push the envelope with what is possible with C++/Rust interop,
automatically supporting cross-language calls with the minimum of developer
toil. Where this is not possible, Crubit makes it possible (and easy, in most
cases) to work around the mismatch, with documented patterns for the most common
deficiencies.

## High Level Features

Below is an incomplete overview of features we expect Crubit to support
eventually. (This attempts to cover major axes in which interfaces can present
challenges for interop.) Included in the overview is the short term estimation
of overall difficulty, and long-term confidence levels of feasibility for fully
automatic interop. Note that some of these depend on features outside the direct
control of the Crubit team, such as in the Rust language and compiler.

We also describe, in abbreviated form, some of the patterns one can employ to
work around limitations in Crubit. More detailed information is available in
crubit.rs/cpp/cookbook and crubit.rs/errors.

### Functions

**Feature**                                                               | % functions | Expected Interop Effort (2027)    | Will Crubit automate this (eventually)?
:------------------------------------------------------------------------ | ----------- | --------------------------------- | ---------------------------------------
Call into a non-templated C++ function from Rust, if annotated correctly. | x           | [Light](#light){.😊}               | [**Planned: 2027**](#ga){.ga}
Call into a templated C++ function                                        | x           |                                   | Depends, see [Generics and Templates](#generics).
<!-- blank separator -->                                                  |             |                                   |
Call into a non-generic Rust function from C++.                           | x           | [Fully automatic](#automatic){.😊} | [**Confidence: Very High**](#veryhigh){.😊}
Call into a generic Rust function                                         | x           |                                   | Depends, see [Generics and Templates](#generics).

Most functions will be callable if Crubit is enabled, but it may require more or
different code in the other language. A notable example is functions which
return non-Rust-movable types, which must be invoked differently from Rust than
functions which do not. Or, for Rust calling C++, while you can *call* any
function that is designed to be looked up by
[ADL](https://en.cppreference.com/w/cpp/language/adl.html), you cannot use ADL
to do so.

In the immediate future, there is a long tail of reasons that functions may
*not* get bindings. For example, any unrecognized function attribute will cause
the bindings to be disabled. For the foreseeable future, barring changes to
Rust, we will also require annotations if a function is overloaded, to give it a
unique Rust name. However, we expect unsupported parameter or return types to be
the most common reason that a function does not receive bindings. These reasons
are expected to go away over time, and we aim for something like 95%+ of
non-templated functions receiving bindings by the end of 2027.

<!-- TODO(jeanpierreda): gather statistics -->

Fortunately, missing support for a function is easy to work around. Sometimes,
this can be solved with a simple
[Crubit annotation](http://crubit.rs/cpp/customizing) (such as
crubit.rs/errors/unknown_attribute), or some other minor change to the function
declaration. In the case of the type being unsupported, enabling Crubit on the
target that defines that type will be a common fix. But, at worst, one can wrap
it behind a new function which Crubit does support. The new function can use
fewer attributes, fewer language features, hide unsupported types behind wrapper
types (or void pointers), and so on.

Using C++ as an example:

```c++
// This function will always get bindings
inline void DoStuff(void* untyped_out, const void* untyped_in) {
    auto* out = reinterpret_cast<UnsupportedType*>(out);
    const auto* in = reinterpret_cast<const OtherUnsupportedType*>(untyped_in);
    // arbitrary C++ logic here
    out->UnsupportedMethod(*in);
}
```

Any C++ feature or type that is not supported in the automatically generated
Rust bindings can be hidden in this way. (crubit.rs/errors/unsupported_type
describes other solutions for the unsupported type case.)

A similar strategy can be employed for Rust.

### Non-templated C++ Types

**Feature**                                                                           | % functions | % classes | Expected Interop Effort (2027)    | Will Crubit automate this (eventually)?
:------------------------------------------------------------------------------------ | ----------- | --------- | --------------------------------- | ---------------------------------------
Pass any Rust-movable C++ type by pointer or by value, including inside Rust structs. | x           | x         | [Fully automatic](#automatic){.😊} | [**Planned: 2027**](#ga){.ga}
Pass any non-Rust-movable C++ type by pointer, including inside Rust structs.         | x           | x         | [Fully automatic](#automatic){.😊} | [**Planned: 2027**](#ga){.ga}
Pass any non-Rust-movable C++ type by value, including inside Rust structs.           | x           | x         | [Fully automatic](#automatic){.😊} | [**Planned: 2027**](#ga){.ga}
A lightweight syntax can be used to perform C++ move construction or assignment       | N/A         | N/A       | [Fully automatic](#automatic){.😊} | [**Planned: 2027**](#ga){.ga}

C++ types are more complicated than Rust types, and can maintain invariants Rust
does not. For example, they can enforce non-destructibility (via a deleted
destructor), or automatically maintain pinnedness (via implicitly invoked copy
and move constructors). These types will not be as easy to use from Rust, but we
are aiming for near 100% coverage, so that functions can typically receive
bindings.

Bindings can be generated for a type, even if the fields or other subobjects
inside of it do not have bindings. What matters are the high level properties:
the type's layout/ABI, what operations it supports, and so on. So when a type is
not supported by Crubit, it almost always works to wrap it in a new type.
However, more than for functions, we want to make this necessary as little as
possible. Types have a wide blast radius: if a type does not get bindings, then
no function using that type gets bindings. It is more important to support all
types than to support all functions, and we plan to entirely cover the space of
types and fully characterize any cases where they cannot receive bindings.

### Non-Generic Rust Types

**Feature**                                                   | % functions | Expected Interop Effort (2027)    | Will Crubit automate this (eventually)?
:------------------------------------------------------------ | ----------- | --------------------------------- | ---------------------------------------
Pass any non-Generic Rust type by pointer or by value in C++. | x           | [Fully automatic](#automatic){.😊} | [**Confidence: Very High**](#veryhigh){.😊}

There are currently no known cases where we cannot in principle support using a
(non-generic) Rust type from C++.

### Containers and Smart Pointers

| **Feature**       | % functions | Expected Interop      | Will Crubit        |
:                   :             : Effort (2027)         : automate this      :
:                   :             :                       : (eventually)?      :
| :---------------- | ----------- | --------------------- | ------------------ |
| Pass all standard | x           | [Medium](#medium){.😑} | [**Confidence:     |
: containers, smart :             :                       : High**](#high){.😊} :
: pointers, and     :             :                       :                    :
: idiomatic         :             :                       :                    :
: vocabulary types  :             :                       :                    :
: between           :             :                       :                    :
: languages, with   :             :                       :                    :
: zero or           :             :                       :                    :
: constant-time     :             :                       :                    :
: overhead bridging :             :                       :                    :
: between           :             :                       :                    :
: corresponding     :             :                       :                    :
: types.            :             :                       :                    :

Every type gets some baseline level of support (and the wording around C++ types
above applies), but these vocabulary types are valuable to special-case to
receive a better Rust API. We do this by rewriting the types in pure Rust. For
example, the C++ template class:

```c++ {.no-copy}
template <typename T> struct Span {T* ptr; size_t len;};
```

Has a Rust equivalent:

```rust {.no-copy}
#[repr(C)] pub struct Span<T>(*mut T, usize);
```

If the layouts match, then these can be used interchangeably in most places,
even with otherwise no template/generic support.

However, there is a long tail of vocabulary types, and types in the standard
library often have differing implementations, some of which have incompatible
Rust semantics. (GCC's `std::string` is pinned, Clang's is Rust-movable.)

We expect users of Crubit to gravitate towards a set of types that works well in
both languages, especially on multiplatform codebases.

### Generics and Templates {#generics}

| **Feature**  | % functions | Expected Interop Effort    | Will Crubit       |
:              :             : (2027)                     : automate this     :
:              :             :                            : (eventually)?     :
| :----------- | ----------- | -------------------------- | ----------------- |
| Instantiate  | x           | [Fully                     | [**Planned:       |
: C++ class    :             : automatic](#automatic){.😊} : 2027**](#ga){.ga} :
: templates    :             :                            :                   :
: from Rust    :             :                            :                   :
: with         :             :                            :                   :
: **concrete** :             :                            :                   :
: types and    :             :                            :                   :
: constants as :             :                            :                   :
: template     :             :                            :                   :
: parameters.  :             :                            :                   :

**Feature**                                                                                                                                          | %   | Expected Interop Effort (2027) | Will Crubit automate this (eventually)?
:--------------------------------------------------------------------------------------------------------------------------------------------------- | --- | ------------------------------ | ---------------------------------------
Instantiate C++ function templates from Rust with **concrete** types and constants as template parameters.                                           | x   | [Medium](#medium){.😑}          | [**Confidence: Speculative**](#speculative){.🙁}
Instantiate C++ function and class templates using types defined in the current crate.                                                               | x   | [Heavy](#heavy){.🙁}            | [**Confidence: Uncertain**](#uncertain){.🙁}
Instantiate C++ function and class templates with **generic** type parameters. Checking of function calls will be deferred to monomorphization time. | x   | [Heavy](#heavy){.🙁}            | [**Confidence: Speculative**](#speculative){.🙁}
Instantiate generic Rust types and functions from C++.                                                                                               | x   | [Medium](#mediun){.😑}          | [**Confidence: Uncertain**](#uncertain){.🙁}

Templates (and generics) are instantiated lazily on use, while Crubit is
designed as an ahead-of-time interface/type transpiler. Ultimately, this
approach hits its limit here, and a new design involving more directly hooking
the compilers together will be necessary to fully support templates and generics
in every circumstance.

Crubit plans to (by the end of 2027) support fully instantiated templates or
generic types: functions which return or accept a `Foo<Bar>`, where `Bar` is not
a template type parameter. Other uses of templates will need to be decomposed
into building blocks that can receive bindings, or else reimplemented in the
other language (as we do with types like `Span`).

### Abstraction and interfaces

**Feature**                                                                                                | %             | Expected Interop Effort (2027) | Will Crubit automate this (eventually)?
:--------------------------------------------------------------------------------------------------------- | ------------- | ------------------------------ | ---------------------------------------
Implement an interface defined by a C++ base class from Rust code.                                         | x% of classes | [Heavy](#heavy){.🙁}            | [**Confidence: High**](#high){.😊}
Implement a trait defined in Rust from C++ statically.                                                     | N/A           | [Heavy](#heavy){.🙁}            | [**Confidence: High**](#high){.😊}
Implement a dyn-compatible trait defined in Rust using an abstract base class and virtual dispatch in C++. | N/A           | [Heavy](#heavy){.🙁}            | [**Confidence: Uncertain**](#uncertain){.🙁}

Rust and C++ have many features that are not just "a function can be called", or
"a type can be used". The way in which you define or dispatch to functions can
be a load bearing part of the interface, and doesn't always have a natural
equivalent on the other side, and Crubit will not support all of these for some
time yet.

The recommended workaround, in most places, is to define a part of your logic in
one language, and then call into the other. For example, write a derived class
in C++, but implement all of its virtual methods by calling into Rust functions.

There is an example of this in
[crubit/examples/cpp/virtual/](https://github.com/google/crubit/tree/main/examples/cpp/virtual/).

### Safety

**Feature**                                                                                         | %functions | Expected Interop Effort (2027)                     | Will Crubit automate this (eventually)?
:-------------------------------------------------------------------------------------------------- | ---------- | -------------------------------------------------- | ---------------------------------------
Safely call C++ functions from Rust, with lifetime preconditions checked by the Rust borrow checker | x          | [Light](#light){.😊} (where possible)               | Crubit gives x% of functions safe bindings
Check lifetime constraints on Rust APIs used from C++.                                              | x          | [Fully automatic](#automatic){.😊} (where possible) | [**Confidence: Speculative**](#speculative){.🙁}

Rust and C++ have very different stances on memory safety, but both languages do
share a goal to, when reasonable in that language, make it safe to call given
functions. In Rust, this is embedded in the type system. In C++, this often
takes the form of implementation-specific attributes which the compiler can read
to inform what warnings to give.

It is possible to lose safety unless FFI correctly propagates the relevant
information to the other side. And in Rust, incorrectly marking functions as
unsafe can lead to safety fatigue, where most calls are trivially correct, and
the few that aren't have insufficient review.

### Confidence Levels

Confidence levels:

<a id="ga" class="ga">**Planned: 2027**</a> We plan on finishing this before the
end of 2027, as part of our next major milestone.

<a id="veryhigh" class="😊">**Confidence: Very High**</a> We know we can achieve
this. There may still be caveats for edge cases that require language or
compiler work to fix.

<a id="high" class="😊">**Confidence: High**</a> We think we can achieve this,
but some of the details are not fully designed, and there is always a risk of
"unknown unknowns". May depend on getting features upstream, but we are
reasonably confident we can do so.

<a id="uncertain" class="🙁">**Confidence: Uncertain**</a> There are substantial
unknowns in the "how" that will take some dedicated engineering effort to
resolve. We think there is a path that looks feasible.

<a id="speculative" class="🙁">**Confidence: Speculative**</a> At a high level it
seems like this should be possible, but there are major technical, social, or
resourcing risks that could prevent it from happening.

### Expected Interop Effort (2027)

<a id="automatic" class="😊">**Fully Automatic Interop**</a> Crubit must be
enabled on targets.

<a id="light" class="😊">**Light**</a> Some declarations need to be annotated.
For example, overloaded functions must have a Rust name specified.

<a id="medium" class="😑">**Medium**</a> A layer of FFI glue code is required,
such as new function or type definitions. The patterns involved are
straightforward, and documented. Expertise is helpful, but not required.

<a id="heavy" class="🙁">**Heavy**</a> Multiple layers of ad-hoc FFI glue are
required, even with Crubit, or else the task may require reimplementing
significant chunks of code in the other language. Some expertise may be
required.

## Irreconcilable differences

### Expressiveness

C++ is fundamentally a more expressive language, in terms of accepted code
patterns, than Rust. While Rust will continue to evolve and gain some forms of
expressiveness, many of its strengths draw from its relatively limited
expressiveness in certain areas. Therefore, differences between C++ and Rust
will always exist, and show up in the source code.

Examples of these include:

*   Safety annotations
*   Implicit invocation of user-defined constructors, conversion operators, and
    assignment operators
*   Overloading operators in C++ that are not overloadable in Rust
*   Text-based macro substitution

Typically, this means that Rust code ends up more verbose than C++ code, though
it can also require adding additional annotations to the C++ to better inform
how to match C++'s very general semantics with Rust's more narrow interfaces.

### Diverging patterns

Idiomatic patterns are not the same between the two languages, and will continue
to diverge further over time. For example, many more objects are pinned in C++
than are pinned in Rust, and this will present an ongoing impedance mismatch.

### Ecosystem considerations

The Rust and C++ languages are limited in how far they can evolve to accommodate
interoperability. Within each community there are priorities besides interop at
play.
