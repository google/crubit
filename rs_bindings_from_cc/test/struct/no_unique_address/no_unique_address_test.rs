// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#[cfg(test)]
mod tests {
    use no_unique_address::*;

    #[test]
    fn test_get() {
        let s = Struct::Make(1, 2);
        assert_eq!(s.field1(), &1);
        assert_eq!(s.field2(), &2);
    }
}
