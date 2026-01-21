<style>
.multicol {
  width: 60vw;
  grid-column-gap: 0 !important;
  border-bottom: 1px solid #ebebeb;
}
.multicol * {
  padding: 0.25rem;
}
.table-header {
  background-color: #bbdefb;
}
</style>

<section class="multicol">

<section>

#### Function parameter and return types

</section>

</section>

<section class="multicol">

<section class="table-header">**C++**</section>

<section class="table-header">**Rust (Non-Rust-movable)**</section>

<section class="table-header">**Rust (Rust-movable)**</section>

</section>

<section class="multicol">

<section>

```c++ {.no-copy}
[const] T
```

</section>

<section>

```rust {.no-copy}
Ctor![T]
```

</section>

<section>

```rust {.no-copy}
T
```

</section>

</section>

<section class="multicol">

<section>

```c++ {.no-copy}
T&&
```

</section>

<section>

```rust {.no-copy}
RvalueReference<T>
```

</section>

<section>

```rust {.no-copy}
&mut T
```

</section>

</section>

<section class="multicol">

<section>

```c++ {.no-copy}
const T&&
```

</section>

<section>

```rust {.no-copy}
ConstRvalueReference<T>
```

</section>

<section>

```rust {.no-copy}
&T
```

</section>

</section>

<section class="multicol">

<section>

```c++ {.no-copy}
T&
```

</section>

<section>

```rust {.no-copy}
Pin<&mut T>
```

</section>

<section>

```rust {.no-copy}
&mut T
```

</section>

</section>

<section class="multicol">

<section>

```c++ {.no-copy}
const T&
```

</section>

<section>

```rust {.no-copy}
&T
```

</section>

<section>

```rust {.no-copy}
&T
```

</section>

</section>

<section class="multicol">

<section>

#### Basic object creation and storage

</section>

</section>

<section class="multicol">

<section class="table-header">**C++**</section>

<section class="table-header">**Rust (Non-Rust-movable)**</section>

<section class="table-header">**Rust (Rust-movable)**</section>

</section>

<section class="multicol">

<section>

```c++ {.no-copy}
auto x = T();
```

</section>

<section>

```rust {.no-copy}
let x = emplace!(T::ctor_new(()));
```

</section>

<section>

```rust {.no-copy}
let x = T::default();
```

</section>

</section>

<section class="multicol">

<section>

```c++ {.no-copy}
auto x = y;
```

</section>

<section>

```rust {.no-copy}
let x = emplace!(ctor::copy(&*y));
// sugar for:
let x = emplace!(
  T::ctor_new(&*y));
```

</section>

<section>

```rust {.no-copy}
let x = y.clone();
```

</section>

</section>

<section class="multicol">

<section>

```c++ {.no-copy}
auto x = std::move(y);
```

</section>

<section>

```rust {.no-copy}
let x = emplace!(ctor::mov(y));
// sugar for:
let x = emplace!(
  T::ctor_new(ctor::mov(y)));
```

</section>

<section>

```rust {.no-copy}
let x = y;
```

</section>

</section>

<section class="multicol">

<section>

```c++ {.no-copy}
auto x = T(1, 2);
```

</section>

<section>

```rust {.no-copy}
let x = emplace!(
  T::ctor_new((1, 2)));
```

</section>

<section>

```rust {.no-copy}
let x = T::new(1, 2);
```

</section>

</section>

<section class="multicol">

<section>

```c++ {.no-copy}
// Takes T or const T&
TakesTemporary(T())
```

</section>

<section>

```rust {.no-copy}
// if it takes a value:
TakesTemporary(T::ctor_new(()))

// if it takes a const reference:
TakesTemporary(
  &*emplace!(T::ctor_new(())))
// (Materialization is explicit!)
```

</section>

<section>

```rust {.no-copy}
// if it takes a value
TakesTemporary(T::default())

// if it takes a const reference:
TakesTemporary(&T::default())
```

</section>

</section>

<section class="multicol">

<section>

```c++ {.no-copy}
// Takes T&&
TakesMutableTemporary(T())
```

</section>

<section>

```rust {.no-copy}
TakesMutableTemporary(
  emplace!(T::ctor_new()))
```

</section>

<section>

```rust {.no-copy}
TakesMutableTemporary(
  &mut T::default())
```

</section>

</section>

<section class="multicol">

<section>

```c++ {.no-copy}
T Source() { return T(); }
```

</section>

<section>

```rust {.no-copy}
fn Source() -> Ctor![T] {
  // copy-elided and lazy.
  // constructor won't run until
  // the returned Ctor is "emplaced".
  T::ctor_new(())
}
```

</section>

<section>

```rust {.no-copy}
fn Source() -> T { T::default() }
```

</section>

</section>

<section class="multicol">

<section>

```c++ {.no-copy}
T Source2() {
  T x = T();
  x.Mutate();
  return x;
}
```

</section>

<section>

```rust {.no-copy}
fn Source2() -> Ctor![T] {
  T::ctor_new(()).ctor_then(|x| {
    x.Mutate();
  })
}
```

</section>

<section>

```rust {.no-copy}
fn Source2() -> T {
  let mut x = T::default();
  x.Mutate();
  x
}
```

</section>

</section>

<section class="multicol">

<section>

```c++ {.no-copy}
void Sink(T x) {
  x.Something();
}
```

</section>

<section>

```rust {.no-copy}
fn Sink(x: Ctor![T]) {
  emplace!(x).Something()
}
```

</section>

<section>

```rust {.no-copy}
fn Sink(x: T) {
  x.Something()
}
```

</section>

</section>

<section class="multicol">

<section>

#### Compound data types

</section>

</section>

<section class="multicol">

<section class="table-header">**C++**</section>

<section class="table-header">**Rust (Non-Rust-movable)**</section>

<section class="table-header">**Rust (Rust-movable)**</section>

</section>

<section class="multicol">

<section>

```c++ {.no-copy}
struct T { U field; };
```

</section>

<section>

```rust {.no-copy}
#[recursively_pinned]
struct T { field: U }
```

</section>

<section>

```rust {.no-copy}
struct T { field: U }
```

</section>

</section>

<section class="multicol">

<section>

```c++ {.no-copy}
std::make_unique<T>()
```

</section>

<section>

```rust {.no-copy}
Box::emplace(T::ctor_new(()))
```

</section>

<section>

```rust {.no-copy}
Box::new(T::default())
```

</section>

</section>

<section class="multicol">

<section>

```c++ {.no-copy}
T x = {.field = std::move(y)}
```

</section>

<section>

```rust {.no-copy}
let x = emplace!(ctor!(
  T {field: ctor::mov(y)}));
```

</section>

<section>

```rust {.no-copy}
let x = T { field: y };
```

</section>

</section>

<section class="multicol">

<section style="background-color: #efefef">

N/A: C++ doesn't have tuple-structs. </section>

<section>

```rust {.no-copy}
#[recursively_pinned]
struct T2(U);
```

</section>

<section>

```rust {.no-copy}
struct T2(U);
```

</section>

</section>

<section class="multicol">

<section style="background-color: #efefef">

N/A: C++ doesn't have tuple-structs. </section>

<section>

```rust {.no-copy}
let x = emplace!(ctor!(T2(ctor::mov(y))));
```

</section>

<section>

```rust {.no-copy}
let x = T2(y);
```

</section>

</section>

<section class="multicol">

<section>

#### Object access and mutation

</section>

</section>

<section class="multicol">

<section class="table-header">**C++**</section>

<section class="table-header">**Rust (Non-Rust-movable)**</section>

<section class="table-header">**Rust (Rust-movable)**</section>

</section>

<section class="multicol">

<section>

```c++ {.no-copy}
x.ConstMethod()
```

</section>

<section>

```rust {.no-copy}
x.ConstMethod()
```

</section>

<section>

```rust {.no-copy}
x.ConstMethod()
```

</section>

</section>

<section class="multicol">

<section>

```c++ {.no-copy}
x.NonConstMethod()
```

</section>

<section>

```rust {.no-copy}
x.as_mut().NonConstMethod()
```

</section>

<section>

```rust {.no-copy}
x.NonConstMethod()
```

</section>

</section>

<section class="multicol">

<section>

```c++ {.no-copy}
x = {};
```

</section>

<section>

```rust {.no-copy}
x.as_mut().assign(T::ctor_new(()));
```

</section>

<section>

```rust {.no-copy}
x = T::default();
```

</section>

</section>

<section class="multicol">

<section>

```c++ {.no-copy}
x = y;
```

</section>

<section>

```rust {.no-copy}
x.as_mut().assign(&*y);
```

</section>

<section>

```rust {.no-copy}
x = y.clone();
```

</section>

</section>

<section class="multicol">

<section>

```c++ {.no-copy}
x = std::move(y);
```

</section>

<section>

```rust {.no-copy}
x.as_mut().assign(ctor::mov(y));
```

</section>

<section>

```rust {.no-copy}
x = y;
```

</section>

</section>

<section class="multicol">

<section>

```c++ {.no-copy}
x = New();  // returns T by value
```

</section>

<section>

```rust {.no-copy}
x.as_mut().assign(ctor::mov(
  emplace!(New())));
```

</section>

<section>

```rust {.no-copy}
x = New();
```

</section>

</section>

<section class="multicol">

<section>

```c++ {.no-copy}
x.~T();
new (&x) T();
```

</section>

<section>

```rust {.no-copy}
x.as_mut().reconstruct(
  T::ctor_new(()));
```

</section>

<section>

```rust {.no-copy}
x = T::default();
```

</section>

</section>

<section class="multicol">

<section>

```c++ {.no-copy}
x.y = …;  // x is a struct
```

</section>

<section>

```rust {.no-copy}
x.as_mut().project_pin()
  .y.assign(…);
```

</section>

<section>

```rust {.no-copy}
x.y = …;
```

</section>

</section>

--------------------------------------------------------------------------------
