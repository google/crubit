// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#[cfg(feature = "enable_bonus")]
pub fn bonus_value() -> i32 {
    crate::BaseValue() + 32
}

#[cfg(not(feature = "enable_bonus"))]
pub fn bonus_value() -> i32 {
    crate::BaseValue()
}

#[cfg(feature = "disabled_feature")]
pub fn disabled_feature_enabled() -> bool {
    true
}

#[cfg(not(feature = "disabled_feature"))]
pub fn disabled_feature_enabled() -> bool {
    false
}

#[cfg(all(custom_flag_a, custom_flag_b))]
pub fn custom_flags_enabled() -> bool {
    true
}

#[cfg(not(all(custom_flag_a, custom_flag_b)))]
pub fn custom_flags_enabled() -> bool {
    false
}

pub const FROM_RUSTC_ENV: &str = env!("MY_RUSTC_ENV_VAR");
pub const FROM_ENV_FILE: &str = env!("MY_ENV_FILE_VAR");
pub const FROM_LOCATION_ENV: &str = include_str!(env!("MY_COMPILE_DATA_PATH"));
