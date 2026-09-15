// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#[cfg(test)]
mod tests {
    use googletest::gtest;

    #[gtest]
    fn test_status_or_success() {
        let res = status_user_rust::call_make_status_or(42);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 42);
    }

    #[gtest]
    fn test_status_or_failure() {
        let res = status_user_rust::call_make_status_or(-1);
        assert!(res.is_err());
    }
}
