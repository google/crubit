// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#[cfg(test)]
mod tests {
    use hello_world::hello_world;

    #[test]
    fn test_hello_world() {
        assert_eq!(hello_world(), 42);
    }
}
