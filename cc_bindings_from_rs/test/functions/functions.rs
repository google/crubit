// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! This crate is used as a test input for `cc_bindings_from_rs` and the
//! generated C++ bindings are then tested via `functions_test.cc`.

/// APIs for testing various function calling conventions and linking options:
/// - `#[unsafe(no_mangle)]`
/// - `#[unsafe(export_name = ...)]`
/// - `extern "C"` vs default/Rust ABI
/// - etc.
pub mod fn_abi_tests {

    /// Testing one of simpler function bindings:
    /// - `extern "C"` means that no thunk is required
    /// - `#[unsafe(no_mangle)]` means that the function is already exposed with
    ///   the desired, public name (and just needs to be redeclared in C++).
    #[unsafe(no_mangle)]
    pub extern "C" fn get_42_as_f64_via_no_mangle_extern_c() -> f64 {
        42.0
    }

    /// Testing `#[unsafe(export_name = ...)]` - the generated bindings need to
    /// forward/proxy the call into a function with a different name.
    #[unsafe(export_name = "custom_export_name_for_add_i32")]
    pub extern "C" fn add_i32_via_extern_c_with_export_name(x: i32, y: i32) -> i32 {
        x + y
    }

    /// Testing bindings for an `extern "C"` function (no thunk required) with a
    /// mangled name. This test verifies that:
    /// * `cc_bindings_from_rs` can correctly discover mangled names that
    ///   `rustc` produces
    /// * Bazel support for `cc_bindings_from_rs` invokes it with the same
    ///   command line flags as the ones used when invoking `rustc` when
    ///   building the `functions` crate.
    ///
    /// TODO(b/262904507): Bazel integration is currently broken and the
    /// coresponding test is commented out in `functions_test.cc`.
    pub extern "C" fn add_i32_via_extern_c_with_mangling(x: i32, y: i32) -> i32 {
        x + y
    }

    /// Testing the default / Rust ABI (one used in absence of `extern "C"`).
    pub fn add_i32_via_rust_abi(x: i32, y: i32) -> i32 {
        x + y
    }
}

/// APIs for testing various kinds of function parameter types.
pub mod fn_param_ty_tests {
    /// Testing a type that maps to a built-in C++ type (spelled with a
    /// keyword). `float` is one such example.
    pub fn add_f64(x: f64, y: f64) -> f64 {
        x + y
    }

    /// Testing a type that requires `#include`ing a standard C++ header.
    /// `std::int32_t` is one such example - it requires `#include <cstdint>`.
    pub fn add_i32(x: i32, y: i32) -> i32 {
        x + y
    }

    pub fn add_i32_via_ptr(x: *const i32, y: *const i32, sum: *mut i32) {
        #![allow(clippy::not_unsafe_ptr_arg_deref)]
        unsafe {
            *sum = *x + *y;
        }
    }

    pub fn char_to_ascii_lowercase(c: char) -> char {
        // This function used to return unmodified `c` value, but (as we learned when
        // authoring `rs_bindings_from_cc/test/struct/abi_class` tests) making
        // some simple calculations below helps to exercise the ABI
        // compatibility between Rust `char` and C++ `rs_std::char_`.
        c.to_ascii_lowercase()
    }

    pub fn apply_binary_i32_op(x: i32, y: i32, f: extern "C" fn(i32, i32) -> i32) -> i32 {
        f(x, y)
    }

    pub fn get_ref_to_smaller_int<'a>(x: &'a i32, y: &'a i32) -> &'a i32 {
        if *x < *y {
            x
        } else {
            y
        }
    }

    pub fn get_identical_ref_with_inferred_lifetime(x: &'_ i32) -> &'_ i32 {
        x
    }

    pub fn set_mut_ref_to_sum_of_ints(sum: &mut i32, x: i32, y: i32) {
        *sum = x + y;
    }
}

/// APIs for testing functions that return the unit / `()` / `void` type.
pub mod unit_ret_ty_tests {
    use std::sync::Mutex;

    static G_I32: Mutex<i32> = Mutex::new(0);

    // Presence of the API below tests how bindings handle functions returning
    // `void`.
    #[unsafe(export_name = "custom_export_name_for_get_global_i32")]
    pub extern "C" fn set_global_i32_via_extern_c_with_export_name(x: i32) {
        *G_I32.lock().unwrap() = x;
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn get_global_i32_via_extern_c_with_export_name() -> i32 {
        *G_I32.lock().unwrap()
    }
}

pub mod other_fn_param_tests {
    pub fn add_i32_via_rust_abi_with_duplicated_param_names(x: i32, y: i32, _: i32, _: i32) -> i32 {
        x + y
    }
}

pub mod fn_attribute_tests {
    #[deprecated(since = "1.2.3", note = "★ Deprecated note for add_i32 ★")]
    pub fn add_i32(x: i32, y: i32) -> i32 {
        x + y
    }
}

pub mod unsafe_fn_tests {
    /// # Safety
    ///
    /// This function has no safety requirements - it is only marked as `unsafe`
    /// to facilitate minimal testing of bindings generated for such functions.
    pub unsafe fn unsafe_add(x: i32, y: i32) -> i32 {
        x + y
    }
}

// Tests the use of the #[must_use] attribute
pub mod fn_must_use_tests {
    #[must_use]
    pub fn no_msg_add(x: i32, y: i32) -> i32 {
        x + y
    }

    #[must_use = "woohoo"]
    pub fn msg_add(x: i32, y: i32) -> i32 {
        x + y
    }
}

pub mod generic_fn_tests {
    pub mod into_trait_tests {
        pub fn basic_test(arg: impl Into<i32>) -> i32 {
            arg.into()
        }

        pub fn where_clause<T>(x: T) -> i32
        where
            T: Into<i32>,
        {
            x.into()
        }

        pub fn reused_generic_param<T: Into<i32>>(x: T, y: T) -> i32 {
            x.into() + y.into()
        }

        pub fn multiple_generic_params(x: impl Into<i32>, y: impl Into<i32>) -> i32 {
            x.into() + y.into()
        }

        pub fn return_type<T: Into<i32> + Default>() -> T {
            T::default()
        }

        /// This test was initially added to cover/verify the call to
        /// `super_visit_with` from an `impl` of `GenericParamsFinder` in
        /// `get_generic_args.rs`.
        pub fn generic_param_nested_deeper_in_param_ty<T: Into<i32>>(xs: [T; 3]) -> i32 {
            xs.into_iter().map(Into::into).sum()
        }

        /// Bindings for `fn unused_generic_param` are not supported, because
        /// currently we don't spell out the generic type arguments in the
        /// generated code (depending on type inference instead).  This doesn't
        /// work for generic unused generic type parameters - e.g.:
        ///
        /// ```txt
        /// error[E0283]: type annotations needed
        ///
        /// unsafe { ::functions::generic_fn_tests::into_trait_tests::unused_generic_param() }
        ///          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        /// cannot infer type of the type parameter `T` declared on the function `unused_generic_param`
        /// ...
        /// help: consider specifying the generic argument
        ///
        /// unsafe { ::functions::generic_fn_tests::into_trait_tests::unused_generic_param::<T>() }
        /// ```
        pub fn unused_generic_param<T: Into<i32>>() {}
    }

    pub mod as_ref_trait_tests {
        pub fn slice_ref_sum(arg: impl AsRef<[i32]>) -> i32 {
            arg.as_ref().iter().sum()
        }

        /// The substitution `impl AsRef<[i32]>` => `&[u32]` needs to "conjure" a new, late-bound
        /// lifetime/region.  The test below is an ad-hoc attempt to test that the new region
        /// doesn't somehow clobber/conflict with existing implicit or explicit lifetimes.
        /// `impl AsRef<[i32]>` is "sandwiched" in the middle to increase the chances that
        /// a conflict would be caught somehow.  The test never failed, so it's unclear how
        /// useful it is.
        #[allow(clippy::needless_lifetimes)]
        pub fn diverse_lifetimes<'a, 'b>(
            arg1: &[i32],
            arg2: &'a [i32],
            arg3: impl AsRef<[i32]>,
            result: &'b mut [i32],
        ) {
            let sums = arg1
                .iter()
                .zip(arg2.iter())
                .map(|(x, y)| x + y)
                .zip(arg3.as_ref().iter())
                .map(|(x, y)| x + y);
            for (sum, result) in sums.zip(result.iter_mut()) {
                *result = sum;
            }
        }

        /// This is an attempt to trigger an error seen in
        /// https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=42303eaaafe4a3538ad259e9e9b67f05
        ///
        /// Today the error doesn't happen in Crubit, because the thunks explicitly
        /// declare all their lifetimes as `'static` - see `fn replace_all_regions_with_static`.
        pub fn static_lifetime_requirement<T>(arg: T) -> i32
        where
            T: AsRef<[i32]> + 'static,
        {
            arg.as_ref().iter().sum()
        }

        #[repr(transparent)]
        pub struct MyStruct(i32);

        impl MyStruct {
            pub fn new(x: i32) -> Self {
                MyStruct(x)
            }
        }

        impl AsRef<MyStruct> for MyStruct {
            fn as_ref(&self) -> &MyStruct {
                self
            }
        }

        pub fn struct_ref(arg: impl AsRef<MyStruct>) -> i32 {
            arg.as_ref().0
        }
    }

    pub mod as_mut_trait_tests {
        pub fn prefix_sums(mut arg: impl AsMut<[i32]>) {
            let mut sum = 0;
            for x in arg.as_mut() {
                sum += *x;
                *x = sum;
            }
        }
    }

    pub mod ctor_trait_tests {
        use crubit_annotate::must_bind;
        use ctor::{emplace, Ctor, CtorNew, Infallible, RustMoveCtor, RvalueReference};

        #[must_bind]
        pub struct CppMovable {
            pub value: i32,
        }

        impl CppMovable {
            pub fn new(value: i32) -> Self {
                CppMovable { value }
            }
        }

        impl<'a> CtorNew<RvalueReference<'a, Self>> for CppMovable {
            type CtorType = RustMoveCtor<Self>;
            type Error = Infallible;
            fn ctor_new(args: RvalueReference<'a, Self>) -> Self::CtorType {
                RustMoveCtor::new(Self::new(args.0.value))
            }
        }

        pub fn accept_ctor(c1: Ctor![CppMovable], c2: Ctor![CppMovable]) -> i32 {
            emplace!(c1).value + emplace!(c2).value
        }

        pub fn accept_rvalue_ref(c: RvalueReference<'_, CppMovable>) -> i32 {
            c.value
        }

        pub fn return_rvalue_reference<'a>(
            c: RvalueReference<'a, CppMovable>,
        ) -> RvalueReference<'a, CppMovable> {
            c
        }

        pub fn return_rvalue_reference_tuple<'a>(
            c: RvalueReference<'a, CppMovable>,
        ) -> (RvalueReference<'a, CppMovable>,) {
            (c,)
        }

        pub fn return_rvalue_reference_array<'a>(
            c: RvalueReference<'a, CppMovable>,
        ) -> [RvalueReference<'a, CppMovable>; 1] {
            [c]
        }

        pub fn return_ctor() -> impl Ctor<Output = CppMovable> {
            CppMovable::new(123)
        }

        // This needs the rvalue references to become pointers: tuples of references don't
        // mean what you would expect.
        pub fn accept_rvalue_reference_tuple(t: (RvalueReference<'_, CppMovable>,)) -> i32 {
            t.0.value
        }

        // This needs the rvalue references to become pointers: arrays can't hold references.
        pub fn accept_rvalue_reference_array(a: [RvalueReference<'_, CppMovable>; 3]) -> i32 {
            a.into_iter().map(|c| c.value).sum()
        }

        pub fn accept_ctor_tuple(c: (Ctor![CppMovable],)) -> i32 {
            emplace!(c.0).value
        }

        pub fn accept_ctor_array<C: Ctor<Output = CppMovable, Error = Infallible>>(
            a: [C; 3],
        ) -> i32 {
            a.into_iter().map(|c| emplace!(c).value).sum()
        }
    }
}
