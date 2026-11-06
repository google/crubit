// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#![feature(negative_impls)]
#![deny(unsafe_op_in_unsafe_fn)]

//! Layout-compatible Rust wrapper for inline C++ Protobuf messages.
//!
//! When using the C++ kernel, Rust Protobuf message types (`M: protobuf::Message`)
//! are internally a single pointer to a heap-allocated C++ Protobuf message (`*mut CppProto`),
//! whereas C++ Protobuf message types have an inline layout (`sizeof(CppProto)` bytes).
//!
//! When `rs_bindings_from_cc` binds a C++ type that embeds an inline C++ Protobuf message
//! (such as `absl::StatusOr<MyProto>`, `std::vector<MyProto>`, `std::optional<MyProto>`,
//! or a struct field), it generates a zero-sized marker type under
//! `<cc_library>::proto::<proto_target>::<MyProto>` implementing [`CppProto`] and represents
//! the inline C++ Protobuf message in Rust as
//! `::proto_cpp::Cpp<<cc_library>::proto::<proto_target>::<MyProto>>`.
//!
//! This mirrors `::proto::Rust<CppProto>` in `support/protobuf/rust.h`,
//! which wraps a Rust heap-allocated Protobuf message in a layout-compatible C++ type for
//! `cc_bindings_from_rs`.

use core::convert::Infallible;
use core::ffi::c_void;
use core::fmt;
use core::marker::PhantomData;
use core::pin::Pin;
use ctor::{Assign, ConstRvalueReference, Ctor, CtorNew, RvalueReference, SelfCtor};
use protobuf::{CopyFrom, Message, MessageMutInterop, MessageViewInterop, Mut, TakeFrom, View};

unsafe extern "C" {
    fn crubit_protobuf_placement_new(
        default_instance: *const c_void,
        dest: *mut c_void,
        expected_size: usize,
        expected_align: usize,
    );
    fn crubit_protobuf_destroy(msg: *mut c_void);
}

#[doc(hidden)]
pub mod internal {
    use core::mem::MaybeUninit;

    #[repr(align(1))]
    #[derive(Copy, Clone)]
    pub struct Align1;
    #[repr(align(2))]
    #[derive(Copy, Clone)]
    pub struct Align2;
    #[repr(align(4))]
    #[derive(Copy, Clone)]
    pub struct Align4;
    #[repr(align(8))]
    #[derive(Copy, Clone)]
    pub struct Align8;
    #[repr(align(16))]
    #[derive(Copy, Clone)]
    pub struct Align16;
    #[repr(align(32))]
    #[derive(Copy, Clone)]
    pub struct Align32;
    #[repr(align(64))]
    #[derive(Copy, Clone)]
    pub struct Align64;

    pub trait Alignment<const ALIGN: usize> {
        type Marker: Copy + Send + Sync + 'static;
    }

    impl Alignment<1> for () {
        type Marker = Align1;
    }
    impl Alignment<2> for () {
        type Marker = Align2;
    }
    impl Alignment<4> for () {
        type Marker = Align4;
    }
    impl Alignment<8> for () {
        type Marker = Align8;
    }
    impl Alignment<16> for () {
        type Marker = Align16;
    }
    impl Alignment<32> for () {
        type Marker = Align32;
    }
    impl Alignment<64> for () {
        type Marker = Align64;
    }

    #[repr(C)]
    pub struct RawStorage<const SIZE: usize, const ALIGN: usize = 8>
    where
        (): Alignment<ALIGN>,
    {
        _align: [<() as Alignment<ALIGN>>::Marker; 0],
        _data: [MaybeUninit<u8>; SIZE],
    }
}

/// Marker trait implemented by Crubit-generated ZST marker types (under
/// `<cc_library>::proto::<proto_target>::<Message>`) to associate a Rust Protobuf
/// message type with the size and alignment of its inline C++ representation.
///
/// # Safety
///
/// `Self::Storage` must have the exact size and alignment of the C++ Protobuf
/// message corresponding to `Self::Message`.
pub unsafe trait CppProto: 'static {
    /// The Rust Protobuf message type (e.g. `::my_rust_proto::MyMessage`).
    type Message: Message;

    /// Opaque storage with the exact size and alignment of the inline C++ Protobuf message.
    #[doc(hidden)]
    type Storage;
}

/// Returns a pointer to the static C++ default instance for `M` without allocating.
#[inline]
fn default_instance_ptr<M: Message>() -> *const c_void {
    let default_view = <View<'static, M> as Default>::default();
    MessageViewInterop::__unstable_as_raw_message(&default_view)
}

/// Placement-news a default C++ protobuf message of type `T::Message` into `dest`.
///
/// # Safety
///
/// `dest` must be non-null, aligned to `align_of::<Cpp<T>>()`, and valid for writes of
/// `size_of::<Cpp<T>>()` bytes.
#[inline]
unsafe fn placement_new_default<T: CppProto>(dest: *mut c_void) {
    // SAFETY: `default_instance_ptr::<T::Message>()` returns a valid pointer to the C++
    // default instance for `T::Message`, and the caller guarantees `dest` is valid for writes
    // of `size_of::<Cpp<T>>()` bytes at `align_of::<Cpp<T>>()` alignment.
    unsafe {
        crubit_protobuf_placement_new(
            default_instance_ptr::<T::Message>(),
            dest,
            core::mem::size_of::<Cpp<T>>(),
            core::mem::align_of::<Cpp<T>>(),
        );
    }
}

/// A layout-compatible Rust representation of an inline C++ Protobuf message described by
/// the marker type `T: CppProto` (with Rust message type `T::Message`).
///
/// Because C++ Protobuf messages are not trivially relocatable (e.g. due to inline strings or
/// repeated field storage), `Cpp<T>` is `!Unpin` and must be constructed and mutated in-place
/// using [`ctor::Ctor`] (e.g. `ctor::emplace!`).
#[doc = "CRUBIT_ANNOTATE: cpp_type={T}"]
#[doc = "CRUBIT_ANNOTATE: cpp_move_constructible="]
#[repr(transparent)]
pub struct Cpp<T: CppProto> {
    _storage: T::Storage,
    _marker: PhantomData<fn() -> T>,
}

impl<T: CppProto> !Unpin for Cpp<T> {}

impl<T: CppProto> Drop for Cpp<T> {
    #[inline]
    fn drop(&mut self) {
        // SAFETY: `self` contains an initialized inline C++ protobuf message of type
        // `T::Message` with `GetArena() == nullptr`:
        // - Messages constructed in Rust via `placement_new_default` pass `arena = nullptr`.
        // - Messages constructed by value in C++ (e.g. inside `absl::StatusOr<Msg>`,
        //   `std::vector<Msg>`, or a struct field) always use the public default/copy/move
        //   constructors, which set `GetArena() == nullptr` (the `Message(Arena*)` constructor is
        //   `protected` in C++ Protobuf and only used by `Arena::Create<Msg>(arena)`, which returns
        //   a `Msg*` pointing into arena memory and never an inline `Msg` value).
        // - C++ Protobuf `Swap` and move assignment never transfer arena ownership to a non-arena
        //   (`GetArena() == nullptr`) message (cross-arena swaps copy instead).
        // `crubit_protobuf_destroy` additionally `ABSL_CHECK`s `msg->GetArena() == nullptr`.
        unsafe {
            crubit_protobuf_destroy((&raw mut self._storage).cast::<c_void>());
        }
    }
}

impl<T: CppProto> Cpp<T> {
    /// Returns a `Ctor` that constructs an inline C++ Protobuf message in-place by moving
    /// the contents of an owned Rust message `src` in O(1) time.
    #[inline]
    pub fn from_rust(src: T::Message) -> FromRustCtor<T> {
        FromRustCtor { src, _marker: PhantomData }
    }

    /// Returns a shared [`View`] of this inline C++ Protobuf message.
    #[inline]
    pub fn as_view(&self) -> View<'_, T::Message> {
        // SAFETY: `self.raw_message()` points to a valid initialized C++ protobuf message of
        // type `T::Message` that remains valid and unmodified for the lifetime of `&self`.
        unsafe {
            MessageViewInterop::__unstable_wrap_raw_message_unchecked_lifetime(self.raw_message())
        }
    }

    /// Returns a mutable [`Mut`] proxy to this inline C++ Protobuf message.
    ///
    /// Named `as_proto_mut` rather than `as_mut` to avoid shadowing [`Pin::as_mut`] on
    /// `Pin<&mut Cpp<T>>`.
    #[inline]
    pub fn as_proto_mut(self: Pin<&mut Self>) -> Mut<'_, T::Message> {
        // SAFETY: `self.raw_message_mut()` points to a valid initialized C++ protobuf message of
        // type `T::Message` that is exclusively borrowed for the lifetime of `self`.
        unsafe {
            MessageMutInterop::__unstable_wrap_raw_message_mut_unchecked_lifetime(
                self.raw_message_mut(),
            )
        }
    }

    /// Moves the contents of this inline C++ Protobuf message into a new owned Rust message
    /// `T::Message` in O(1) time (via C++ `Swap`), leaving `self` cleared.
    #[doc(alias = "into_rust")]
    #[inline]
    pub fn take(self: Pin<&mut Self>) -> T::Message {
        let mut msg = T::Message::default();
        TakeFrom::take_from(&mut msg, self.as_proto_mut());
        msg
    }

    /// Returns a raw const pointer to the underlying C++ Protobuf message.
    #[inline]
    fn raw_message(&self) -> *const c_void {
        (&raw const self._storage).cast::<c_void>()
    }

    /// Returns a raw mutable pointer to the underlying C++ Protobuf message.
    #[inline]
    fn raw_message_mut(self: Pin<&mut Self>) -> *mut c_void {
        // SAFETY: We do not move out of the pinned reference; the caller receives a raw pointer.
        unsafe { (&raw mut self.get_unchecked_mut()._storage).cast::<c_void>() }
    }
}

impl<T: CppProto> fmt::Debug for Cpp<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.as_view(), f)
    }
}

// ============================================================================
// In-place constructors (`ctor::Ctor` and `ctor::CtorNew`)
// ============================================================================

/// `Ctor` that default-constructs a [`Cpp<T>`] in-place.
#[must_use]
pub struct DefaultCtor<T: CppProto>(PhantomData<fn() -> Cpp<T>>);

impl<T: CppProto> !SelfCtor for DefaultCtor<T> {}

// SAFETY: `ctor` initializes `dest` in-place via `placement_new_default`.
unsafe impl<T: CppProto> Ctor for DefaultCtor<T> {
    type Output = Cpp<T>;
    type Error = Infallible;

    #[inline]
    unsafe fn ctor(self, dest: *mut Self::Output) -> Result<(), Infallible> {
        // SAFETY: `dest` is valid for writes and properly aligned.
        unsafe {
            placement_new_default::<T>(dest.cast::<c_void>());
        }
        Ok(())
    }
}

/// `Ctor` that constructs a [`Cpp<T>`] in-place by copying from a [`View`].
#[must_use]
pub struct CopyViewCtor<'a, T: CppProto>
where
    T::Message: 'a,
{
    src: View<'a, T::Message>,
    _marker: PhantomData<fn() -> Cpp<T>>,
}

impl<'a, T: CppProto> !SelfCtor for CopyViewCtor<'a, T> where T::Message: 'a {}

// SAFETY: `ctor` default-constructs `dest` in-place and then copies `self.src` into `*dest`.
unsafe impl<'a, T: CppProto> Ctor for CopyViewCtor<'a, T>
where
    T::Message: 'a,
{
    type Output = Cpp<T>;
    type Error = Infallible;

    #[inline]
    unsafe fn ctor(self, dest: *mut Self::Output) -> Result<(), Infallible> {
        // SAFETY: `dest` is valid for writes and properly aligned.
        unsafe {
            placement_new_default::<T>(dest.cast::<c_void>());
            CopyFrom::copy_from(&mut Pin::new_unchecked(&mut *dest).as_proto_mut(), self.src);
        }
        Ok(())
    }
}

/// `Ctor` that constructs a [`Cpp<T>`] in-place by taking ownership of `T::Message` in O(1).
#[must_use]
pub struct FromRustCtor<T: CppProto> {
    src: T::Message,
    _marker: PhantomData<fn() -> Cpp<T>>,
}

impl<T: CppProto> !SelfCtor for FromRustCtor<T> {}

// SAFETY: `ctor` default-constructs `dest` in-place and then swaps `self.src` into `*dest`.
unsafe impl<T: CppProto> Ctor for FromRustCtor<T> {
    type Output = Cpp<T>;
    type Error = Infallible;

    #[inline]
    unsafe fn ctor(mut self, dest: *mut Self::Output) -> Result<(), Infallible> {
        // SAFETY: `dest` is valid for writes and properly aligned.
        unsafe {
            placement_new_default::<T>(dest.cast::<c_void>());
            TakeFrom::take_from(&mut Pin::new_unchecked(&mut *dest).as_proto_mut(), &mut self.src);
        }
        Ok(())
    }
}

/// `Ctor` that move-constructs a [`Cpp<T>`] from an [`RvalueReference`] to a compatible [`Cpp<U>`].
#[must_use]
pub struct MoveCppCtor<'a, T: CppProto, U: CppProto<Message = T::Message> = T>(
    RvalueReference<'a, Cpp<U>>,
    PhantomData<fn() -> Cpp<T>>,
);

impl<'a, T: CppProto, U: CppProto<Message = T::Message>> !SelfCtor for MoveCppCtor<'a, T, U> {}

// SAFETY: `ctor` default-constructs `dest` in-place and then swaps from `self.0` into `*dest`.
unsafe impl<'a, T: CppProto, U: CppProto<Message = T::Message>> Ctor for MoveCppCtor<'a, T, U> {
    type Output = Cpp<T>;
    type Error = Infallible;

    #[inline]
    unsafe fn ctor(mut self, dest: *mut Self::Output) -> Result<(), Infallible> {
        // SAFETY: `dest` is valid for writes and properly aligned.
        unsafe {
            placement_new_default::<T>(dest.cast::<c_void>());
            TakeFrom::take_from(
                &mut Pin::new_unchecked(&mut *dest).as_proto_mut(),
                self.0.as_mut().as_proto_mut(),
            );
        }
        Ok(())
    }
}

impl<T: CppProto> CtorNew<()> for Cpp<T> {
    type CtorType = DefaultCtor<T>;
    type Error = Infallible;

    #[inline]
    fn ctor_new(_: ()) -> Self::CtorType {
        DefaultCtor(PhantomData)
    }
}

impl<'a, T: CppProto, U: CppProto<Message = T::Message>> CtorNew<&'a Cpp<U>> for Cpp<T> {
    type CtorType = CopyViewCtor<'a, T>;
    type Error = Infallible;

    #[inline]
    fn ctor_new(src: &'a Cpp<U>) -> Self::CtorType {
        CopyViewCtor { src: src.as_view(), _marker: PhantomData }
    }
}

impl<'a, T: CppProto, U: CppProto<Message = T::Message>> CtorNew<ConstRvalueReference<'a, Cpp<U>>>
    for Cpp<T>
{
    type CtorType = CopyViewCtor<'a, T>;
    type Error = Infallible;

    #[inline]
    fn ctor_new(src: ConstRvalueReference<'a, Cpp<U>>) -> Self::CtorType {
        CopyViewCtor { src: src.0.as_view(), _marker: PhantomData }
    }
}

impl<'a, T: CppProto, U: CppProto<Message = T::Message>> CtorNew<RvalueReference<'a, Cpp<U>>>
    for Cpp<T>
{
    type CtorType = MoveCppCtor<'a, T, U>;
    type Error = Infallible;

    #[inline]
    fn ctor_new(src: RvalueReference<'a, Cpp<U>>) -> Self::CtorType {
        MoveCppCtor(src, PhantomData)
    }
}

// ============================================================================
// Assignment (`ctor::Assign`)
// ============================================================================

impl<'a, T: CppProto, U: CppProto<Message = T::Message>> Assign<&'a Cpp<U>> for Cpp<T> {
    #[inline]
    fn assign(self: Pin<&mut Self>, src: &'a Cpp<U>) {
        CopyFrom::copy_from(&mut self.as_proto_mut(), src.as_view());
    }
}

impl<'a, T: CppProto, U: CppProto<Message = T::Message>> Assign<ConstRvalueReference<'a, Cpp<U>>>
    for Cpp<T>
{
    #[inline]
    fn assign(self: Pin<&mut Self>, src: ConstRvalueReference<'a, Cpp<U>>) {
        CopyFrom::copy_from(&mut self.as_proto_mut(), src.0.as_view());
    }
}

impl<'a, T: CppProto, U: CppProto<Message = T::Message>> Assign<RvalueReference<'a, Cpp<U>>>
    for Cpp<T>
{
    #[inline]
    fn assign(self: Pin<&mut Self>, mut src: RvalueReference<'a, Cpp<U>>) {
        TakeFrom::take_from(&mut self.as_proto_mut(), src.as_mut().as_proto_mut());
    }
}
