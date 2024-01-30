// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#![allow(unreachable_code)] // compilation-only test.

use core::ffi::c_void;

macro_rules! struct_field_type_is {
  ($mod:ident, $($cc_name:ident => $rs_type:ty),* $(,)?) => {
    $(
    const _ : () = {
      fn test_struct_field_type(struct_value: $mod::$cc_name) {
        let _ : $rs_type = struct_value.field;
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
        let _ : $rs_type = $mod::$cc_name::Function(unreachable!());
      }
    };
  )*
  }
}
macro_rules! function_parameter_type_is {
  ($mod:ident, $($cc_name:ident => $rs_type:ty),* $(,)?) => {
    $(
    const _ : () = {
      unsafe fn test_param_type(rs_value: $rs_type) {
        $mod::$cc_name::Function(rs_value);
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
    Char => u8,

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

    TypeMapOverrideStruct => i8,
    TypeMapOverrideClass => i8,
    TypeMapOverrideEnum => i8,
    TypeMapOverrideAlias => i8,
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

// TODO(jeanpierreda): Why are these pointers?
function_return_type_is!(types_inferred_lifetimes,
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

function_parameter_type_is!(types_inferred_lifetimes,
    IntP => &mut i32,
    ConstIntP => &i32,
    IntRef => &mut i32,
    ConstIntRef => &i32,
    VoidP => &mut c_void,
    ConstVoidP => &c_void,
    // TODO(jeanpierreda): Why is this a pointer?
    VoidPP => *mut *mut c_void,

    StructPtr => &mut types_inferred_lifetimes::ExampleStruct,
    ConstStructPtr => &types_inferred_lifetimes::ExampleStruct,
    StructRef => &mut types_inferred_lifetimes::ExampleStruct,
    ConstStructRef => &types_inferred_lifetimes::ExampleStruct,
);

#[test]
fn test_typemap_suppresses_bindings() {
    assert!(!item_exists::type_exists!(types_nonptr::MyI8Class));
    assert!(!item_exists::type_exists!(types_nonptr::MyI8Struct));
    assert!(!item_exists::type_exists!(types_nonptr::MyI8Enum));
    assert!(!item_exists::type_exists!(types_nonptr::MyI8Alias));
}
