// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#[cfg(test)]
mod tests {
    use abi_class::*;

    #[test]
    fn test_struct_float() {
        let x = StructFloat::Create(123.0);
        let y = StructFloat::Create(456.0);
        let sum = StructFloat::Add(x, y);
        assert_eq!(123.0 + 456.0, StructFloat::Inspect(sum));
    }

    #[test]
    fn test_struct_memory() {
        let x = StructMemory::Create(456);
        let y = StructMemory::Create(321);
        let sum = StructMemory::Add(x, y);
        assert_eq!(456 + 321, StructMemory::Inspect(sum));
    }

    #[test]
    fn test_struct_integer() {
        let x = StructInteger::Create(456);
        let y = StructInteger::Create(789);
        let sum = StructInteger::Add(x, y);
        assert_eq!(456 + 789, StructInteger::Inspect(sum));
    }
}
