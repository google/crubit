// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

/// The `cc_template!` macro tells the C++ / Rust interop tooling to instantiate
/// the specified C++ class template with the given arguments. The tooling will
/// generate bindings for the instantiation, and the macro will expand to a type
/// path for the Rust struct representing the instantiation.
///
/// Example:
///   Consider the following snippet of Rust code:
///   ```rust
///     fn make_bool_vector() -> cc_template!(std::vector<bool>) {...}
///   ```
///   The C++ / Rust interop tooling will detect the use of the `cc_template!`
///   macro with `std::vector<bool>` as an argument. The tooling will generate
///   bindings for the instantiation and it will expand to something like:
///   ```rust
///     fn make_bool_vector() ->
/// __cc_template_instantiations_rs_api::__CcTemplateInstSt6VectorIbE {
///       ...
///     }
///   ```
#[proc_macro]
pub fn cc_template(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    cc_template_impl::to_private_struct_path(input.into())
        .unwrap_or_else(|err| err.into_compile_error())
        .into()
}
