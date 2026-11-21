// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct LeafRsType {
    field: u8,
}

pub fn wrap(x: u8) -> LeafRsType {
    LeafRsType { field: x }
}

pub fn unwrap(x: LeafRsType) -> u8 {
    x.field
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum LeafRsEnum {
    KUnknown,
    KValue1,
    KValue2,
    KValue3,
}

pub fn wrap_enum(x: u8) -> LeafRsEnum {
    match x {
        1 => LeafRsEnum::KValue1,
        2 => LeafRsEnum::KValue2,
        3 => LeafRsEnum::KValue3,
        _ => LeafRsEnum::KUnknown,
    }
}

pub fn unwrap_enum(x: LeafRsEnum) -> u8 {
    x as u8
}

pub use LeafRsType as LeafRsTypeAlias;

#[derive(Copy, Clone)]
pub struct LeafRsLifetime<'a> {
    field: u8,
    _marker: core::marker::PhantomData<&'a u8>,
}

pub fn wrap_lifetime<'a>(x: &'a u8) -> LeafRsLifetime<'a> {
    LeafRsLifetime { field: *x, _marker: core::marker::PhantomData }
}

pub fn unwrap_lifetime(x: LeafRsLifetime<'_>) -> u8 {
    x.field
}

pub fn static_lifetime() -> LeafRsLifetime<'static> {
    LeafRsLifetime { field: 42, _marker: core::marker::PhantomData }
}

pub use LeafRsLifetime as LeafRsLifetimeAlias;
pub type StaticLeafRsLifetime = LeafRsLifetime<'static>;

/// Several lifetime parameters: `'b` must outlive `'a`, and `'c` is unrelated to both.
#[derive(Copy, Clone)]
pub struct LeafRsBounded<'a, 'b: 'a, 'c> {
    field: u8,
    _marker: core::marker::PhantomData<(&'a u8, &'b u8, &'c u8)>,
}

pub fn wrap_bounded<'a, 'c>(x: &'a u8, _z: &'c u8) -> LeafRsBounded<'a, 'a, 'c> {
    LeafRsBounded { field: *x, _marker: core::marker::PhantomData }
}

pub fn unwrap_bounded(x: LeafRsBounded<'_, '_, '_>) -> u8 {
    x.field
}

/// A lifetime parameter that must be `'static`.
#[derive(Copy, Clone)]
pub struct LeafRsStaticBound<'a: 'static> {
    field: u8,
    _marker: core::marker::PhantomData<&'a u8>,
}

pub fn wrap_static_bound(x: &'static u8) -> LeafRsStaticBound<'static> {
    LeafRsStaticBound { field: *x, _marker: core::marker::PhantomData }
}

pub fn unwrap_static_bound(x: LeafRsStaticBound<'_>) -> u8 {
    x.field
}

/// Borrows a `str` (only its first byte is kept, so the layout does not depend
/// on the pointer size).
#[derive(Copy, Clone)]
pub struct LeafRsStr<'a> {
    first: u8,
    _marker: core::marker::PhantomData<&'a str>,
}

pub fn wrap_str<'a>(s: &'a str) -> LeafRsStr<'a> {
    LeafRsStr { first: s.as_bytes()[0], _marker: core::marker::PhantomData }
}

pub fn first_byte(x: LeafRsStr<'_>) -> u8 {
    x.first
}
