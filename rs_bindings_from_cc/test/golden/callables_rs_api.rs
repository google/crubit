// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:callables_cc

#![rustfmt::skip]
#![feature(allocator_api, cfg_sanitize, custom_inner_attributes)]
#![allow(stable_features)]
#![no_std]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code, unused_mut)]
#![deny(warnings)]

// Error while generating bindings for function 'apply':
// Can't generate bindings for apply, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:callables_cc needs [//features:wrapper] for apply (the type of callback (parameter #0): error: Can't generate bindings for rs_std::DynCallable<int (int) const> due to missing bindings for its dependency)

// Error while generating bindings for function 'apply_mut':
// Can't generate bindings for apply_mut, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:callables_cc needs [//features:wrapper] for apply_mut (the type of callback (parameter #0): error: Can't generate bindings for rs_std::DynCallable<int (int)> due to missing bindings for its dependency)

// Error while generating bindings for function 'apply_once':
// Can't generate bindings for apply_once, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:callables_cc needs [//features:wrapper] for apply_once (the type of callback (parameter #0): error: Can't generate bindings for rs_std::DynCallable<int (int) &&> due to missing bindings for its dependency)

// Error while generating bindings for struct 'std::integral_constant<bool, false>':
// Can't generate bindings for std::integral_constant<bool, false>, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:callables_cc needs [//features:wrapper] for std::integral_constant<bool, false> (crate::__CcTemplateInstNSt3__u17integral_constantIbLb0EEE is a template instantiation)

// Error while generating bindings for struct 'std::integral_constant<bool, true>':
// Can't generate bindings for std::integral_constant<bool, true>, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:callables_cc needs [//features:wrapper] for std::integral_constant<bool, true> (crate::__CcTemplateInstNSt3__u17integral_constantIbLb1EEE is a template instantiation)

// Error while generating bindings for class 'rs_std::DynCallable<int (int)>':
// Can't generate bindings for rs_std::DynCallable<int (int)> due to missing bindings for its dependency: Failed to get type from template arg: Unsupported type 'int (int)': Unsupported clang::Type class 'FunctionProto'

// Error while generating bindings for class 'rs_std::DynCallable<int (int) &&>':
// Can't generate bindings for rs_std::DynCallable<int (int) &&> due to missing bindings for its dependency: Failed to get type from template arg: Unsupported type 'int (int) &&': Unsupported clang::Type class 'FunctionProto'

// Error while generating bindings for class 'rs_std::DynCallable<int (int) const>':
// Can't generate bindings for rs_std::DynCallable<int (int) const> due to missing bindings for its dependency: Failed to get type from template arg: Unsupported type 'int (int) const': Unsupported clang::Type class 'FunctionProto'
