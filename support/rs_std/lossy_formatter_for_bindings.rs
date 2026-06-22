// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
use lossy_formatter::LossyFormatter;

/// FFI bindings for `LossyFormatter::write_bytes` to be linked to by
/// `lossy_formatter_for_bindings.h`.
///
/// # Safety
///
/// The caller must provide a valid `&mut LossyFormatter`.
/// `data` must point to `count` bytes. `data` may be null only if `count` is 0.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn crubit_LossyFormatter_write_bytes(
    formatter: &mut LossyFormatter<'_, '_>,
    data: *const ffi_11::c_char,
    count: usize,
) -> usize {
    let data = if count > 0 {
        // SAFETY: caller guarantees that `data` points to `count` bytes.
        unsafe { std::slice::from_raw_parts(data as *const u8, count) }
    } else {
        &[]
    };
    formatter.write_slice(data)
}

/// FFI bindings for `LossyFormatter::write_byte` to be linked to by
/// `lossy_formatter_for_bindings.h`.
///
/// # Safety
///
/// The caller must provide a valid `&mut LossyFormatter`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn crubit_LossyFormatter_write_byte(
    formatter: &mut LossyFormatter<'_, '_>,
    data: u8,
) -> bool {
    formatter.write_byte(data)
}

/// FFI bindings for `LossyFormatter::write_fill` to be linked to by
/// `lossy_formatter_for_bindings.h`.
///
/// # Safety
///
/// The caller must provide a valid `&mut LossyFormatter`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn crubit_LossyFormatter_write_fill(
    formatter: &mut LossyFormatter<'_, '_>,
    count: usize,
    data: u8,
) -> usize {
    formatter.write_fill(count, data)
}

/// FFI bindings for `LossyFormatter::flush` to be linked to by
/// `lossy_formatter_for_bindings.h`.
///
/// # Safety
///
/// The caller must provide a valid `&mut LossyFormatter`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn crubit_LossyFormatter_flush(
    formatter: &mut LossyFormatter<'_, '_>,
) -> bool {
    formatter.flush()
}
