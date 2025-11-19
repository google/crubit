### The Cheat Sheet {#the-cheat-sheet}

As a quick tl;dr, here is the end state we will reach, using the crate name
`ctor`.

Each pattern is a triple: C++ version, Rust version for non-relocatable types,
and the Rust version when dealing with idiomatic, normal-looking Rust types.

A cell is colored white if the implementation already exists, yellow if I am in
the process of implementing it right now :), orange if it is possible, but
unimplemented and out of scope, red if impossible. (Spoiler alert: currently,
nothing is considered impossible.)

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

<section class="table-header">**Rust (non-relocatable)**</section>

<section class="table-header">**Rust (relocatable)**</section>

</section>

<section class="multicol">

<section>

```c++ {.no-copy}
[const] T
```

</section>

<section>

```rs {.no-copy}
impl Ctor<Output=T> // a lazy construction type
```

</section>

<section>

```rs {.no-copy}
T // T also implements Ctor<Output=T>
  // when T is relocatable (i.e. Unpin).
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

```rs {.no-copy}
RvalueReference<T>
```

</section>

<section>

```rs {.no-copy}
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

```rs {.no-copy}
ConstRvalueReference<T>
```

</section>

<section>

```rs {.no-copy}
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

```rs {.no-copy}
Pin<&mut T>
```

</section>

<section>

```rs {.no-copy}
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

```rs {.no-copy}
&T
```

</section>

<section>

```rs {.no-copy}
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

<section class="table-header">**Rust (non-relocatable)**</section>

<section class="table-header">**Rust (relocatable)**</section>

</section>

<section class="multicol">

<section>

```c++ {.no-copy}
auto x = T();
```

</section>

<section>

```rs {.no-copy}
let x = emplace!(T::ctor_new(()));
```

</section>

<section>

```rs {.no-copy}
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

```rs {.no-copy}
let x = emplace!(ctor::copy(&*y));
// sugar for:
let x = emplace!(T::ctor_new(&*y));
```

</section>

<section>

```rs {.no-copy}
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

```rs {.no-copy}
let x = emplace!(ctor::mov(y));
// sugar for:
let x = emplace!(T::ctor_new(ctor::mov(y)));
```

</section>

<section>

```rs {.no-copy}
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

```rs {.no-copy}
let x = emplace!(T::ctor_new((1, 2)));
```

</section>

<section>

```rs {.no-copy}
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

```rs {.no-copy}
// if it takes a value:
TakesTemporary(T::ctor_new(()))

// if it takes a const reference:
TakesTemporary(&*emplace!(T::ctor_new(())))
// (Materialization is explicit!)
```

</section>

<section>

```rs {.no-copy}
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

```rs {.no-copy}
TakesMutableTemporary(emplace!(T::ctor_new()))
```

</section>

<section>

```rs {.no-copy}
TakesMutableTemporary(&mut T::default())
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

```rs {.no-copy}
fn Source() -> impl Ctor<Output=T> {
  // copy-elided and lazy -- constructor won't
  // run until the returned Ctor is "emplaced".
  T::ctor_new(())
}
```

</section>

<section>

```rs {.no-copy}
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

```rs {.no-copy}
fn Source2() -> impl Ctor<Output=T> {
  T::ctor_new(()).ctor_then(|x| {
    x.Mutate();
  })
}
```

</section>

<section>

```rs {.no-copy}
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

```rs {.no-copy}
fn Sink(x: impl Ctor<Output=T>) {
  emplace!(x).Something()
}
```

</section>

<section>

```rs {.no-copy}
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

<section class="table-header">**Rust (non-relocatable)**</section>

<section class="table-header">**Rust (relocatable)**</section>

</section>

<section class="multicol">

<section>

```c++ {.no-copy}
struct T { U field; };
```

</section>

<section>

```rs {.no-copy}
#[recursively_pinned]
// TODO: show the derive
struct T { field: U }
```

</section>

<section>

```rs {.no-copy}
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

```rs {.no-copy}
Box::emplace(T::ctor_new(()))
```

</section>

<section>

```rs {.no-copy}
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

```rs {.no-copy}
let x = emplace!(ctor!(T {field: ctor::mov(y)}));
```

</section>

<section>

```rs {.no-copy}
let x = T { field: y };
```

</section>

</section>

<section class="multicol">

<section style="background-color: #efefef">

N/A: C++ doesn't have tuple-structs. </section>

<section>

```rs {.no-copy}
#[recursively_pinned]
// TODO: show the derive
struct T2(U);
```

</section>

<section>

```rs {.no-copy}
struct T2(U);
```

</section>

</section>

<section class="multicol">

<section style="background-color: #efefef">

N/A: C++ doesn't have tuple-structs. </section>

<section>

```rs {.no-copy}
let x = emplace!(ctor!(T2(ctor::mov(y))));
```

</section>

<section>

```rs {.no-copy}
let x = T2(y);
```

</section>

</section>

<section class="multicol">

<section style="background-color: #efefef">

N/A: C++ doesn't have sum types. </section>

<section style="background-color: #fce5cd">

```rs {.no-copy}
#[recursively_pinned] // implicitly: #[repr(C)]
// TODO: show the derive
enum Enum{ A(U) }
```

</section>

<section>

```rs {.no-copy}
enum Enum { A(U) }
```

</section>

</section>

<section class="multicol">

<section style="background-color: #efefef">

N/A: C++ doesn't have sum types. </section>

<section style="background-color: #fce5cd">

Could work for a decorated `repr(C)` enum, as above:

```rs {.no-copy}
let x = emplace!(enum_ctor!(Enum::A(ctor::mov(y))));
```

</section>

<section>

```rs {.no-copy}
let x = Enum::A(y);
```

</section>

</section>

<section class="multicol">

<section>

```c++ {.no-copy}
auto x = std::make_tuple(y);
```

</section>

<section style="background-color: #fce5cd">

We probably need to either make a new tuple type, or straight up re-use C++
std::tuple.

In that case, something like:

```rs {.no-copy}
let x = emplace!(cstd::tuple::ctor_new(
  (ctor::mov(y),)
));
```

</section>

<section>

```rs {.no-copy}
let x = (y,);
```

</section>

</section>

<section class="multicol">

<section>

```c++ {.no-copy}
xs.push_back(std::move(x));
```

</section>

<section style="background-color: #fce5cd">

As with tuple, we must either create a new type, or reuse `std::vector`:

```rs {.no-copy}
xs.push_back(ctor::mov(x));
```

</section>

<section>

```rs {.no-copy}
xs.push(x);
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

<section class="table-header">**Rust (non-relocatable)**</section>

<section class="table-header">**Rust (relocatable)**</section>

</section>

<section class="multicol">

<section>

```c++ {.no-copy}
x.ConstMethod()
```

</section>

<section>

```rs {.no-copy}
x.ConstMethod()
```

</section>

<section>

```rs {.no-copy}
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

```rs {.no-copy}
x.as_mut().NonConstMethod()
```

</section>

<section>

```rs {.no-copy}
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

```rs {.no-copy}
x.as_mut().assign(T::ctor_new(()));
```

</section>

<section>

```rs {.no-copy}
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

```rs {.no-copy}
x.as_mut().assign(&*y);
```

</section>

<section>

```rs {.no-copy}
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

```rs {.no-copy}
x.as_mut().assign(ctor::mov(y));
```

</section>

<section>

```rs {.no-copy}
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

```rs {.no-copy}
x.as_mut().assign(ctor::mov(emplace!(New())));
```

</section>

<section>

```rs {.no-copy}
x = New();
```

</section>

</section>

<section class="multicol">

<section>

```c++ {.no-copy}
x.~T();
// UB if the constructor throws...
new (&x) T();
```

</section>

<section>

```rs {.no-copy}
// note: will abort if constructor panics
x.as_mut().reconstruct(T::ctor_new(()));
```

</section>

<section>

```rs {.no-copy}
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

```rs {.no-copy}

x.as_mut().project().y.as_mut().assign(…);


x.as_mut().y_mut().assign(…)
```

</section>

<section>

```rs {.no-copy}
x.y = …;
```

</section>

</section>

<section class="multicol">

<section>

```c++ {.no-copy}
std::get<0>(tuple) =…;
```

</section>

<section style="background-color: #fce5cd">

depends on representation </section>

<section>

```rs {.no-copy}
tuple.0 = …;
```

</section>

</section>

<section class="multicol">

<section>

```c++ {.no-copy}
xs[0] = std::move(x);
```

</section>

<section style="background-color: #fce5cd">

Depends on representation, but *very* likely:

```rs {.no-copy}
xs[0].as_mut().assign(ctor::mov(x));
```

</section>

<section>

```rs {.no-copy}
xs[0] = x;
```

</section>

</section>

<section class="multicol">

<section>

#### Definitions of constructors and assignment

</section>

</section>

<section class="multicol">

<section class="table-header">**C++**</section>

<section class="table-header">**Rust (non-relocatable)**</section>

<section class="table-header">**Rust (relocatable)**</section>

</section>

<section class="multicol">

<section>

```c++ {.no-copy}
T(const T& other) : x_(other.x_) {}
```

</section>

<section>

```rs {.no-copy}
impl CtorNew<&T> for T {
  type CtorType = impl Ctor<Output = Self>;
  fn ctor_new(other: &T) -> Self::CtorType {
    ctor!( T {
      x: copy(&other.x),
    })
  }
}
```

</section>

<section>

```rs {.no-copy}
impl Clone for T {
  fn clone(&self) -> T {
    T { x: self.x }
  }
}
```

</section>

</section>

<section class="multicol">

<section>

```c++ {.no-copy}
T& operator=(const T& other) {
  x_ = other.x_;
}
```

</section>

<section>

```rs {.no-copy}
impl Assign<&T> for T {
  fn assign(self: Pin<&mut Self>, other: &T) {
    let this = self.project();
    this.x.assign(other.x);
  }
}
```

</section>

<section>

```rs {.no-copy}
impl Clone for T {
  fn clone(&self) -> T {
    T { x: self.x }
  }
}
```

</section>

</section>

<section class="multicol">

<section>

```c++ {.no-copy}
T() = default;
```

</section>

<section style="background-color: #fff2cc">

derive() takes a path, not a type, so one of the following:

```rs {.no-copy}
#[derive(CtorNew_Default)]
```

or:

```rs {.no-copy}
#[derive_constructible(())]
```

or some other syntax -- suggestions welcome!

</section>

<section>

```rs {.no-copy}
#[derive(Default)]
```

</section>

</section>

<section class="multicol">

<section>

```c++ {.no-copy}
T(const T&) = default;
```

</section>

<section style="background-color: #fff2cc">

```rs {.no-copy}
#[derive(CtorNew_Copy)]
```

</section>

<section>

```rs {.no-copy}
#[derive(Clone)]
```

</section>

</section>

<section class="multicol">

<section>

```c++ {.no-copy}
T(T&&) = default;
```

</section>

<section style="background-color: #fff2cc">

```rs {.no-copy}
  #[derive(CtorNew_Move)]
```

</section>

<section style="background-color: #efefef">

N/A </section>

</section>

<section class="multicol">

<section>

```c++ {.no-copy}
T& operator=(const T&) = default;
```

</section>

<section style="background-color: #fff2cc">

```rs {.no-copy}
  #[derive(Assign_Copy)]
```

</section>

<section>

```rs {.no-copy}
#[derive(Clone)]
```

</section>

</section>

<section class="multicol">

<section>

```c++ {.no-copy}
T& operator=(T&&) = default;
```

</section>

<section style="background-color: #fff2cc">

```rs {.no-copy}
#[derive(Assign_Move)]
```

</section>

<section style="background-color: #efefef"> N/A </section>

</section>

--------------------------------------------------------------------------------
