// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

pub struct GetsResult {
    pub value: Result<u32, u32>,
}
impl GetsResult {
    pub fn new(val: u32) -> Self {
        GetsResult { value: Ok(val) }
    }
}

pub struct NestedResult {
    pub in_ok: Result<Result<u32, u32>, u32>,
    pub in_err: Result<u32, Result<u32, u32>>,
}

impl NestedResult {
    pub fn new(val: u32) -> Self {
        NestedResult { in_ok: Ok(Err(val)), in_err: Err(Ok(val)) }
    }
}

#[derive(Copy, Clone)]
pub struct CopyNoDefault {
    pub val: u8,
}

pub struct CopyNoDefaultResult {
    pub in_ok: Result<CopyNoDefault, u8>,
    pub in_err: Result<u8, CopyNoDefault>,
}
impl CopyNoDefaultResult {
    pub fn new(val: u8) -> Self {
        CopyNoDefaultResult { in_ok: Ok(CopyNoDefault { val }), in_err: Err(CopyNoDefault { val }) }
    }
}

#[derive(Clone)]
pub struct CloneNoDefault {
    pub val: u8,
}
pub struct CloneNoDefaultResult {
    pub in_ok: Result<CloneNoDefault, u8>,
    pub in_err: Result<u8, CloneNoDefault>,
}
impl CloneNoDefaultResult {
    pub fn new(val: u8) -> Self {
        CloneNoDefaultResult {
            in_ok: Ok(CloneNoDefault { val }),
            in_err: Err(CloneNoDefault { val }),
        }
    }
}

#[derive(Default)]
pub struct HasDefault {
    pub val: String,
}
impl HasDefault {
    pub fn new(val: &str) -> Self {
        HasDefault { val: val.to_string() }
    }

    pub fn val(&self) -> &str {
        &self.val
    }
}
pub struct HasDefaultResult {
    pub in_ok: Result<HasDefault, u8>,
    pub in_err: Result<u8, HasDefault>,
}
impl HasDefaultResult {
    pub fn new(val: &str) -> Self {
        HasDefaultResult {
            in_ok: Ok(HasDefault { val: val.to_string() }),
            in_err: Err(HasDefault { val: val.to_string() }),
        }
    }
}

pub struct HasNoDefault {
    pub val: String,
}
impl HasNoDefault {
    pub fn val(&self) -> &str {
        &self.val
    }
}
pub struct HasNoDefaultResult {
    pub in_ok: Result<HasNoDefault, u8>,
    pub in_err: Result<u8, HasNoDefault>,
}
impl HasNoDefaultResult {
    pub fn new(val: &str) -> Self {
        HasNoDefaultResult {
            in_ok: Ok(HasNoDefault { val: val.to_string() }),
            in_err: Err(HasNoDefault { val: val.to_string() }),
        }
    }
}
pub fn take_result_copy_no_default_ok(r: &Result<CopyNoDefault, u8>) -> u8 {
    match r {
        Ok(v) => v.val,
        Err(v) => *v,
    }
}

pub fn take_result_clone_no_default_err(r: &Result<u8, CloneNoDefault>) -> u8 {
    match r {
        Ok(v) => *v,
        Err(e) => e.val,
    }
}

pub fn take_result_has_default(r: &Result<HasDefault, u8>) -> &str {
    match r {
        Ok(v) => v.val(),
        Err(_) => "a number",
    }
}

pub fn take_result_by_value(r: Result<u8, u8>) -> u8 {
    match r {
        Ok(v) => v,
        Err(e) => e,
    }
}

pub fn return_result_by_value() -> Result<u8, u8> {
    Ok(1)
}

pub struct ResultWithSizeTypes {
    // b/491106325 - We expect these not to get bindings.
    pub uval_in_ok: Result<usize, u8>,
    pub uval_in_err: Result<u8, usize>,
    pub ival_in_ok: Result<isize, i8>,
    pub ival_in_err: Result<i8, isize>,
}

// Replicate failure around pointer types from zlib_rs.
pub type Voidpf = *mut std::ffi::c_void;
pub type FreeFunc = unsafe extern "C" fn(Voidpf, Voidpf);

// We just need to confirm the bindings received compile.
pub struct ZStream {
    pub zfree: Result<FreeFunc, FreeFunc>,
}
