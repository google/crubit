// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

pub struct GenericStruct<T> {
    pub value: T,
}

// ============================================================================
// 1. Inherent Methods & `impl` Block Selection
// ============================================================================

impl<T> GenericStruct<T> {
    pub fn new(value: T) -> Self {
        Self { value }
    }

    pub fn get_ref(&self) -> &T {
        &self.value
    }

    pub fn set(&mut self, value: T) {
        self.value = value;
    }

    pub fn into_inner(self) -> T {
        self.value
    }

    pub const VALUE_SIZE: usize = core::mem::size_of::<T>();

    pub fn type_size() -> usize {
        core::mem::size_of::<T>()
    }
}

impl<T: PartialEq + Default> GenericStruct<T> {
    pub fn is_default(&self) -> bool {
        self.value == T::default()
    }
}

impl<T: core::ops::Deref> GenericStruct<T> {
    pub fn deref_value(&self) -> &T::Target {
        &self.value
    }
}

impl GenericStruct<i32> {
    pub const ZERO: Self = Self { value: 0 };

    pub fn add_one(&self) -> i32 {
        self.value + 1
    }

    pub fn concrete_id() -> i32 {
        32
    }
}

impl GenericStruct<i64> {
    pub fn concrete_id() -> i32 {
        64
    }
}

impl<T> GenericStruct<T> {
    // Should not receive bindings because of the predicates on the method itself.
    pub fn t_iter(self) -> impl Iterator<Item = T>
    where
        T: IntoIterator<Item = T>,
    {
        self.value.into_iter()
    }

    // Should not receive bindings because the method has its own type parameter `U`.
    pub fn map<U>(self, f: fn(T) -> U) -> GenericStruct<U> {
        GenericStruct { value: f(self.value) }
    }

    // Method with an explicit lifetime parameter.
    pub fn borrow_with_explicit_lt(&self) -> &T {
        &self.value
    }
}

pub struct MultiGenericStruct<T, U> {
    pub first: T,
    pub seconds: Vec<U>,
}

impl<T: Copy> MultiGenericStruct<T, i32> {
    pub fn first_and_len(&self) -> (T, usize) {
        (self.first, self.seconds.len())
    }
}

#[derive(PartialEq, Default)]
pub struct PartialEqAndDefault(pub i32);

impl core::ops::Deref for PartialEqAndDefault {
    type Target = i32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn instantiate_generic_struct_at_i32() -> GenericStruct<PartialEqAndDefault> {
    GenericStruct::new(PartialEqAndDefault(10))
}

pub fn instantiate_generic_struct_at_primitive_i32() -> GenericStruct<i32> {
    GenericStruct::new(10)
}

pub struct NotPartialEqOrDefault(pub i32);

pub fn instantiate_generic_struct_at_not_partial_eq_or_default(
) -> GenericStruct<NotPartialEqOrDefault> {
    GenericStruct::new(NotPartialEqOrDefault(10))
}

pub fn instantiate_multi_generic_at_bool_i32() -> MultiGenericStruct<bool, i32> {
    MultiGenericStruct { first: true, seconds: vec![1, 2, 3] }
}

pub fn instantiate_multi_generic_at_bool_f64() -> MultiGenericStruct<bool, f64> {
    MultiGenericStruct { first: false, seconds: vec![1.0] }
}

// ============================================================================
// 2. Trait Impls for Generics (Derives, Drop, Operators, Conversions, Display)
// ============================================================================

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DerivedGeneric<T> {
    pub value: T,
}

pub fn instantiate_derived_generic_at_i32() -> DerivedGeneric<i32> {
    DerivedGeneric { value: 42 }
}

pub fn instantiate_derived_generic_at_not_partial_eq_or_default(
) -> DerivedGeneric<NotPartialEqOrDefault> {
    DerivedGeneric { value: NotPartialEqOrDefault(42) }
}

#[derive(Default)]
pub struct GenericWithDrop<T> {
    pub value: T,
}

impl<T> Drop for GenericWithDrop<T> {
    fn drop(&mut self) {}
}

// Movable in C++ because `GenericWithDrop<i32>` implements `Default`.
pub fn instantiate_generic_with_drop_movable() -> GenericWithDrop<i32> {
    GenericWithDrop { value: 7 }
}

// Not C++-movable because `NotPartialEqOrDefault` does not implement `Default`.
pub fn instantiate_generic_with_drop_unmovable() -> GenericWithDrop<NotPartialEqOrDefault> {
    GenericWithDrop { value: NotPartialEqOrDefault(7) }
}

pub fn accept_generic_with_drop_unmovable(_x: GenericWithDrop<NotPartialEqOrDefault>) {}

impl<T: std::ops::Add<Output = T>> std::ops::Add for GenericStruct<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self { value: self.value + rhs.value }
    }
}

impl<T> From<T> for GenericStruct<T> {
    fn from(value: T) -> Self {
        Self { value }
    }
}

impl<T: Clone> From<GenericWithDrop<T>> for Wrapper<T> {
    fn from(g: GenericWithDrop<T>) -> Self {
        Wrapper(g.value.clone())
    }
}

#[allow(clippy::from_over_into)]
impl<T> Into<GenericWithDrop<T>> for Wrapper<T> {
    fn into(self) -> GenericWithDrop<T> {
        GenericWithDrop { value: self.0 }
    }
}

impl<T: std::fmt::Display> std::fmt::Display for GenericStruct<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GenericStruct({})", self.value)
    }
}

// ============================================================================
// 3. Template Specialization Discovery, Nesting & Ordering
// ============================================================================

// `GenericStruct<u32>` is only referenced transitively as a nested generic field.
pub struct OuterWithNestedGenericField {
    pub field: GenericStruct<GenericStruct<u32>>,
}

// `MultiGenericStruct<i32, i32>` is only discovered via an inherent method on `GenericStruct<i32>`.
impl<T: Clone> GenericStruct<T> {
    pub fn into_multi(&self) -> MultiGenericStruct<T, T> {
        MultiGenericStruct { first: self.value.clone(), seconds: vec![self.value.clone()] }
    }
}

pub type AliasedGenericI64 = GenericStruct<i64>;

pub fn use_aliased_generic(x: AliasedGenericI64) -> AliasedGenericI64 {
    x
}

pub mod submodule {
    pub struct SubGeneric<T> {
        pub item: T,
    }
}
pub use submodule::SubGeneric;
pub use submodule::SubGeneric as RenamedSubGeneric;

pub fn instantiate_sub_generic() -> RenamedSubGeneric<i32> {
    SubGeneric { item: 5 }
}

#[repr(C)]
pub enum GenericEnum<T> {
    Empty,
    Value(T),
}

pub fn instantiate_generic_enum() -> GenericEnum<i32> {
    GenericEnum::Value(99)
}

#[repr(C)]
pub union GenericUnion<T: Copy> {
    pub val: T,
    pub raw: u32,
}

pub fn instantiate_generic_union() -> GenericUnion<u32> {
    GenericUnion { val: 123 }
}

pub struct LifetimeAndTypeGeneric<'a, T> {
    pub reference: &'a T,
}

pub fn instantiate_lifetime_and_type_generic(x: &i32) -> LifetimeAndTypeGeneric<'_, i32> {
    LifetimeAndTypeGeneric { reference: x }
}

// ============================================================================
// 4. Corner Cases & Safety Guards
// ============================================================================

pub struct Wrapper<T>(pub T);

impl<T> GenericStruct<T> {
    // Direct self-nesting (`GenericStruct<T> -> GenericStruct<GenericStruct<T>>`).
    pub fn nest_in_self(self) -> GenericStruct<GenericStruct<T>> {
        GenericStruct { value: self }
    }

    // Inductive growth in type parameter (`GenericStruct<T> -> GenericStruct<Wrapper<T>>`).
    pub fn wrap_type_param(self) -> GenericStruct<Wrapper<T>> {
        GenericStruct { value: Wrapper(self.value) }
    }

    // Self-containment inside another generic (`Self -> Wrapper<Self>`).
    pub fn wrap_self(self) -> Wrapper<Self> {
        Wrapper(self)
    }
}

// Recursive operator trait impls (via `Output` or `Rhs`) must not cause an infinite
// specialization loop in `generate_specializations_fixpoint`.
impl<T> std::ops::Neg for GenericStruct<T> {
    type Output = GenericStruct<GenericStruct<T>>;
    fn neg(self) -> Self::Output {
        GenericStruct { value: self }
    }
}

impl<T: PartialEq> PartialEq<Wrapper<Self>> for GenericStruct<T> {
    fn eq(&self, other: &Wrapper<Self>) -> bool {
        self.value == other.0.value
    }
}

impl<T: PartialOrd> PartialOrd<Wrapper<Self>> for GenericStruct<T> {
    fn partial_cmp(&self, other: &Wrapper<Self>) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(&other.0.value)
    }
}

// Mutual recursion between two distinct generic ADTs (`Ping<T> <-> Pong<T>`).
pub struct Ping<T>(pub T);
pub struct Pong<T>(pub T);

impl<T: Copy> Ping<T> {
    pub fn to_pong(&self) -> Pong<Wrapper<T>> {
        Pong(Wrapper(self.0))
    }
}

impl<T: Copy> Pong<T> {
    pub fn to_ping(&self) -> Ping<Wrapper<T>> {
        Ping(Wrapper(self.0))
    }
}

pub fn instantiate_ping() -> Ping<i32> {
    Ping(1)
}

// Layout-independent type parameter (`PhantomData<T>`) breaking cyclic dependency.
pub struct PhantomGeneric<T> {
    pub id: i32,
    pub marker: std::marker::PhantomData<T>,
}

pub struct SelfReferentialViaPhantom {
    pub phantom: PhantomGeneric<SelfReferentialViaPhantom>,
}

pub fn instantiate_self_referential_phantom() -> SelfReferentialViaPhantom {
    SelfReferentialViaPhantom {
        phantom: PhantomGeneric { id: 1, marker: std::marker::PhantomData },
    }
}

// Instantiating generics with types that do not receive bindings.
pub struct ZeroSizedUnboundType;

pub fn instantiate_with_unbound_zst() -> GenericStruct<ZeroSizedUnboundType> {
    GenericStruct { value: ZeroSizedUnboundType }
}

pub fn instantiate_with_i128() -> GenericStruct<i128> {
    GenericStruct { value: 1 }
}

pub fn instantiate_with_static_ref(x: &'static i32) -> GenericStruct<&'static i32> {
    GenericStruct { value: x }
}

pub fn instantiate_with_array() -> GenericStruct<[i32; 4]> {
    GenericStruct { value: [1, 2, 3, 4] }
}

pub struct ConstGenericStruct<T, const N: usize>(pub [T; N]);

pub fn instantiate_const_generic() -> ConstGenericStruct<i32, 4> {
    ConstGenericStruct([1, 2, 3, 4])
}

impl<T> GenericStruct<T> {
    pub fn to_unbound(&self) -> GenericStruct<ZeroSizedUnboundType> {
        GenericStruct { value: ZeroSizedUnboundType }
    }
}
