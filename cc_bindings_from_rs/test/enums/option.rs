// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use crubit_annotate::must_bind;

#[must_bind]
#[derive(Copy, Clone)]
#[repr(u8)]
pub enum LessThan20U8 {
    N0,
    N1,
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
    N10,
    N11,
    N12,
    N13,
    N14,
    N15,
    N16,
    N17,
    N18,
    N19,
}

impl LessThan20U8 {
    #[must_bind]
    pub fn new(value: u8) -> Option<Self> {
        if value > 19 {
            return None;
        }
        // Safety: we're sure the value is in 0-19,
        // and this enum is `repr(u8)`.
        Some(unsafe { std::mem::transmute(value) })
    }

    #[must_bind]
    pub fn value(self) -> u8 {
        self as u8
    }
}

#[must_bind]
pub struct HasOptions {
    pub niche: Option<LessThan20U8>,
    pub nested: Option<Option<LessThan20U8>>,
    pub direct: Option<u8>,
}

impl HasOptions {
    #[must_bind]
    pub fn new(value: u8) -> Self {
        HasOptions {
            niche: LessThan20U8::new(value),
            nested: Some(LessThan20U8::new(value)),
            direct: Some(value),
        }
    }

    #[must_bind]
    pub fn with_option(value: Option<u8>) -> Self {
        let lt20 = value.and_then(LessThan20U8::new);
        HasOptions { niche: lt20, nested: Some(lt20), direct: value }
    }

    #[must_bind]
    pub fn from_ref(value: &Option<u8>) -> Self {
        match value {
            Some(v) => HasOptions::new(*v),
            None => HasOptions::with_none(),
        }
    }

    #[must_bind]
    pub fn with_none() -> Self {
        HasOptions { niche: None, nested: None, direct: None }
    }
}

#[must_bind]
pub struct HasHasOptions {
    pub me: Option<HasOptions>,
}

impl HasHasOptions {
    #[must_bind]
    pub fn new(value: u8) -> Self {
        HasHasOptions { me: Some(HasOptions::new(value)) }
    }
}

#[must_bind]
#[derive(Default)]
pub struct HasDefault {
    pub foo: String,
}

impl HasDefault {
    #[must_bind]
    pub fn new(s: &str) -> Self {
        Self { foo: s.to_string() }
    }

    #[must_bind]
    pub fn get_string_inside_option(&self) -> &str {
        &self.foo
    }
}

#[must_bind]
pub struct OptDefaultWithDrop {
    pub opt: Option<HasDefault>,
}
impl OptDefaultWithDrop {
    #[must_bind]
    pub fn new(s: &str) -> Self {
        Self { opt: Some(HasDefault { foo: s.to_string() }) }
    }
}

#[must_bind]
pub struct HasNoDefault {
    pub foo: String,
    pub a: u32,
}
impl HasNoDefault {
    #[must_bind]
    pub fn new(s: &str) -> Self {
        Self { foo: s.to_string(), a: 3033 }
    }
    #[must_bind]
    pub fn get_string_inside_option(&self) -> &str {
        &self.foo
    }
}

#[must_bind]
pub struct OptNoDefaultWithDrop {
    pub val: Option<HasNoDefault>,
}

impl OptNoDefaultWithDrop {
    #[must_bind]
    pub fn new(s: &str) -> Self {
        Self { val: Some(HasNoDefault { foo: s.to_string(), a: 1045 }) }
    }

    #[must_bind]
    pub fn get_string_inside_option(&self) -> &str {
        self.val.as_ref().unwrap().get_string_inside_option()
    }
}

#[must_bind]
#[derive(Clone)]
pub struct CloneNoDefault {
    pub val: u8,
}

#[must_bind]
#[derive(Clone)]
pub struct OptCloneNoDefault {
    pub val: Option<CloneNoDefault>,
}
impl OptCloneNoDefault {
    #[must_bind]
    pub fn new(x: u8) -> Self {
        Self { val: Some(CloneNoDefault { val: x }) }
    }
}

#[must_bind]
#[derive(Copy, Clone)]
pub struct CopyNoDefault {
    pub val: u8,
}

#[must_bind]
#[derive(Copy, Clone)]
pub struct OptCopyNoDefault {
    pub val: Option<CopyNoDefault>,
}

impl OptCopyNoDefault {
    #[must_bind]
    pub fn new(x: u8) -> Self {
        Self { val: Some(CopyNoDefault { val: x }) }
    }
}

// 4. Uninhabited type
pub enum UninhabitedEnum {}

pub struct OptUninhabited {
    pub val: Option<UninhabitedEnum>,
}

// 5. Zero sized type
pub struct Unit;

#[must_bind]
#[derive(Default)]
pub struct OptZst {
    pub val: Option<Unit>,
}

#[must_bind]
pub fn stringify_len(x: &Option<HasDefault>) -> Option<u32> {
    x.as_ref().map(|y| y.get_string_inside_option().len() as u32)
}

#[must_bind]
pub struct OptionWithSizeTypes {
    // b/491106325 - We expect these not to get bindings.
    pub uval: Option<usize>,
    pub ival: Option<isize>,
}

#[doc = "CRUBIT_ANNOTATE: cpp_type=int"]
#[repr(transparent)]
pub struct BridgedType(i32);

pub fn take_option_bridged(x: Option<BridgedType>) -> i32 {
    x.map(|b| b.0).unwrap_or(-1)
}

#[must_bind]
pub fn return_option_result() -> Option<Result<i32, String>> {
    Some(Ok(1))
}

#[must_bind]
#[allow(clippy::type_complexity)]
pub fn stress_testing_nested_types(
) -> Option<Result<Option<Result<i32, String>>, Result<Option<i32>, Option<i32>>>> {
    None
}

pub fn take_option_result_unmovable(_x: Option<Result<HasNoDefault, String>>) {}

pub fn return_option_result_unmovable() -> Option<Result<HasNoDefault, String>> {
    None
}

#[must_bind]
pub fn pass_option_ptr(x: Option<*const i32>) -> Option<*const i32> {
    x
}

// Replicate failure around pointer types from zlib_rs.
pub type Voidpf = *mut std::ffi::c_void;
pub type FreeFunc = unsafe extern "C" fn(Voidpf, Voidpf);

// We just need to confirm the bindings received compile.
pub struct ZStream {
    pub zfree: Option<FreeFunc>,
}
