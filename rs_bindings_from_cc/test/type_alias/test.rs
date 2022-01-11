// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#[cfg(test)]
mod tests {
    #[test]
    fn test_return_value() {
        use type_alias::return_underlying;
        use type_alias::Int;
        let i: Int = 42;
        assert_eq!(return_underlying(i), 42);
    }
}
