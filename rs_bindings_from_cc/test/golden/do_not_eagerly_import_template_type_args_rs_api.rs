// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:do_not_eagerly_import_template_type_args_cc

#![rustfmt::skip]
#![feature(custom_inner_attributes)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
#![deny(warnings)]

// error: class `DoesNotUse` could not be bound
//   Class templates are not yet supported

// `DoesNotUse<DoesNotUse<int>>` does not instantiate the inner template
// parameter, `DoesNotUse<int>`, but it _is_ instantiatable. This test
// shows that we should not import it in its uninstantiated form, otherwise
// ImportedSecond will read this cached import and think that `DoesNotUse<int>`
// is incomplete, which is false.

// error: function `ImportedFirst` could not be bound
//   Unsupported parameter #0 (__param_0): template instantiation is not yet supported
//   template instantiation is not yet supported

// We expect ImportedSecond to fail because we need wrapper mode, _not_ because
// `DoesNotUse<int>` is incomplete.

// error: function `ImportedSecond` could not be bound
//   Unsupported parameter #0 (__param_0): template instantiation is not yet supported
//   template instantiation is not yet supported

// error: struct `DoesNotUse<DoesNotUse<int>>` could not be bound
//   template instantiation is not yet supported
//   template instantiation is not yet supported

// error: struct `DoesNotUse<int>` could not be bound
//   template instantiation is not yet supported
//   template instantiation is not yet supported
