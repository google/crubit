// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#[cfg(test)]
mod tests {
    use upcast::*;

    #[test]
    fn test_upcast() {
        let derived = Derived::default();
        let derived = &derived;
        let base0: &Base0 = derived.into();
        assert_eq!(base0 as *const _ as usize, derived.base0_address());
        assert_eq!(base0 as *const _ as usize, derived as *const _ as usize);
        let base1: &Base1 = derived.into();
        assert_eq!(base1 as *const _ as usize, derived.base1_address());
        let base2: &Base2 = derived.into();
        assert_eq!(base2 as *const _ as usize, derived.base2_address());
        let base3: &Base3 = derived.into();
        assert_eq!(base3 as *const _ as usize, derived.base3_address());
        let base4: &Base4 = derived.into();
        assert_eq!(base4 as *const _ as usize, derived.base4_address());
    }
}
