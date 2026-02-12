// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:do_not_eagerly_import_template_type_args_cc

#![rustfmt::skip]
#![feature(allocator_api, cfg_sanitize, custom_inner_attributes)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code, unused_mut)]
#![deny(warnings)]

// Error while generating bindings for class 'DoesNotUse':
// Class templates are not supported yet

// `DoesNotUse<DoesNotUse<int>>` does not instantiate the inner template
// parameter, `DoesNotUse<int>`, but it _is_ instantiatable. This test
// shows that we should not import it in its uninstantiated form, otherwise
// ImportedSecond will read this cached import and think that `DoesNotUse<int>`
// is incomplete, which is false.

// Error while generating bindings for function 'ImportedFirst':
// Can't generate bindings for ImportedFirst, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:do_not_eagerly_import_template_type_args_cc needs [//features:wrapper] for ImportedFirst (the type of __param_0 (parameter #0): error: Can't generate bindings for DoesNotUse<DoesNotUse<int>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:do_not_eagerly_import_template_type_args_cc needs [//features:wrapper] for DoesNotUse<DoesNotUse<int>> (crate::__CcTemplateInst10DoesNotUseIS_IiEE is a template instantiation)
// //rs_bindings_from_cc/test/golden:do_not_eagerly_import_template_type_args_cc needs [//features:wrapper] for DoesNotUse<DoesNotUse<int>> (crate::__CcTemplateInst10DoesNotUseIS_IiEE is a template instantiation))

// We expect ImportedSecond to fail because we need wrapper mode, _not_ because
// `DoesNotUse<int>` is incomplete.

// Error while generating bindings for function 'ImportedSecond':
// Can't generate bindings for ImportedSecond, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:do_not_eagerly_import_template_type_args_cc needs [//features:wrapper] for ImportedSecond (the type of __param_0 (parameter #0): error: Can't generate bindings for DoesNotUse<int>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:do_not_eagerly_import_template_type_args_cc needs [//features:wrapper] for DoesNotUse<int> (crate::__CcTemplateInst10DoesNotUseIiE is a template instantiation)
// //rs_bindings_from_cc/test/golden:do_not_eagerly_import_template_type_args_cc needs [//features:wrapper] for DoesNotUse<int> (crate::__CcTemplateInst10DoesNotUseIiE is a template instantiation))

// Error while generating bindings for struct 'DoesNotUse<DoesNotUse<int>>':
// Can't generate bindings for DoesNotUse<DoesNotUse<int>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:do_not_eagerly_import_template_type_args_cc needs [//features:wrapper] for DoesNotUse<DoesNotUse<int>> (crate::__CcTemplateInst10DoesNotUseIS_IiEE is a template instantiation)
// //rs_bindings_from_cc/test/golden:do_not_eagerly_import_template_type_args_cc needs [//features:wrapper] for DoesNotUse<DoesNotUse<int>> (crate::__CcTemplateInst10DoesNotUseIS_IiEE is a template instantiation)

// Error while generating bindings for struct 'DoesNotUse<int>':
// Can't generate bindings for DoesNotUse<int>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:do_not_eagerly_import_template_type_args_cc needs [//features:wrapper] for DoesNotUse<int> (crate::__CcTemplateInst10DoesNotUseIiE is a template instantiation)
// //rs_bindings_from_cc/test/golden:do_not_eagerly_import_template_type_args_cc needs [//features:wrapper] for DoesNotUse<int> (crate::__CcTemplateInst10DoesNotUseIiE is a template instantiation)
