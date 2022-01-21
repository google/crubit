// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#[cfg(test)]
mod tests {

    #[test]
    fn test_access_to_struct_through_the_right_crate() {
        // MyStruct was defined in a textual header of :defines_struct_in_textual_hdr,
        // but we should consider that header to belong to whichever target
        // ends up including it in a nontextual header, in this case
        // :uses_struct_from_textual_hdr_in_textual_hdr.
        let x = uses_struct_from_textual_hdr_in_textual_hdr::MyStruct { value: 3 };
        assert_eq!(uses_struct_from_textual_hdr_in_textual_hdr::getValue(x), 3);
    }
}
