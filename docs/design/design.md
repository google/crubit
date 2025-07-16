# High-level design of C++/Rust interop

This document describes the high-level design choices of Crubit, a C++/Rust
Bidirectional Interop Tool.

[TOC]

## C++/Rust interop goal

**The primary goal of Crubit is to enable Rust to be used side-by-side with C++
in large existing codebases.**

In the short term we would like to focus on codebases that roughly follow the
Google C++ style guide to improve the interop fidelity. Other, more diverse
codebases are possible prospective users in the long term, and their needs will
be addressed by customization and extension points.

## C++/Rust interop requirements

In support of the interop goal, we identify the following requirements:

1.  **Enable using existing C++ libraries from Rust with high fidelity**
    *   **High fidelity means that interop will make C++ APIs available in Rust,
        even when those API projections would not be idiomatic, ergonomic, or
        safe** in Rust, to facilitate cheap, small step incremental migration
        workflow. Based on the experience of other cross-language
        interoperability systems and language migrations (for example,
        Objective-C/Swift, Java/Kotlin, JavaScript/TypeScript), we believe that
        working in a mixed C++/Rust codebase would be significantly harder if
        some C++ APIs were not available in Rust.
    *   **Interop will bridge C++ constructs to Rust constructs only when the
        semantics match closely**. Bridging large semantic gaps creates a risk
        of making C++ APIs unusable in Rust, as well as a risk of creating
        performance problems. For example, interop will not bridge destructive
        Rust moves and non-destructive C++ moves; instead it will make C++ move
        constructors and move assignment operators available to use in Rust
        code. As another example, interop will not bridge C++ templates and Rust
        generics by default.
    *   Interop should be **performant**, as close to having no runtime cost as
        possible. The performance costs of the interop should be documented, and
        where possible, intuitive to the user.
    *   Interop should be **ergonomic and safe**, as long as ergonomic and
        safety accommodations do not hurt performance or fidelity. Where a
        tradeoff is possible, the interop will choose performance and fidelity
        over ergonomics; the user will be allowed to override this choice.
    *   **Enable owners of the C++ API to control their Rust API projection**,
        for example, with attributes in C++ headers and by extending generated
        bindings with a manually implemented overlay. Such an overlay will wrap
        or extend generated bindings to improve ergonomics and safety.
2.  **Enable using Rust libraries from C++**
    *   However, using C++ libraries from Rust has a higher priority than using
        Rust libraries from C++.
3.  **Put little to no barriers to entry**
    *   **Ideally, no boilerplate code** needs to be written in order to start
        using a C++ library from Rust. Adding some extra information can make
        the generated bindings more ergonomic to use.
    *   The amount of **duplicated API information is minimized**.
    *   **Future evolution of C++ APIs should be minimally hindered by the
        presence of Rust users**.

## Proposal and high-level design

**We propose to develop our own C++/Rust interop tooling.** There are no
existing tools that satisfy all of our requirements. Modifying an existing tool
to fulfill these requirements would take more effort than building a new tool
from scratch or might require forking its codebase given that some existing
tools have goals that conflict with our goals.

See the "alternatives considered" section for a discussion of existing tools.

### Source of information about C++ API

**Interop tooling will read C++ headers**, as they contain the information
needed to generate Rust API projections and the necessary glue code. Interop
tooling that is used during builds will not read C++ source files, to maintain
the principle that C++ API information is only located in headers, and that a
C++ library can't break the build of its dependencies by changing source files.

Some interop-adjacent tools (e.g., large-scale refactoring tools that seed the
initial set of lifetime annotations) will also read C++ sources. These tools
will not be used during builds.

**Pros**

*   **Minimal barrier to entry**: minimal amount of manual work is required to
    start using a C++ library from Rust.
    *   Encourages leaf projects to start incrementally adopting Rust in new
        code, or incrementally rewriting C++ targets in Rust.
*   **C++ API information is located only in headers**, regardless of the
    language that the API consumer is written in (C++ or Rust).
*   **Interop tooling that generates Rust API projections from a C++ header can
    get exactly the same information that the C++ compiler has** when processing
    a translation unit that uses one of the APIs declared within that header.
    *   Interop tooling can generate the most performant calls to C++ APIs,
        without C++-side thunks that translate the C++ ABI into a C ABI.
    *   Interop tooling can autodetect implementation details that are critical
        for interop but are not a part of the API surface (for example, the size
        and alignment of C++ classes that have private data members).
    *   In alternative solutions, users need to repeat these implementation
        details in sidecar files. Interop can verify that the specified
        information is correct through static assertions in generated C++ code,
        but the overall user experience is inferior.

**Cons**

*   **Having to read C++ headers makes interop tooling more complex.**
*   **The Rust projection of the C++ API is only visible in machine-generated
    files.**
    *   These are not trivially accessible.
    *   There is a limit on how readable these files can be made.
    *   We can mitigate these issues by building tooling that shows the Rust
        view of a C++ header (for example in Code Search, or in editors as an
        alternative go-to-definition target).

### Customizability

Interop tooling will be sufficiently customizable to accommodate the unique
needs of different C++ libraries in the codebase. Interop should be customizable
enough to accommodate existing codebases. C++ API owners can:

*   **Guide how interop tooling generates Rust API projections from C++
    headers**. For example, headers can provide:
    *   Custom Rust names for C++ function overloads (instead of applying the
        general interop strategy for function overloads),
    *   Custom Rust names for overloaded C++ operators,
    *   Custom Rust lifetimes for pointers and references mentioned in the C++
        API,
    *   Nullability information for pointers in the C++ API,
    *   Assertions (verified at compile time) and promises (not verified by
        tooling) that certain C++ types are Rust-movable.
*   **Provide custom logic to bridge types**, for example, mapping C++
    `absl::StatusOr` to Rust `Result`.
*   **Provide API overlays** that improve the automatically generated Rust API.
    *   For example, the overlays could inject additional methods into
        automatically generated Rust types or hide some of the generated
        methods.

More intrusive customization techniques will be useful for template and
macro-heavy libraries where the baseline import rules just won't work. We
believe customizability will be an essential enabler for providing high-fidelity
interop.

### Source of additional information that customizes C++ API projection into Rust

Where C++ headers don't already provide all information necessary for interop
tooling to generate a Rust API projection, we will add such information to C++
headers whenever possible. If it is not desirable to edit a certain C++ header,
extra information can be stored in a sidecar file.

Examples of additional information that interop tooling will need:

*   **Nullability annotations.** C++ APIs often expose pointers that are
    documented or assumed by convention to be never null, but can't be
    refactored to references due to language limitations (for example,
    `std::vector<MyProtobuf *>`). If C++ headers don't provide nullability
    information for pointers in a machine-readable form, interop tooling has to
    conservatively mark all C++ pointers as nullable in the Rust API projection.
    The Rust compiler will then force users to write unnecessary (and
    untestable) null checks.
*   **Lifetimes of references and pointers** in C++ headers are not described in
    a machine-readable way (and sometimes are not even documented in prose).
    Lifetime information is essential to generate safe and idiomatic Rust APIs
    from C++ headers.

#### Additional information is stored in C++ headers

**Pros**

*   **Additional information needed for C++/Rust interop will be expressed as
    annotations on existing syntactic elements in C++.**
    *   The annotations are located in the most logical place.
    *   The annotations are more likely to be noticed and updated by C++ API
        owners.
    *   API owners retain full control over how the API looks in Rust.
*   **C++ users may find lifetime and nullability annotations useful.** For
    example, information about lifetimes is highly important to C++ and Rust
    users alike.
*   **C++ API definitions are only written once,** minimizing duplication and
    maintenance burden.

**Cons**

*   **Annotations that benefit Rust users can bother C++ API owners** who don't
    care about Rust. Especially at the beginning of integrating Rust into an
    existing codebase, C++ API owners can push back on adding annotations.
    *   To encourage adoption of annotations, we can develop tooling for C++
        that uses lifetime and nullability annotations to find bugs in C++ code.
    *   The pushback is likely to be short-term: if Rust takes off in a C++
        codebase, C++ library owners in that codebase will need to care about
        Rust users and how their API looks in Rust.
*   **There may be headers that we cannot (or would not want to) change**, for
    example, headers in third-party code, headers that are open-sourced, or when
    first-party owners are not cooperating.
    *   We can apply the
        [sidecar strategy](#additional-information-is-stored-in-sidecar-files)
        to these headers.

#### Additional information is stored in sidecar files

Additional information needed for C++/Rust interop can be stored in sidecar
files, similarly to Swift APINotes, CLIF etc. If sidecar files get sufficiently
broad adoption (for example, if annotating third-party code turns out to be
sufficiently important that optimizing C++/Rust interop ergonomics there would
be worth it), it would make sense to write sidecar files in a Rust-like
language, as that provides the most natural way to define Rust APIs.

**Pros**

*   **Sidecar files enable more broad adoption of annotations** by providing
    additional interop information without modifying C++ headers. Sidecar files
    will allow us to annotate headers in third-party code, headers that can't
    adopt annotations for technical reasons, or headers owned by first-party
    owners who are not cooperating.

**Cons**

*   Like in the
    [Use Rust code to customize API projection into Rust](#use-rust-code-to-customize-api-projection-into-rust)
    alternative, **some part of C++ API information is duplicated**, which is a
    burden for the C++ API owners.
*   The projection of C++ APIs to Rust is defined in a new language.
    *   C++ API owners and Rust users will have to learn this language.
    *   If we expect wide adoption of sidecar files, we will need to create
        tooling to parse, edit, and run LSCs against this language.
*   **Annotations in sidecar files are more prone to become out of sync with the
    C++ code.** When making changes to C++ code, engineers are less likely to
    notice and update the annotations in sidecar files.
    *   Presubmits can catch some cases of desynchronization between C++ headers
        and sidecar filles. However, presubmit errors that remind engineers to
        edit more files create an inferior user experience.
*   **Sidecar files create extra friction to modify the code.** Where previously
    one had to edit only a C++ header and a C++ source file, now one also likely
    needs to update a sidecar file.
    *   When engineers realize that they need to update a sidecar file, opening
        another file and finding the right place to update creates extra
        friction to modify code.
    *   Once engineers understand the extra maintenance burden associated with
        sidecar files that tend to go out of sync with headers, they will be
        less likely to adopt annotations in the first place.

### Glue code generation

C++/Rust interop tooling will generate executable glue code and type definitions
in Rust and in C++ (not just merely `extern "C"` function declarations) in order
to achieve the following goals:

*   **Enable instantiating C++ templates from Rust, and monomorphizing Rust
    generics from C++. Enable Rust types to participate in C++ inheritance
    hierarchies.**
    *   For example, imagine Rust code using an object of type
        `std::vector<MyProtobuf>`, while C++ code in the same program is never
        instantiating this type. The Bazel `rust_library` target that mentions
        this type must therefore be responsible for instantiating this template
        and linking the resulting executable code into the final program. We
        propose that this instantiation happens in an automatically generated
        "glue" C++ translation unit that is a part of that `rust_library`.
*   **Enable automatically wrapping C++ code to be more ergonomic in Rust.** For
    example:
    *   `extern "C"` functions in Rust are necessarily unsafe (it is a language
        rule). We would like the vast majority of C++ API projections into Rust
        to be safe. In the current Rust language, we can achieve that only by
        wrapping the unsafe `extern "C"` function in a safe function marked with
        `#[inline(always)]`.
    *   C++ API owners can provide rules for automatic type bridging, for
        example, mapping C++ `absl::StatusOr` to Rust `Result`. This conversion
        necessitates generation of a Rust wrapper function around a C++ entry
        point that takes advantage of such type bridging.
*   **Provide stable locations (C++ modules, Rust crates) that "own" the types
    from the language point of view.**
    *   For example, when we project a C++ type into Rust, its Rust definition
        must be located in a Rust crate. Furthermore, all Rust users of this
        type must observe it as being defined in the same crate in order for
        every users to consider that they use the same type. Indeed, this is a
        rule in Rust, that types defined in different crates are unrelated
        types.
    *   When we project a Rust type into C++ we could repeat its C++ definition
        in C++ code any number of times (for example, in every C++ user of a
        Rust type). This is technically fine because C++ allows the same type to
        be defined multiple types within a program. Nevertheless, such
        duplication is error-prone.

### Glue code is generated as C++ and Rust source code

Interop tooling will generate glue code as C++ and Rust source files, which are
then compiled with an unmodified compiler for that language. The alternative is
to generate LLVM IR or object files with machine code directly from interop
tooling.

**Pros**

*   **It is easy to inject customizations provided by API owners into generated
    source code.**
    *   The customizations will be written in the target language, making it
        (hopefully) intuitive to write them.
*   **Generated source code can be easily inspected by compiler engineers**
    while debugging interop problems and compiler bugs.
*   **Generated source code can be inspected and understood by interop users,**
    who are not compiler experts.
    *   LLVM IR wouldn't be meaningful to them.
*   **Generated source code is processed by the regular toolchain like any other
    code in the project.**
    *   It automatically benefits from all performance optimizations and
        sanitizers that are newly implemented in Clang and Rust compilers.
*   **We avoid adding a new tool that generates unique LLVM IR patterns.**
    *   We avoid making the job of the C++ toolchain maintainers harder.

**Cons**

*   **Interop tooling will be limited to generating LLVM IR and machine code
    that Clang and Rust compilers can generate.**

### Glue code and API projections will assume implementation details of the target execution environment

To provide the most ergonomic and performant interop, C++/Rust interop tooling
will allow the target codebase to opt into assuming various implementation
details of the target execution environment. For example:

*   When calling C++ from Rust, interop tooling can either wrap C++ functions in
    thunks with a C calling convention, or call C++ entry points directly.
    Thunks cause code bloat and can collectively add up to become a performance
    problem, so it is desirable to call C++ entry points from Rust directly.
    Interop tooling can do that only if it may assume a specific target platform
    and C++ ABI.

Implementation details of the target execution environment that are considered
stable enough will be reflected in API projections, for example:

*   The C++ standard does not specify sizes of integer types (`short`, `int`,
    `long` etc.) To map them to Rust, interop tooling will need to assume a size
    that they have on the platform that targets in practice. The alternative
    would be to create target-agnostic integer types (for example, `Int` in
    Swift is a strong typedef for `Int32` on 32-bit targets, and `Int64` on
    64-bit targets), but this makes it harder to provide idiomatic, transparent,
    high-performance interop.
*   The C++ standard does not specify whether standard library types like
    `std::vector` are in any sense Rust-movable; it is an implementation detail.
    Universal interop tooling would have to conservatively assume
    non-Rust-movable types. Interop tooling specific to certain environments can
    rely on libc++ providing a Rust-movable `std::vector` and project it into
    Rust in a much more ergonomic way.

**Pros**

*   **Interop tooling will generate the most performant code sequences** to call
    foreign language functions.
    *   If interop tooling generates portable code, it would have some overhead.
        The overhead can be eliminated by C++ and Rust optimizers at least in
        some cases, but at the cost of increased build times. For example,
        eliminating thunks would require turning on LTO, which is not fast, and
        usually only used for release builds. It is much preferable to not
        generate thunks in the first place, if the target platform does not need
        them.
*   **Ergonomics of API projections will be improved.**
    *   For example, whether a C++ type is Rust-movable or not is an
        implementation detail in C++, transparent to C++ users of that type, but
        it makes a huge ergonomic difference in the Rust API projection.

**Cons**

*   **C++ code will have additional evolution constraints.**
    *   For example, changing a type from Rust-movable to non-Rust-movable is a
        non-API-breaking change for C++ users, but it would break Rust users.
*   **It would be more difficult to switch internal environments to a different
    C++ standard library.**
*   **Code that is deployed in environments that have incompatible
    implementation details won't be able to use this C++/Rust interop system.**
    *   Alternatively, these executables would have to bring a suitable
        execution environment with them (e.g., a copy of libc++).

### Interop tooling should be maintainable and evolvable for a long time

We should design and implement C++/Rust interop tooling in such a way that we
can maintain and evolve it for more than a decade. If Rust becomes tightly
integrated into an existing C++ project, specific requirements for interop and
API projection rules will keep changing. The more Rust adoption we will have,
the more library and team-specific interop customizations we will have to
support, and the more it will make sense for the performance team to tweak
generated code to implement sweeping optimizations. These kinds of changes
should be readily possible, and they should not create conflicts of interest
between diferent users of the interop tooling.

### Interop tooling should facilitate C++ to Rust migration

C++/Rust interop tooling should try to create a favorable environment for
migrating C++ code to Rust. Specifically, projections of C++ APIs into Rust
should be implementable in Rust. This way, a C++ library can be converted from
C++ into Rust transparently for its users, as its public API won't change.

## Alternatives Considered: Design decisions

### Repeat C++ API completely in a separate IDL

Instead of reading C++ headers in the interop tooling, we would require the user
to repeat the C++ API in some other form, for example, in a Rust-based IDL like
in the cxx crate, or in sidecar files in a completely new format.

**Pros**

*   **Interop tooling can be simpler if it does not have to read C++ headers**.
    But even under this alternative approach, tooling might want to read C++
    headers, nullifying this advantage. For example, tooling might want to
    automatically generate an initial Rust snippet or to suggest in presubmits
    to adjust the Rust code that mirrors a C++ API when that C++ API changes.
*   The **most natural way to define Rust APIs** is by using Rust code or
    Rust-like syntax in sidecar files.
*   **Available Rust APIs are defined in easily accessible checked-in files.**
*   **API definitions written by a human might have higher quality, on
    average.**

**Cons**

*   **A big part of the C++ API needs to be duplicated** to reliably match the
    Rust code with the C++ declarations. The initial code can be generated by
    tooling, but it has to be kept in sync. This is a burden for the C++ API
    owners, potentially a bigger one than allowing annotations in C++ headers.
    *   There is a risk that C++ API owners might refuse to own IDL files.
*   The need to create a sidecar file creates a **barrier to start using C++
    libraries from Rust.**
    *   While the duplication overhead is justifiable for widely-used libraries,
        it is relatively high for libraries with few users and binaries, making
        it less likely that leaf teams will start adopting Rust.
*   **When the C++ API is changed, the Rust definitions become out-of-sync with
    it.** Tooling needs to detect this, and the Rust definitions need to be
    changed (either manually or tool-assisted).
*   There is no effective way to verify Rust binding code at the presubmit time
    of a C++ library other than building downstream projects.
*   **Mapping Rust API definitions to the original C++ API definitions is more
    complicated and error-prone**. For example, how would we target a specific
    overload of a function or constructor?
*   There is a **risk that individual teams will build team-specific tooling
    that generates IDL files** from C++ headers or generates both IDL files and
    C++ headers from a single source. These solutions are unlikely to scale to
    existing large codebases and will likely only work for that specific team.

### Use Rust code to customize API projection into Rust

An alternative to storing additional information in C++ headers is to put it
into Rust code. For example, the cxx crate requires users to re-state the C++
API in Rust syntax, adding information about lifetimes and nullability. The pros
and cons of this choice are the same as when defining a special IDL that repeats
the C++ API completely (see above).

### Generate glue code in binary formats

Instead of generating glue code as textual sources, interop tooling could use
Clang and LLVM APIs to emit object files with C++ glue code and use Rust
compiler APIs to generate rmeta and rlib files with Rust glue code.

**Pros**

*   **More flexibility in the code that can be generated.** Controlling LLVM IR
    generation allows interop tooling to generate code that an unmodified
    compiler can't generate from textual source code. For example, the Rust
    language does not have any constructs that map to `linkonce_odr` functions
    in LLVM IR; if the interop tooling embedded the Rust compiler as a library
    and had more control over how it generates the IR, we could make that
    happen.

**Cons**

*   Injecting customizations provided by API owners is harder.
*   LLVM, Clang, and Rust compiler APIs are not stable. The format of Rust
    metadata files is not stable either. The larger the API subset we consume
    from Clang and Rust, the more difficult it becomes to maintain the tooling.
*   To generate object files the interop tooling has to ensure that its
    Clang/LLVM version and configuration is identical with the Clang compiler
    used to build other C++ code.
    *   We can solve this problem, but it makes the system more fragile,
        compared to using existing C++ and Rust compilers to compile generated
        sources.
*   From time to time LLVM introduces bugs that cause miscompilations. If
    interop tooling embeds LLVM, we would be adding another tool that toolchain
    engineers will need to look into when debugging a miscompilation. We would
    be making the job of C++ toolchain maintainers harder.

## Alternatives Considered: Existing tools

### bindgen

[bindgen](https://rust-lang.github.io/rust-bindgen/) **automatically generates
Rust bindings from C and C++ headers**, which it consumes using libclang. The
generated **bindings are pure Rust code** that interfaces with C and C++ using
Rust’s [built-in FFI for C](https://doc.rust-lang.org/nomicon/ffi.html)
(`#[repr(C)]` to indicate that a struct should use C memory layout and `extern
"C"` to indicate that a function should use a C calling convention). C++
functions are handled by generating a Rust `extern "C"` function that has the
same ABI as the C++ function and attaching a `link_name` attribute with the
mangled name.

See
[here](https://manishearth.github.io/blog/2021/02/22/integrating-rust-and-c-plus-plus-in-firefox/)
for an in-depth description of the use of bindgen in Stylo, a Rust component in
Firefox.

**Pros**

*   **The oldest and the most mature** of the existing C++ interop tools
    (developed
    [since Feb 2012](https://github.com/rust-lang/rust-bindgen/commit/9fe92b0cfd48d5ebd1c82af8b1ff041f8c416a65)).

**Cons**

*   **Deficiencies in safety and ergonomics**, for example:
    *   References are imported as pointers. No lifetimes, no null-safety.
    *   Constructors and destructors are not called automatically.
    *   Overloads are distinguished by a numbered suffix in Rust. These numbers
        clutter the source code and are hard to remember, as they have no
        meaning. Adding overloads can change the numbering and hence break Rust
        callers.
*   It is **impossible to use C++ inline functions and templates** from Rust
    because of bindgen’s architecture[^1]. The architecture is unlikely to
    change, and therefore, this is a dealbreaker.

**Evaluation**

bindgen could be used in a project that has very limited C++ interop needs.
However, creating safe and ergonomic wrappers for the generated bindings would
require additional effort. Our vision and goals for C++ interop are very
different from what bindgen provides.

### cbindgen

[cbindgen](https://github.com/eqrion/cbindgen) **automatically generates C or
C++ headers for Rust libraries which expose a public C API**.

**Pros**

*   **An old and mature tool** (developed
    [since March 2017](https://github.com/eqrion/cbindgen/commit/215d3a987b223d4a1a878e2385c8677d5ae3a80b)).

**Cons**

*   **Shallow understanding of Rust's modules and types**.

    *   [`cbindgen`'s docs](https://github.com/eqrion/cbindgen/blob/master/docs.md)
        point out that "A major limitation of cbindgen is that it does not
        understand Rust's module system or namespacing. This means that if
        cbindgen sees that it needs the definition for MyType and there exists
        two things in your project with the type name MyType, it won't know what
        to do. Currently, cbindgen's behaviour is unspecified if this happens."
    *   This limitation seems mostly caused by building `cbindgen` on top of
        [the `syn` crate](https://docs.rs/syn). `syn` is able to parse Rust
        source code into an AST, but there is no facility at the `syn` level for
        type deduction or module traversal. Building such functionality would
        require replicating parts of the `rustc` compiler into `cbindgen`, or
        alternatively rewriting `cbindgen` on top of
        [the `rustc_driver` crate](https://doc.rust-lang.org/stable/nightly-rustc/rustc_driver/)).

*   **Support of only `extern "C"` functions**.

    *   Supporting Rust functions that use the default calling convention would
        require generating not only C/C++ headers, but also generating Rust
        source with `extern "C"` thunks that trampoline into the original
        function (requiring that `cbindgen` starts generating Rust sources).

*   **Support of only `#[repr(C)]` structs**.

    *   Default memory layout of Rust structs is
        [unspecified](https://rust-lang.github.io/unsafe-code-guidelines/layout/structs-and-tuples.html#default-layout-repr-rust:~:text=the%20default%20layout%20of%20structs%20is%20not%20specified)
        and therefore cannot be determined by code examination at the `syn`
        level.
    *   Even if the memory layout could be determined, the layout can change in
        a future compiler version, or change depending on compilation command
        line flags. To prevent using stale layout information, the
        auto-generated FFI code should therefore include compile-time assertions
        that the layout didn't change from the FFI generation time. The
        assertions should be present both in the generated C/C++ headers *and*
        on the Rust side (requiring that `cbindgen` starts generating Rust
        sources). The assertions would effectively verify that the FFI
        generation is driven by the build system (i.e. by Bazel, or Cargo, or
        GN/ninja, rather than manually) and that the integration between the FFI
        tools and the build system doesn't have any bugs (e.g. that it
        faithfully replicates all relevent compilation flags).

**Evaluation**

cbindgen could be used in a project that can create a narrow `extern "C"` /
`#[repr(C)]` API and that is ready to manage the risk of incorrect name/module
resolution. Wrapping additional Rust APIs would require extra effort.

**Take-aways for Crubit design**

Notes and observations about `cbindgen` can guide some design aspects of
Crubit's [`cc_bindings_from_rs`](../cc_bindings_from_rs/README.md) tool
(that similarly to `cbindgen` generates C++ bindings for Rust crates).
Using internal compiler knowledge (e.g. memory layout of structs, name and type
resolution) requires that `cc_bindings_from_rs` depends on
`rustc_driver` and other internal crates of `rustc`. The API of these crates is
unstable which might increase the risk and maintenance cost of Crubit.
Nevertheless, our experience with maintaining tools based on (also unstable)
Clang APIs suggests that this extra risk and cost is likely going to be
acceptable.

Build determinism requires that the Rust compiler produces the same output for
the same set of inputs (the same compiler version, the same command-line flags,
the same sources, etc.). This means that (despite
[conservative reservations about layout determinism](https://rust-lang.github.io/unsafe-code-guidelines/layout/structs-and-tuples.html#default-layout-repr-rust:~:text=A%20note%20on%20determinism))
it should be okay to assume that `cc_bindings_from_rs` and `rustc` invocations
will observe the same memory layout of structs, but this requires that
`cc_bindings_from_rs` is built against exactly the same version of
`rustc_driver` libraries as `rustc`. (This should also be reinforced by
compile-time assertions in the generated FFI layer.)

### cxx

[cxx](https://cxx.rs/) generates **Rust bindings for C++ APIs and vice versa**
from an **interface definition language (IDL) included inline in Rust source
code.** cxx generates Rust and C++ source code from IDL definitions. To check
that the IDL definitions match the actual C++ API, cxx inserts static
assertions[^2] into the generated C++ code; it does not, however, read the C++
headers itself. cxx contains built-in bindings for various Rust and C++ standard
library types that are not customizable.

As far as we understand, cxx has the following design constraints and goals:

*   **Ship a stable product for its intended audience.**
    *   As a consequence, improvements such as integrating move semantics are
        not going to be accepted soon. We understand that cxx is not a vehicle
        for experimentation. cxx maintainers would prefer us to first show that
        our ideas work in a fork of cxx or in a different system, such as
        autocxx, and that our improvements pull their weight given the added
        complexity.
*   **Remain simple and transparent.** There is a limit on the amount of
    complexity that will be tolerated.
    *   There is a chance that improvements such as modeling C++ move semantics
        or various attempts at eliminating thunks will not be ever accepted in
        upstream cxx.
*   **Non-goal: Automatically provide high fidelity interop.**
    *   cxx is designed for the use case of an executable where C++ and Rust
        parts communicate through a narrow interface.
*   **Non-goal: Automatically provide the most performant interop in as many
    cases as possible.** For example:
    *   cxx does not attempt to eliminate C++-side thunks. Instead, using LTO is
        recommended.
    *   cxx considers it acceptable to allocate all objects of "opaque" types on
        the heap. Users who find these heap allocations unacceptable for
        performance reasons are expected to implement a different C++ entry
        point that does not hit this limitation and bind it to Rust instead of
        the original C++ API. Heap allocation is acceptable for many C++ classes
        in most environments, but the exceptions are important enough for us
        that this is a major restriction.

**Pros**

*   **Mature and ergonomic enough today for mixing C++ and Rust in existing
    codebases with limited C++ interop needs.**
*   We avoid being on a tech island.

**Cons**

*   cxx’s stability goal makes it **hard to experiment with how the Rust API
    looks.**
*   **Our goals are unlikely to align well with the goals of the intended user
    audience of cxx.** We would be pulling cxx in directions that make it a
    worse product for its current users.
*   **Almost no customizability**. Users who are not satisfied with what cxx
    does are expected to wrap the target C++ API in a different C++ API that is
    more friendly to cxx.
*   cxx tries to be compatible with most standard C++ implementations found in
    the real world, so it **cannot take advantage of unique guarantees provided
    by the target execution environment.**

**Evaluation**

cxx could be used in projects with limited C++/Rust interop requirements.
However, we would not be able to implement many interop features that we
consider essential (for example, move semantics, templates).

### autocxx

[autocxx](https://github.com/google/autocxx) **automatically generates Rust
bindings from C++ headers**. As the name implies, it automatically generates IDL
definitions for cxx, which then produces the actual bindings. In addition,
autocxx generates its own Rust and C++ code to extend the Rust API beyond what
cxx itself would provide, for example to support passing POD types by value.
autocxx consumes C++ headers indirectly by first running bindgen on them and
then parsing the Rust code output by bindgen.

autocxx’s
[design goals](https://www.chromium.org/Home/chromium-security/memory-safety/rust-and-c-interoperability)
are similar to our own in this document.

We did a case study on using an existing project's C++ API from Rust using
autocxx.

**Pros**

*   **Low barrier to entry**: Bindings are generated from C++ headers, no need
    to write duplicate API definitions.
*   **Ergonomic mappings** for many C++ constructs.
*   **Open to contributions that change the generated Rust APIs** or make
    architectural changes.

**Cons**

*   **Relatively new and immature.**
*   **Cannot (yet) consume complex headers without errors.** We’ve managed to
    import some actual Spanner headers, but there are still enough outstanding
    issues that we can’t yet do anything useful with Spanner.
*   **Architecture can make modifications difficult.** autocxx is built on top
    of two other tools, bindgen and cxx, and the interfaces between these
    components can make it harder to make a modification than it would be in a
    monolithic tool. Specifically:
    *   autocxx uses bindgen to generate a description of the C++ API that it
        can parse easily (as opposed to trying to parse C++ headers either
        directly or using Clang APIs). Since bindgen was not intended for this
        purpose, its output lacks some information that autocxx needs, so
        autocxx [has forked](https://crates.io/crates/autocxx-bindgen) bindgen
        to adapt it to its needs. The forked version emits additional
        information about the C++ API in the form of attributes attached to
        various API elements.
    *   bindgen in turn is built on the libclang API, which doesn’t surface all
        of the functionality available through Clang’s C++ API. Adding features
        to libclang requires additional effort and has a 6 month lead time to
        appear in a stable release (to become eligible to be used from bindgen).
    *   When errors occur, it can be hard to figure out which of the components
        is responsible.
    *   Adding features can require touching multiple components, which requires
        commits to multiple repositories.

**Evaluation**

We initially intended to use autocxx to prototype various interop ideas and
potentially as a basis for a field trial. We still believe this would be
feasible, but after trying to modify autocxx and its bindgen fork during an
internal C++/Rust interop study, we feel that autocxx’s complex architecture is
enough of an impediment that we could achieve our goals with less total effort
by creating an interop tool from scratch that consists of a single codebase and
uses the Clang C++ API to directly interface with Clang.

[^1]: Doing so would require either generating C++ source code or interfacing
    deeply enough with Clang to generate object code for inline functions and
    template instantiation.
[^2]: And tricks such as suitable type conversions that force the C++ compiler
    to perform appropriate checks at compile time.
