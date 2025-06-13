// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

extern crate rustc_span;

use crate::{AdtCoreBindings, SugaredTy};
use rustc_span::Symbol;
use std::fmt::Debug;
use std::rc::Rc;

/// A representation of a reference to a C++ type created from a Rust type.
///
/// Note that:
/// - not all C++ types can be represented with a `CppType`,
/// - a single Rust type can be mapped to multiple C++ types depending on the context.
/// - `CppType` is not unique: multiple `CcType`s can refer to the same C++ type.
#[derive(Clone)]
pub struct CppType<'tcx> {
    /// The kind of C++ type this is.
    pub kind: CppTypeKind<'tcx>,
    /// The Rust type that this C++ type was created from.
    pub rs_type_origin: SugaredTy<'tcx>,
}

#[derive(Clone)]
pub enum CppTypeKind<'tcx> {
    /// A C++ type generated from a `struct`, `enum`, or `union` defined in Rust.
    RsAdt(Rc<AdtCoreBindings<'tcx>>),

    /// A C++ pointer or reference type.
    Pointer { element_type: Rc<CppType<'tcx>>, is_const: bool, is_reference: bool },

    /// An ABI-compatible built-in type.
    Primitive(CppPrimitiveKind),

    /// C++ binding for `&[T]`/`&mut [T]` as defined by `slice_ref.h`.
    /// Note that `&[T]` is a `SliceRef<const T>`, not a `SliceRef<const T>&`.
    SliceRef { element_type: Rc<CppType<'tcx>>, is_const: bool },

    /// C++ binding for `char` as defined by `rs_char.h`.
    RsChar,

    /// A user-defined C++ type.
    UserDefined {
        /// The raw text of the C++ type the user wrote in `cpp_type="..."`,
        /// e.g. `path::to::MyCppType<u32>` or `std::string&`.
        text: Symbol,

        /// Whether the type is a pointer.
        ///
        /// Currently, this is inferred textually by looking for `&` in or `*` in the
        /// `cpp_type="..."` text. TODO(bug): users should be asked to specify this explicitly,
        /// or at least allowed to override the textual inference.
        is_pointer: bool,
    },

    /// C++'s `void`.
    ///
    /// Note that C++'s `void` is strange in that it is not a valid type for a variable,
    /// nor is it valid as an array element type, nor as the target of a reference.
    /// Functions that return `void` cannot have their result bound to a void-typed local.
    /// See https://en.cppreference.com/w/cpp/language/types#void
    Void,
}

/// A type built into both Rust and C++.
#[derive(PartialEq, Eq, Clone, Copy, Hash)]
pub enum CppPrimitiveKind {
    // C++ primitives from cstdint.
    Int8,
    Int16,
    Int32,
    Int64,
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    IntPtr,
    UIntPtr,

    // C++ primitives with corresponding {core, std}::ffi::c_* aliases.
    //
    // TODO(b/283258442): Also handle `libc` aliases.
    Char,
    SignedChar,
    UnsignedChar,
    Short,
    UnsignedShort,
    Int,
    UnsignedInt,
    Long,
    UnsignedLong,
    LongLong,
    UnsignedLongLong,

    // Other shared primitives.
    Float,  // aka f32
    Double, // aka f64
    Bool,
}

impl CppPrimitiveKind {
    pub fn requires_stdint(self) -> bool {
        use CppPrimitiveKind::*;
        matches!(
            self,
            Int8 | Int16 | Int32 | Int64 | UInt8 | UInt16 | UInt32 | UInt64 | IntPtr | UIntPtr
        )
    }
    pub fn to_str(self) -> &'static str {
        use CppPrimitiveKind::*;
        match self {
            Int8 => "std::int8_t",
            Int16 => "std::int16_t",
            Int32 => "std::int32_t",
            Int64 => "std::int64_t",
            UInt8 => "std::uint8_t",
            UInt16 => "std::uint16_t",
            UInt32 => "std::uint32_t",
            UInt64 => "std::uint64_t",
            IntPtr => "std::intptr_t",
            UIntPtr => "std::uintptr_t",
            Char => "char",
            SignedChar => "signed_char",
            UnsignedChar => "unsigned char",
            Short => "short",
            UnsignedShort => "unsigned short",
            Int => "int",
            UnsignedInt => "unsigned int",
            Long => "long",
            UnsignedLong => "unsigned long",
            LongLong => "long long",
            UnsignedLongLong => "unsigned long long",
            Float => "float",
            Double => "double",
            Bool => "bool",
        }
    }
}

impl Debug for CppPrimitiveKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.pad(self.to_str())
    }
}
