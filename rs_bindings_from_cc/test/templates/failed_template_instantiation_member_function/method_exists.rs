// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

/// Returns whether `$ty` has an inherent method or associated constant named `$method`.
#[macro_export]
macro_rules! method_exists {
    ($ty:ty, $method:ident) => {{
        struct Fallback;
        trait Probe {
            #[allow(non_upper_case_globals)]
            const $method: Fallback = Fallback;
        }
        impl<T> Probe for T {}
        ::core::any::Any::type_id(&<$ty>::$method)
            != ::core::any::Any::type_id(&<$ty as Probe>::$method)
    }};
}
