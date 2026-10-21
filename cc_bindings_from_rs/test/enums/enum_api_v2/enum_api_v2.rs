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

/// `Clone` without `Copy` and without drop glue: copy calls into Rust (`Clone::clone`), while
/// move and destruction stay trivial.
pub struct CloneCounter {
    pub value: i32,
    clones: i32,
}

impl Clone for CloneCounter {
    fn clone(&self) -> Self {
        Self { value: self.value, clones: self.clones + 1 }
    }
}

#[derive(Clone)]
pub enum ClonedEnum {
    Empty,
    Tracked(CloneCounter),
}

pub fn make_cloned_tracked(value: i32) -> ClonedEnum {
    ClonedEnum::Tracked(CloneCounter { value, clones: 0 })
}

pub fn read_cloned_enum(e: &ClonedEnum) -> i32 {
    match e {
        ClonedEnum::Empty => -1,
        ClonedEnum::Tracked(c) => c.value * 100 + c.clones,
    }
}

/// Move-only enum with no drop glue, holding a `Clone` (non-`Copy`) inner enum.
pub enum MoveOnlyOuter {
    Empty,
    Nested(ClonedEnum),
}

pub fn consume_move_only_outer(outer: MoveOnlyOuter) -> i32 {
    match outer {
        MoveOnlyOuter::Empty => -1,
        MoveOnlyOuter::Nested(inner) => read_cloned_enum(&inner),
    }
}

use std::rc::Rc;

/// Counts the live `TrackedPayload`s it created: each holds a clone of the tracker's `Rc`, so
/// the number of live payloads is the `Rc`'s number of owners other than the tracker itself.
#[derive(Default)]
pub struct PayloadTracker {
    owners: Rc<()>,
}

impl PayloadTracker {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn track(&self, id: i32) -> TrackedPayload {
        TrackedPayload { id, _owner: Rc::clone(&self.owners) }
    }

    pub fn live_payloads(&self) -> i32 {
        (Rc::strong_count(&self.owners) - 1) as i32
    }
}

/// A payload with drop glue (from its `Rc`) but no `Drop` impl of its own.
#[derive(Clone, Default)]
pub struct TrackedPayload {
    pub id: i32,
    /// Never read: only held so that `PayloadTracker` can count live payloads.
    _owner: Rc<()>,
}

/// Enum with drop glue (from `TrackedPayload`), `Clone`, and `Default`, but no `Drop` impl of its
/// own: its destructor is `constexpr`, so `None` can be constructed in constant expressions.
#[derive(Clone, Default)]
pub enum MaybeTracked {
    #[default]
    None,
    Some(TrackedPayload),
}

pub fn make_some_tracked(tracker: &PayloadTracker, id: i32) -> MaybeTracked {
    MaybeTracked::Some(tracker.track(id))
}

pub fn consume_maybe_tracked(m: MaybeTracked) -> i32 {
    match m {
        MaybeTracked::None => -1,
        MaybeTracked::Some(ref p) => p.id,
    }
}

/// Outer enum wrapping `MaybeTracked` to test that a variant holding a non-trivially-destructible
/// type with a `constexpr` destructor (`MaybeTracked`) cannot be constructed at compile time.
pub enum NestedMaybeTracked {
    OuterNone,
    OuterSome(MaybeTracked),
}

/// Enum with an (empty) `Drop` impl of its own: Rust runs `Drop::drop` even on `Empty`, so its
/// destructor is not `constexpr`.
pub enum CustomDropEnum {
    Empty,
    Value(i32),
}

impl Drop for CustomDropEnum {
    fn drop(&mut self) {}
}

/// Methods, associated functions and constants on an `EnumApiV2` enum coexist with its variant
/// structs.
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum Instruction {
    #[default]
    TurnLeft,
    TurnRight,
    Walk(u8),
}

impl Instruction {
    pub const MAX_STEPS: u8 = 100;

    pub fn walk_clamped(steps: u8) -> Instruction {
        Instruction::Walk(steps.min(Self::MAX_STEPS))
    }

    pub fn steps(&self) -> u8 {
        match *self {
            Instruction::Walk(steps) => steps,
            Instruction::TurnLeft | Instruction::TurnRight => 0,
        }
    }

    pub fn reverse(&mut self) {
        *self = match *self {
            Instruction::TurnLeft => Instruction::TurnRight,
            Instruction::TurnRight => Instruction::TurnLeft,
            Instruction::Walk(steps) => Instruction::Walk(steps),
        };
    }
}

/// An `EnumApiV2` enum as a struct field, and in `Option`.
#[derive(Clone, Copy, Default)]
pub struct Route {
    pub first_step: Instruction,
    pub next_step: Option<Instruction>,
}

pub fn total_steps(route: Route) -> u8 {
    route.first_step.steps() + route.next_step.map_or(0, |step| step.steps())
}

/// A variant named `Discriminant` collides with the enum's `Discriminant` type alias, so this enum
/// falls back to the opaque blob of bytes and keeps the `MakeV` constructors.
#[derive(Clone, Copy)]
pub enum CollidesWithDiscriminant {
    Discriminant,
    Other(i32),
}

pub fn collides_value(c: CollidesWithDiscriminant) -> i32 {
    match c {
        CollidesWithDiscriminant::Discriminant => -1,
        CollidesWithDiscriminant::Other(value) => value,
    }
}

/// `#[non_exhaustive]` only restricts other Rust crates, so it doesn't stop `EnumApiV2`.
#[derive(Clone, Copy)]
#[non_exhaustive]
pub enum NonExhaustive {
    A,
    B(i32),
}

pub fn non_exhaustive_value(e: NonExhaustive) -> i32 {
    match e {
        NonExhaustive::A => -1,
        NonExhaustive::B(value) => value,
    }
}
