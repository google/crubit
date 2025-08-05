# Limits of Crubit

Crubit intends to push the envelope of what is possible with C++/Rust interop,
and make Rust a viable option for engineering teams with large, pre-existing C++
codebases. However, tradeoffs still exist, and sometimes there is unresolvable
tension between competing goals, or with the practical reality.

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

### Diverging patterns

Idiomatic patterns are not the same between the two languages, and will continue
to diverge further over time. For example, many more objects are pinned in C++
than are pinned in Rust, and this will present an ongoing impedance mismatch.

### Ecosystem considerations

Crubit may be limited by how the Rust and C++ languages can evolve to
accommodate its aims. Crubit does not directly control either language, and
within each community there are priorities besides interop at play.

## High Level Features

Below is a sampling of features we expect Crubit to support eventually, with
confidence levels of their feasibility. Note that some of these depend on
features outside the direct control of the Crubit team, such as in the Rust
language and compiler.

**Functions**                                      | Confidence
:------------------------------------------------- | ------------:
Call into any non-template C++ function from Rust. | **Very High**
Call into any non-generic Rust function from C++.  | **Very High**

| **Values and references**                                   | Confidence    |
| :---------------------------------------------------------- | ------------: |
| Pass any C++ type by \[smart\] pointer or by value,         | **Very High** |
: including inside Rust structs.                              :               :
| A lightweight syntax like `x.move()` can be used to perform | **High**      |
: C++ move construction or assignment                         :               :
| Pass any Rust type by \[smart\] pointer or by value in C++. | **Very High** |
: Rust types that do not have a representable ABI should be   :               :
: wrapped in a struct that allows them to be passed by value. :               :
| Pass all standard containers and idiomatic vocabulary types | **High**      |
: between languages, with zero or constant-time overhead      :               :
: bridging between corresponding types.                       :               :

| **Generics and templates**                               | Confidence      |
| :------------------------------------------------------- | --------------: |
| Instantiate C++ function and class templates from Rust   | **High**        |
: with **concrete** types and constants as template        :                 :
: parameters.                                              :                 :
| Instantiate C++ function and class templates using types | **Uncertain**   |
: defined in the current crate.                            :                 :
| Instantiate C++ function and class templates with        | **Speculative** |
: **generic** type parameters. Checking of function calls  :                 :
: will be deferred to monomorphization time.               :                 :
| Instantiate generic Rust types and functions from C++.   | **Uncertain**   |
| Check lifetime constraints on Rust APIs used from C++.   | **Speculative** |

| **Abstraction and interfaces**                              | Confidence    |
| :---------------------------------------------------------- | ------------: |
| Implement an interface defined by a C++ abstract base class | **High**      |
: from Rust code.                                             :               :
| Implement a trait defined in Rust from C++ statically.      | **High**      |
| Implement a dyn-compatible trait defined in Rust using an   | **Uncertain** |
: abstract base class and virtual dispatch in C++.            :               :

Confidence levels:

**Very High** We know we can achieve this. There may still be caveats for edge
cases that require language or compiler work to fix.

**High** We think we can achieve this, but some of the details are not fully
designed, and there is always a risk of "unknown unknowns". May depend on
getting features upstream, but we are reasonably confident we can do so.

**Uncertain** There are substantial unknowns in the "how" that will take some
dedicated engineering effort to resolve. We think there is a path that looks
feasible.

**Speculative** At a high level it seems like this should be possible, but there
are major technical, social, or resourcing risks that could prevent it from
happening.
