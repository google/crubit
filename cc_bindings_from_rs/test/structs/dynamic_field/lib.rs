// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use cc_struct::{DynamicMovable, DynamicNoDtor, DynamicWithVirtualDtor};

pub struct StructWithDynamicField {
    pub dynamic: DynamicWithVirtualDtor,
}

pub fn inspect_dynamic(s: &StructWithDynamicField) -> i32 {
    s.dynamic.value()
}

pub struct DynamicNoDtorWrapper {
    pub dynamic: DynamicNoDtor,
}

pub fn inspect_no_dtor(s: &DynamicNoDtorWrapper) -> i32 {
    s.dynamic.value()
}

#[repr(transparent)]
pub struct TransparentDynamicField(DynamicWithVirtualDtor);

pub fn inspect_transparent(s: &TransparentDynamicField) -> i32 {
    s.0.value()
}

pub fn inspect_tuple(t: &(DynamicWithVirtualDtor, i32)) -> i32 {
    t.0.value() + t.1
}

pub fn inspect_dynamic_movable_tuple(t: &(DynamicMovable, i32)) -> i32 {
    t.0.value() + t.1
}

// Not bindable by value (`::rs::Tuple<DynamicMovable, int32_t>` has deleted move
// operations and no `UnsafeRelocateTag`); must be cleanly omitted rather than
// emitting uncompilable C++ calls.
pub fn consume_dynamic_movable_tuple(_t: (DynamicMovable, i32)) {}

pub enum EnumWithDynamicTuple {
    Tuple((DynamicMovable, i32)),
    Int(i32),
}

pub enum EnumWithDynamicVariant {
    Dynamic(DynamicMovable),
    Int(i32),
}

// Not bindable by value (non-movable, non-bitwise-relocatable); must not emit a
// UB `memcpy` relocating constructor just to compile.
pub fn make_int_variant(x: i32) -> EnumWithDynamicVariant {
    EnumWithDynamicVariant::Int(x)
}

pub fn make_boxed_int_variant(x: i32) -> Box<EnumWithDynamicVariant> {
    Box::new(EnumWithDynamicVariant::Int(x))
}

pub struct BoxedDynamic {
    pub boxed: std::pin::Pin<Box<DynamicWithVirtualDtor>>,
}

pub fn inspect_boxed_dynamic(b: BoxedDynamic) -> i32 {
    b.boxed.value()
}
