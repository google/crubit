// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! This crate is used as a test input for `cc_bindings_from_rs` with the `enum_api_v2` feature,
//! and the generated C++ bindings are then tested via `enum_api_v2_test.cc`.

/// Unit, tuple, and struct-like variants, with a direct tag.
#[derive(Clone, Copy)]
pub enum Shape {
    Point,
    Circle(i32),
    Rect { w: i32, h: i32 },
}

pub fn make_circle(r: i32) -> Shape {
    Shape::Circle(r)
}

pub fn make_rect(w: i32, h: i32) -> Shape {
    Shape::Rect { w, h }
}

pub fn area(shape: Shape) -> i32 {
    match shape {
        Shape::Point => 0,
        Shape::Circle(r) => 3 * r * r,
        Shape::Rect { w, h } => w * h,
    }
}

/// Negative discriminants.
#[derive(Clone, Copy)]
#[repr(i8)]
pub enum Sign {
    Negative = -1,
    Zero = 0,
    Positive = 1,
}

pub fn sign_of(x: i32) -> Sign {
    match x {
        ..0 => Sign::Negative,
        0 => Sign::Zero,
        _ => Sign::Positive,
    }
}

pub fn sign_value(sign: Sign) -> i32 {
    sign as i32
}

/// A single variant, so no tag at all.
#[derive(Clone, Copy)]
pub enum Wrapper {
    Only(i64),
}

pub fn unwrap(wrapper: Wrapper) -> i64 {
    let Wrapper::Only(value) = wrapper;
    value
}

/// Niche-encoded enum: `Start` and `Stop` are stored in the unused bit patterns of `bool`.
#[derive(Clone, Copy)]
pub enum Command {
    SetFlag(bool),
    Start,
    Stop,
}

pub fn make_set_flag(flag: bool) -> Command {
    Command::SetFlag(flag)
}

pub fn make_start() -> Command {
    Command::Start
}

pub fn make_stop() -> Command {
    Command::Stop
}

pub fn describe_command(cmd: Command) -> i32 {
    match cmd {
        Command::SetFlag(false) => 0,
        Command::SetFlag(true) => 1,
        Command::Start => 2,
        Command::Stop => 3,
    }
}

/// Niche at a non-zero offset (`flag` sits at offset 3 after `count` and `extra`), with another
/// variant that carries its own payload in the bytes before the niche.
#[derive(Clone, Copy)]
pub enum TaggedEntry {
    Entry { count: i16, extra: i8, flag: bool },
    Empty(i16),
    Tombstone,
}

pub fn make_entry(count: i16, extra: i8, flag: bool) -> TaggedEntry {
    TaggedEntry::Entry { count, extra, flag }
}

pub fn make_empty_entry(id: i16) -> TaggedEntry {
    TaggedEntry::Empty(id)
}

pub fn make_tombstone() -> TaggedEntry {
    TaggedEntry::Tombstone
}

pub fn describe_entry(entry: TaggedEntry) -> i32 {
    match entry {
        TaggedEntry::Entry { count, extra, flag: true } => i32::from(count) + i32::from(extra),
        TaggedEntry::Entry { count, extra, flag: false } => -(i32::from(count) + i32::from(extra)),
        TaggedEntry::Empty(id) => 1000 + i32::from(id),
        TaggedEntry::Tombstone => -999,
    }
}

/// Pointer-niched enum (`None` is stored as the null pointer).
#[derive(Clone, Copy)]
pub enum MaybeRef {
    None,
    Some(&'static i32),
}

pub static STATIC_INT: i32 = 99;

pub fn make_some_ref() -> MaybeRef {
    MaybeRef::Some(&STATIC_INT)
}

pub fn make_none_ref() -> MaybeRef {
    MaybeRef::None
}

pub fn read_maybe_ref(r: MaybeRef) -> i32 {
    match r {
        MaybeRef::None => -1,
        MaybeRef::Some(&x) => x,
    }
}
