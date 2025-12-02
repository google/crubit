// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:try_force_template_inst_cc

#![rustfmt::skip]
#![feature(allocator_api, cfg_sanitize, custom_inner_attributes)]
#![allow(stable_features)]
#![no_std]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code, unused_mut)]
#![deny(warnings)]

// Error while generating bindings for class 'Foo':
// Class templates are not supported yet

// Error while generating bindings for class 'Bar':
// Class templates are not supported yet

// Error while generating bindings for function 'Baz':
// Can't generate bindings for Baz, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:try_force_template_inst_cc needs [//features:wrapper] for Baz (the type of __param_0 (parameter #0): error: Can't generate bindings for Foo<Bar<int>> due to missing bindings for its dependency)

// Error while generating bindings for struct 'Foo<Bar<int>>':
// Can't generate bindings for Foo<Bar<int>> due to missing bindings for its dependency: Can't generate bindings for Bar<int>, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:try_force_template_inst_cc needs [//features:wrapper] for Bar<int> (incomplete type)

// Error while generating bindings for struct 'Bar<int>':
// Can't generate bindings for Bar<int>, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:try_force_template_inst_cc needs [//features:wrapper] for Bar<int> (incomplete type)
