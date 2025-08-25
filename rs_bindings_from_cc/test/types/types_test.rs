// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#![allow(unreachable_code)] // compilation-only test.

use core::cell::Cell;
use core::ffi::c_void;
use googletest::prelude::*;

trait ParameterIs<P> {}

impl<P, R> ParameterIs<P> for unsafe fn(P) -> R {}

fn test_parameter_is<P, F: ParameterIs<P>>(_: F) {}

macro_rules! struct_field_type_is {
  ($mod:ident, $($cc_name:ident => $rs_type:ty),* $(,)?) => {
    $(
    const _ : () = {
      fn test_struct_field_type(struct_value: $mod::$cc_name) {
        let _c = Cell::new(struct_value.field);
        let _: Cell<$rs_type> = _c;
      }
    };
  )*
  }
}
macro_rules! function_return_type_is {
  ($mod:ident, $($cc_name:ident => $rs_type:ty),* $(,)?) => {
    $(
    const _ : () = {
      unsafe fn test_return_type() {
        let _c = Cell::new($mod::$cc_name::Function(unreachable!()));
        let _: Cell<$rs_type> = _c;
      }
    };
  )*
  }
}
macro_rules! function_parameter_type_is {
  ($mod:ident, $($cc_name:ident => $rs_type:ty),* $(,)?) => {
    $(
    const _ : () = {
      unsafe fn test_param_type() {
        let f: unsafe fn(_) -> _ = $mod::$cc_name::Function;
        test_parameter_is::<$rs_type, _>(f);
      }
    };
  )*
  }
}

macro_rules! type_is {
  (
    $mod:ident,
    $(
      $cc_name:ident => $rs_type:ty
    ),* $(,)?
  ) => {
    $(
      struct_field_type_is!($mod, $cc_name => $rs_type);
      function_return_type_is!($mod, $cc_name => $rs_type);
      function_parameter_type_is!($mod, $cc_name => $rs_type);
    )*
  }
}

type_is!(
    types_nonptr,
    Bool => bool,
    Char => ::core::ffi::c_char,

    UnsignedChar => u8,
    SignedChar => i8,
    Char16 => u16,
    Char32 => u32,

    Short => i16,
    Int => i32,
    Long => i64,
    LongLong => i64,

    UnsignedShort => u16,
    UnsignedInt => u32,
    UnsignedLong => u64,
    UnsignedLongLong => u64,

    SignedShort => i16,
    SignedInt => i32,
    SignedLong => i64,
    SignedLongLong => i64,

    Int8 => i8,
    Int16 => i16,
    Int32 => i32,
    Int64 => i64,
    Uint8 => u8,
    Uint16 => u16,
    Uint32 => u32,
    Uint64 => u64,

    PtrDiff => isize,
    Size => usize,
    IntPtr => isize,
    UintPtr => usize,

    StdInt8 => i8,
    StdInt16 => i16,
    StdInt32 => i32,
    StdInt64 => i64,
    StdUint8 => u8,
    StdUint16 => u16,
    StdUint32 => u32,
    StdUint64 => u64,

    StdPtrDiff => isize,
    StdSize => usize,
    StdIntPtr => isize,
    StdUintPtr => usize,

    Float => f32,
    Double => f64,

    Struct => types_nonptr::ns::ExampleStruct,
    TypeAlias => types_nonptr::Alias,
    Using => types_nonptr::ns::ExampleStruct,

    Enum => types_nonptr::ns::ExampleEnum,
    TypeAliasEnum => types_nonptr::AliasEnum,
    UsingEnum => types_nonptr::ns::ExampleEnum,

    ExistingRustTypeStruct => i8,
    ExistingRustTypeClass => i8,
    ExistingRustTypeEnum => i8,
    ExistingRustTypeAlias => i8,

    ExistingRustTypeSliceRefConstUint8 => *const [u8],
    ExistingRustTypeSliceRefUint8 => *mut [u8],
    ExistingRustTypeSliceRefConstUint16 => *const [u16],
    ExistingRustTypeSliceRefUint16 => *mut [u16],
    ExistingRustTypeSliceRefConstUint32 => *const [u32],
    ExistingRustTypeSliceRefUint32 => *mut [u32],
    ExistingRustTypeSliceRefConstUint64 => *const [u64],
    ExistingRustTypeSliceRefUint64 => *mut [u64],

    ExistingRustTypeSliceRefConstInt8 => *const [i8],
    ExistingRustTypeSliceRefInt8 => *mut [i8],
    ExistingRustTypeSliceRefConstInt16 => *const [i16],
    ExistingRustTypeSliceRefInt16 => *mut [i16],
    ExistingRustTypeSliceRefConstInt32 => *const [i32],
    ExistingRustTypeSliceRefInt32 => *mut [i32],
    ExistingRustTypeSliceRefConstInt64 => *const [i64],
    ExistingRustTypeSliceRefInt64 => *mut [i64],

    ExistingRustTypeSliceRefConstFloat => *const [f32],
    ExistingRustTypeSliceRefFloat => *mut [f32],
    ExistingRustTypeSliceRefConstDouble => *const [f64],
    ExistingRustTypeSliceRefDouble => *mut [f64],

    ExistingRustTypeSliceRefArbitraryStruct => *mut [types_nonptr::ns::ExampleStruct],
    ExistingRustTypeSliceRefArbitraryEnum => *const [types_nonptr::ns::ExampleEnum],
    ExistingRustTypeSliceRefArbitraryAliasEnum => *mut [types_nonptr::AliasEnum],
);

// TODO(b/228569417): These should all generate bindings and be & (mut) 'static.
// type_is!(
//     types_lifetimes,
//     IntP => &'static mut i32,
//     ConstIntP => &'static i32,
//     IntRef => &'static mut i32,
//     ConstIntRef => &'static i32,
//     VoidP => &'static mut c_void,
//     ConstVoidP => &'static c_void,
//     VoidPP => &'static mut &'static mut c_void,
//     StructPtr => &'static mut types_lifetimes::ExampleStruct,
//     ConstStructPtr => &'static types_lifetimes::ExampleStruct,
//     StructRef => &'static mut types_lifetimes::ExampleStruct,
//     ConstStructRef => &'static types_lifetimes::ExampleStruct,
// );

type_is!(
    types_nolifetimes,

    IntP => *mut i32,
    ConstIntP => *const i32,
    IntRef => *mut i32,
    ConstIntRef => *const i32,
    VoidP => *mut c_void,
    ConstVoidP => *const c_void,
    VoidPP => *mut *mut c_void,

    StructPtr => *mut types_nolifetimes::ExampleStruct,
    ConstStructPtr => *const types_nolifetimes::ExampleStruct,
    StructRef => *mut types_nolifetimes::ExampleStruct,
    ConstStructRef => *const types_nolifetimes::ExampleStruct,

    FuncRef => extern "C" fn(),
    FuncPtr => Option<extern "C" fn()>,
    UnsafeFuncRef => unsafe extern "C" fn(*mut c_void),
    UnsafeFuncPtr => Option<unsafe extern "C" fn(*mut c_void)>,
);

// inferred lifetimes depend on location.

struct_field_type_is!(
    types_inferred_lifetimes,
    IntP => *mut i32,
    ConstIntP => *const i32,
    IntRef => *mut i32,
    ConstIntRef => *const i32,
    VoidP => *mut c_void,
    ConstVoidP => *const c_void,
    VoidPP => *mut *mut c_void,

    StructPtr => *mut types_inferred_lifetimes::ExampleStruct,
    ConstStructPtr => *const types_inferred_lifetimes::ExampleStruct,
    StructRef => *mut types_inferred_lifetimes::ExampleStruct,
    ConstStructRef => *const types_inferred_lifetimes::ExampleStruct,
);

function_return_type_is!(types_inferred_lifetimes,
    IntP => Option<&mut i32>,
    ConstIntP => Option<&i32>,
    IntRef => &mut i32,
    ConstIntRef => &i32,
    VoidP => Option<&mut c_void>,
    ConstVoidP => Option<&c_void>,
    // TODO: b/436971180 - Why is this a pointer?
    VoidPP => *mut *mut c_void,

    StructPtr => Option<&mut types_inferred_lifetimes::ExampleStruct>,
    ConstStructPtr => Option<&types_inferred_lifetimes::ExampleStruct>,
    StructRef => &mut types_inferred_lifetimes::ExampleStruct,
    ConstStructRef => &types_inferred_lifetimes::ExampleStruct,
);

function_parameter_type_is!(types_inferred_lifetimes,
    IntP => Option<&mut i32>,
    ConstIntP => Option<&i32>,
    IntRef => &mut i32,
    ConstIntRef => &i32,
    VoidP => Option<&mut c_void>,
    ConstVoidP => Option<&c_void>,
    // TODO: b/436971180 - Why is this a pointer?
    VoidPP => *mut *mut c_void,

    StructPtr => Option<&mut types_inferred_lifetimes::ExampleStruct>,
    ConstStructPtr => Option<&types_inferred_lifetimes::ExampleStruct>,
    StructRef => &mut types_inferred_lifetimes::ExampleStruct,
    ConstStructRef => &types_inferred_lifetimes::ExampleStruct,
);

#[gtest]
fn test_typemap_suppresses_bindings() {
    assert!(!item_exists::type_exists!(types_nonptr::MyI8Class));
    assert!(!item_exists::type_exists!(types_nonptr::MyI8Struct));
    assert!(!item_exists::type_exists!(types_nonptr::MyI8Enum));
    assert!(!item_exists::type_exists!(types_nonptr::MyI8Alias));
}
