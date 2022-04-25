## Snippet of C++ to Rust migration

## Overview

This doc contains code snippets of C++ code and how it would look when
automatically translated by a tool, both the first (unsafe) version and after
some automated cleanup passes. Each subsection focus on a specific topic. Unless
specified otherwise, we assume all C++ code is present in a header file.

We also don't consider the possibility of name collision (including with
reserved keywords in Rust). We will use the same naming convention as the
C++/Rust interop, and use a local scope for any extra temporaries we might need.
Lastly, we assume some sort of annotation in C++ code about whether a pointer is
nullable or never null.

## Automated cleanup passes

We assume the first pass would convert the C++ code to an unsafe version of Rust
as similar as possible to the original code. This includes:

  - Using raw pointers instead of references, keeping the same behavior if they
    are nullptr and passing along any lifetime annotation present in C++ code
  - Using wrapping unsigned arithmetic
  - Replacing any explicit or implicit C++ casts by Rust casts that truncate if
    the content doesn't fit
  - Implementing `T::Drop` for types with different destruction order between
    C++ and Rust (structs, tuples, arrays/owned slices and enum variants), to
    keep the same destruction order as in the C++ implementation. This is only
    done if the type has 2+ non-trivially-destructible fields.

After the first pass is done, we can run multiple cleanup passes, running tests
in each pass to see if they respect expected behavior. Some of them are safe to
be applied automatically, while for others we might prefer to require human
review (we should strive to make the latter simple and fast to review).

Automated passes include:

  - Replace raw pointers with lifetime annotations by references, checking for
    aliasing issues using the borrow checker. Nullable raw pointers would be
    replaced by an Option containing a reference.
  - Rename class fields to not have a trailing underscore.
  - Move any automatically-generated nested statements to separate statements.
    Example includes unsafe blocks automatically added inside other expressions
    when dealing with raw pointers and (possibly) constructor calls of
    non-primitive types. This should probably be done after the other automatic
    cleanup passes, since some of them might remove unsafe blocks nested inside
    other statements, thus making this step unnecessary.

Passes that require human review include:

  - Replace wrapping arithmetic by normal arithmetic that panics on overflow
  - Converting raw pointers without lifetime annotations into references with
    elided lifetime annotations (if it passes the borrow checker aliasing rules)
  - Converting raw pointers with lifetime annotations into references when it
    requires usage of `RefCell`.
  - Removing automatically-generated `T::Drop` implementations if the user
    confirms that the destruction order does not matter for the type T.

### Converting raw pointers into references

The cleanup pass to convert Rust raw pointers that have a lifetime annotation
into references is probably the most complicated one, because we need guarantees
that opaque functions (including FFI calls) don't alias pointers. This also
includes objects that contain at least a pointer T\* and either another field of
the same type T, another pointer of the same type T\* or another type that
contains T inside. Our strategy here is multifold:

  - We will create a specific annotation for C++ code to tag an argument of a
    function that can be aliased. If Rust code calls this function passing a
    pointer, we assume the pointer can break aliasing rules, so both this
    pointer and any other pointer that aliases it in Rust code will not be
    refactored into references (they will be kept as raw pointers).
  - For pointers that are not tagged with this annotation we run some static
    analysis (at conversion time) to check that the transitive closure of all
    function calls from Rust into C++ that take a pointer/reference do not
    introduce aliasing. If it finds that a function F() (called directly from
    Rust) introduces aliasing--either because it introduces alias itself or
    because it indirectly calls a function that does it--we will not allow the
    Rust code to import F(). In this case the user should either mark those
    arguments of the function F() with the annotation mentioned in the previous
    item (and use raw pointers in Rust) or, if possible, should refactor F() not
    to alias.

## Code snippets

All code below assumes C++ pointers are annotated as not nullable, unless stated
otherwise.

### Arithmetic

#### Example 1

Original C++ code:

``` cpp
#include <stdint.h>

uint32_t AddAndCast(uint64_t x, uint64_t y) {
  return x + y;
}
```

Rust code after the first pass:

``` rust
pub fn add_and_cast(x: u64, y: u64) -> u32 {
  x.wrapping_add(y) as u32
}
```

Rust code after the arithmetic cleanup pass, triggered manually:

``` rust
pub fn add_and_cast(x: u64, y: u64) -> u32 {
  (x + y) as u32
}
```

### Side effects

#### Example 1

Original C++ code:

``` cpp
#include <stdint.h>

void UseUInt32T(uint32_t x);

uint32_t UsePtrs(uint32_t* x, uint32_t * $a y) {
  UseUInt32T((*x)++);
  UseUInt32T(++(*x));
  UseUInt32T((*y));
  return *x;
}
```

Rust code after the first pass:

``` rust
fn use_u_int32_t(x: u32) {
  // translated based on the implementation in the .cc file or
  // imported from another crate
}

pub fn use_ptrs(x: *mut u32, y: *mut /* $a */ u32) -> u32 {
  use_u_int32_t(
    unsafe {
      // c++ version:
      // (*x)++
      let temp1 = *x;
      *x += 1;
      temp1
    }
  );
  use_u_int32_t(
    unsafe {
      // c++ version:
      // ++(*x)
      *x += 1;
      *x
    }
  );
  use_u_int32_t(
    unsafe {
      // c++ version:
      // (*y)
      *y
    }
  );
  unsafe {
      // c++ version:
      // (*x)
      *x
    }
}
```

Rust code after converting raw pointers with lifetime annotations to references:

``` rust
fn use_u_int32_t(x: u32) {
  // translated based on the implementation in the .cc file or
  // imported from another crate
}

pub fn use_ptrs(x: *mut u32, y: &'a mut u32) -> u32 {
  use_u_int32_t(
    unsafe {
      // c++ version:
      // (*x)++
      let temp1 = *x;
      *x += 1;
      temp1
    }
  );
  use_u_int32_t(
    unsafe {
      // c++ version:
      // ++(*x)
      *x += 1;
      *x
    }
  );
  use_u_int32_t(*y);
  unsafe {
      // c++ version:
      // (*x)
      *x
    }
}
```

Rust code after pass to move nested blocks to separate line:

``` rust
fn use_u_int32_t(x: u32) {
  // translated based on the implementation in the .cc file or
  // imported from another crate
}

pub fn use_ptrs(x: *mut u32, y: &'a mut u32) -> u32 {
  let temp1 = unsafe {
      // c++ version:
      // (*x)++
      let temp1 = *x;
      *x += 1;
      temp1
    };
  use_u_int32_t(temp1);
  let temp2 = unsafe {
      // c++ version:
      // ++(*x)
      *x += 1;
      *x
    };
  use_u_int32_t(temp2);
  use_u_int32_t(*y);
  unsafe {
      // c++ version:
      // (*x)
      *x
    }
}
```

#### Example 2  - pointer aliasing in Rust code

Original C++ code:

``` cpp
#include <stdint.h>

uint32_t CreateAlias(uint32_t* x, uint32_t * $a y) {
  y = x;
  return *y;
}
```

Rust code after the first pass:

``` rust
pub fn create_alias(x: *mut u32, y: *mut /* $a */ u32) -> u32 {
  y = x;
  unsafe {
      // c++ version:
      // (*y)
      *y
    }
}
```

Since `CreateAlias` does alias y and x, we won't be able to convert raw pointers
with lifetime annotations (in this case y) to references, because x has no
lifetime annotation. If both had a matching lifetime annotation the tool would
try to convert them to references.

#### Example 3 - annotated aliasing function that takes pointers

Original C++ code:

``` cpp
#include <stdint.h>

void AnnotatedFfiFunctionThatCreatesAlias(
    /* alias_tag */ uint32_t* x, /* alias_tag */ uint32_t* y);

uint32_t UsePtrs(uint32_t * $a x, uint32_t * $b y) {
  AnnotatedFfiFunctionThatCreatesAlias(x, y);
  return *y;
}
```

Rust code after the first pass:

``` rust
import crate_name::annotated_ffi_function_that_creates_alias;
// signature:
// pub fn annotated_ffi_function_that_creates_alias(x: *mut u32, y: *mut u32);

pub fn use_ptrs(x: *mut /* $a */ u32, y: *mut /* $b */ u32) -> u32 {
  annotated_ffi_function_that_creates_alias(x, y);
  unsafe {
      // c++ version:
      // (*y)
      *y
    }
}
```

Since `annotated_ffi_function_that_creates_alias` has annotations in both
arguments that it introduces alias, we will not run the cleanup pass to convert
the raw pointers into references.

#### Example 4 - non-annotated aliasing function that takes object

Original C++ code:

``` cpp
#include <stdint.h>
#include other_file.h

struct S [[lifetime_param(a)]] {
  uint32_t *$a x;
  uint32_t *$a y;
}

S createS(uint32_t * $a x, uint32_t * $a y) {
  S s = S{x, y};
  MakeInternalAlias(&s);
  return s;
}

//////////////////////////////////////////////////
// other_file.h
//////////////////////////////////////////////////

// Assume the following function is not being converted into Rust right now (it
// will be called via FFI).
void MakeInternalAlias(S* s) {
  s.x = s.y;
}
```

The automated conversion tool <span style="text-decoration:underline;">would
try</span> to generate the following Rust code:

``` rust
import crate_name::make_internal_alias;
// signature:
// pub fn make_internal_alias(s: *mut /* $a */ S);

struct S /* $a */ {
  x: *mut /* $a */ u32,
  y: *mut /* $a */ u32,
}

pub fn create_s(x: *mut /* $a */ u32, y: *mut /* $a */ u32) -> S {
  let mut s = S {x, y};
  make_internal_alias(&s);
  s
}
```

<span style="text-decoration:underline;">Nonetheless the tool would fail</span>,
since `make_internal_alias` is not annotated as creating alias and the syntactic
analysis would recognize it does so. Therefore the tool would not be able to
import `make_internal_alias` into Rust code and would either ask the user to
annotate the arguments of `MakeInternalAlias` as being aliased or to make the
function not alias (which, in this case, is not possible at all).

#### Example 5 - aliasing function that creates object

Original C++ code:

``` cpp
#include <stdint.h>
#include other_file.h

struct S [[lifetime_param(a)]] {
  uint32_t *$a x;
  uint32_t *$a y;
}

S createS(uint32_t * $a x) {
  return createSWithAlias(x);
}

//////////////////////////////////////////////////
// other_file.h
//////////////////////////////////////////////////

// Assume the following function is not being converted into Rust right now (it
// will be called via FFI).
S CreateSWithAlias(uint32_t * $a x) {
  return S{x, x};
}
```

The automated conversion tool <span style="text-decoration:underline;">would
try</span> to generate the following Rust code:

``` rust
import crate_name::create_s_with_alias;
// signature:
// pub fn create_s_with_alias(x: *mut /* $a */ u32);

struct S /* $a */ {
  x: *mut /* $a */ u32,
  y: *mut /* $a */ u32,
}

pub fn create_s(x: *mut /* $a */ u32) -> S {
  create_s_with_alias(x)
}
```

But <span style="text-decoration:underline;">the tool would fail</span>, since
the static analysis would detect that `create_s_with_alias` creates an alias of
x. Therefore once again the tool would not be able to import
`create_s_with_alias` and would either ask the user to annotate the argument of
`CreateSWithAlias` as being aliased or to make it not alias (which, in this
case, is not possible at all).

### Destructor order

Struct fields in Rust are dropped in declaration order, while in C++ they are
dropped in reverse declaration order. This is also true for enum variants,
tuples, arrays and owned slices. To make matters worse, if Rust panics during
construction of the object, then the fields are dropped in reverse order of
declaration. In other words, the field drop order in Rust depends on when the
object is dropped (i.e. during panic unwind or not).

Therefore the safest solution is for the tool to create `T::Drop`
implementations for any type with 2+ non-trivially-destructible types in the
first pass. The `T::Drop` implementation for types with 2+
non-trivially-destructible fields calls all drop methods in the object fields in
reverse order of declaration (i.e. matching the C++ destruction order). This
method should also wrap `ManuallyDrop<T>` on each of the
(non-trivially-destructible) struct fields and call `MemoryDrop::drop` on them
(analogously for other types mentioned above). For this to work with inlined
arrays/tuples it might also be necessary to wrap it in a Rust struct in order to
be able to implement `Drop` on them. If there is a manually-implemented
destructor for the class, this destructor code will be converted into Rust and
be executed before the `MemoryDrop::drop` calls in the Drop implementation.
Lastly, if the type contains at most one non-trivially destructible field we can
ignore destruction order altogether.

Afterwards, in a cleanup pass, the tool will ask the user to specify whether the
destruction order of each array/owned slice, tuple and struct (and C++ class)
fields matters. If not, we can delete the `T::Drop` implementation, the
`ManuallyDrop` and inlined-arrays/tuple struct wrappers, and just depend on Rust
behavior, even if it doesn't match the C++ one. If the user confirms that the
destruction order matters, then we keep the `T::Drop` implementation for those
types.

#### Example 1:

As a first example, let's look at a C++ class where the destruction order
matters. In the example below `MyFileReader` stores the file path (as a string)
and a `FileHandle`, while the latter stores a `string_view` to the file path.
Therefore the `FileHandle` should be destructed before the file path string.

Original C++ code:

``` cpp
class FileHandle {
  FileHandle(absl::string_view file_path): file_path_(file_path) {
    // ...
  }

  ~FileHandle() {
    // ...
  }

  //...

private:
  absl::string_view file_path_;
  //...
}

class MyFileReader {
  MyFileReader(std::string file_path) {
    this.file_path_ = file_path;
    this.file_handle_ = MyFileHandle(file_path_)
  }

  ~MyFileReader() {
    // <user-specified destructor logic>
  }

  string_view ReadLine() {
    // ...
  }

private:
  const std::string file_path_;
  FileHandle file_handle_;
}
```

Therefore the tool will realize that the destruction order of `MyFileReader`
fields might matter (since both `std::string` and `MyFileHandle` are
non-trivially-destructible) and should match the C++ one. Therefore the
automated conversion tool would generate the following Rust code (assume here
that `string_view` is a type in Rust):

``` rust
struct FileHandle {
  file_path: string_view,
  //...
}

impl FileHandle {
  pub fn new(string_view file_path) {
    FileHandle {
      file_path,
      // ...
    }
  }
}

impl Drop for FileHandle {
  fn drop(&mut self) {
    // ...
  }
}


struct MyFileReader {
  file_path: ManuallyDrop<String>,
  file_handle: ManuallyDrop<FileHandle>,
}

impl MyFileReader {
  pub fn new(String file_path) {
    MyFileReader {
      file_path,
      MyFileHandle(file_path),
    }
  }

  pub string_view ReadLine(&mut self) {
    // ...
  }

}

impl Drop for MyFileReader {
  fn drop(&mut self) {
    // <user-specified destructor logic converted into Rust>
    ManuallyDrop::drop(self.file_handle);
    ManuallyDrop::drop(self.file_path);
  }
}
```

#### Example 2: tuples and arrays

For tuples/arrays the tool behavior would be exactly the same as for a struct
(see example above). The only difference is that in a first step the tool will
create a wrapper struct around the tuple/array (with a single tuple/array public
field), and the tuple/array field inside would contain `ManuallyDrop<T>`, where
T was the type contained inside the typle/array. The drop method for this struct
will be a reverse for-loop, dropping each element of the tuple/array (from last
to first) by calling `ManuallyDrop::drop` on it.

### Temporaries & Order of function call within expression

In [C++ the evaluation
order](https://en.cppreference.com/w/cpp/language/eval_order) (including both
value computation and side effects) of temporaries in the same expression are,
in general, unspecified (i.e. they can happen at whatever order, and may
interleave). In Rust things are [not so well
documented](https://github.com/rust-lang/reference/issues/248): we only know
what Niko said, which is that Rust evaluates things roughly left-to-right,
except for assignments, which are right-to-left. Therefore we should be
concerned about cases where C++ evaluates right-to-left and Rust evaluates
left-to-right.

The only such example is new-assignments in C++, where the call to `new` is
(since C++17) sequenced-before the evaluation of constructor arguments, while in
Rust the order of evaluation of the memory allocation and the struct fields
evaluation is not clear. Nonetheless I can't think of a single interesting
example where the constructor fails and behaviors differ between C++ and Rust.

Of course there are cases where C++ evaluation order is unspecified or
indeterminately sequenced and Rust's evaluation order is specified (example:
evaluation order in expression). Nonetheless this isn't very interesting in the
context of converting non-buggy C++ code to Rust (since non-buggy C++ code
should not rely on an unspecified evaluation order). Therefore I believe the
tool doesn't have to worry about anything specific to these topics.

### Open questions

  - What to do with variadic functions? Replace the varargs by an array has the
    problem that the array size must be known at compile time. Vec implies a
    memory allocation. Probably slices are the natural way to go for the
    signature, and all callers could create an array at the call site.

  - What to do about function overloading? Rust does not allow functions with
    the same name and different signatures. Maybe some naming convention should
    be enough.

  - What about `void*`? Devin mentioned the interop tool will probably use
    extern types. Another option would be to use `*mut libc::c_void` or
    `*libc::c_void`. We should probably not use `*Void`, since it's a zero-sized
    type (which in Rust has 0 bytes, and in C++ has 1 byte, so it can't move
    across FFI boundaries). Maybe since it's a pointer to `Void` it's OK (ptr
    would always occupy word-size bytes, no matter the type it points to)?

  - How to handle class inheritance? I guess pure abstract classes in C++ are
    more straightforward to convert to traits, but I'm not sure about
    non-abstract classes.

  - How to convert C++ namespaces to Rust?

  - How to convert C++ templates to Rust?
