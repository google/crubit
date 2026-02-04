// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
#![cfg(test)]

use arc_anyhow::Result;
use googletest::prelude::*;
use ir_matchers::assert_ir_matches;
use ir_testing::with_full_lifetime_macros;
use lifetime_defaults_transform::{lifetime_defaults_transform, BindingContext};
use multiplatform_ir_testing::ir_from_assumed_lifetimes_cc;
use quote::quote;
use std::rc::Rc;

#[gtest]
fn test_fn_with_no_unbound_lifetimes_is_unchanged() -> Result<()> {
    let ir = ir_from_assumed_lifetimes_cc(
        &(with_full_lifetime_macros()
            + r#"
      LIFETIME_PARAMS("a")
      int& $a f(int& $a i1, int& $a i2);
      "#),
    )?;
    let dir = lifetime_defaults_transform(&ir)?;
    assert_ir_matches!(
        dir,
        quote! {
            Func {
                cc_name: "f",
                rs_name: "f", ...
                return_type: CcType { ... explicit_lifetimes: ["a"] ... }, ...
                params: [
                    FuncParam {
                        type_: CcType { ... explicit_lifetimes: ["a"] ... },
                        identifier: "i1", ...
                    },
                    FuncParam {
                        type_: CcType { ... explicit_lifetimes: ["a"] ... },
                        identifier: "i2", ...
                    },
                ],
                ...
                lifetime_inputs: ["a"],
                ...
            }
        }
    );
    Ok(())
}

#[gtest]
fn test_no_change_if_binder_is_already_added_to_function() -> Result<()> {
    let ir = ir_from_assumed_lifetimes_cc(
        &(with_full_lifetime_macros()
            + r#"
      int& $a f(int& $a i1, int& $a i2);
      "#),
    )?;
    let dir = lifetime_defaults_transform(&ir)?;
    assert_ir_matches!(
        dir,
        quote! {
            Func {
                cc_name: "f",
                rs_name: "f", ...
                return_type: CcType { ... explicit_lifetimes: ["a"] ... }, ...
                params: [
                    FuncParam {
                        type_: CcType { ... explicit_lifetimes: ["a"] ... },
                        identifier: "i1", ...
                    },
                    FuncParam {
                        type_: CcType { ... explicit_lifetimes: ["a"] ... },
                        identifier: "i2", ...
                    },
                ],
                ...
                lifetime_inputs: ["a"],
                ...
            }
        }
    );
    Ok(())
}

#[gtest]
fn test_unique_lifetime_ascribed_to_single_ref() -> Result<()> {
    let ir = ir_from_assumed_lifetimes_cc(
        &(with_full_lifetime_macros()
            + r#"
      void f(int& i1);
      "#),
    )?;
    let dir = lifetime_defaults_transform(&ir)?;
    assert_ir_matches!(
        dir,
        quote! {
            Func {
                cc_name: "f",
                rs_name: "f", ...
                params: [
                    FuncParam {
                        type_: CcType { ... explicit_lifetimes: ["i1"] ... },
                       identifier: "i1", ...
                    }
                ],
                ...
                lifetime_inputs: ["i1"],
                ...
            }
        }
    );
    Ok(())
}

#[gtest]
fn test_distinct_lifetime_returned_for_annotated_ref() -> Result<()> {
    let ir = ir_from_assumed_lifetimes_cc(
        &(with_full_lifetime_macros()
            + r#"
      int& $b f(int& $a i1);
      "#),
    )?;
    let dir = lifetime_defaults_transform(&ir)?;
    assert_ir_matches!(
        dir,
        quote! {
            Func {
                cc_name: "f",
                rs_name: "f", ...
                return_type: CcType { ... explicit_lifetimes: ["b"] ... }, ...
                params: [
                    FuncParam {
                        type_: CcType { ... explicit_lifetimes: ["a"] ... },
                        identifier: "i1", ...
                    }
                ],
                ...
                lifetime_inputs: ["a", "b"],
                ...
            }
        }
    );
    Ok(())
}

#[gtest]
fn test_unique_lifetime_returned_for_single_ref() -> Result<()> {
    let ir = ir_from_assumed_lifetimes_cc(
        &(with_full_lifetime_macros()
            + r#"
      int& f(int& i1);
      "#),
    )?;
    let dir = lifetime_defaults_transform(&ir)?;
    assert_ir_matches!(
        dir,
        quote! {
            Func {
                cc_name: "f",
                rs_name: "f", ...
                return_type: CcType { ... explicit_lifetimes: ["i1"] ... }, ...
                params: [
                    FuncParam {
                        type_: CcType { ... explicit_lifetimes: ["i1"] ... },
                        identifier: "i1", ...
                    }
                ],
                ...
                lifetime_inputs: ["i1"],
                ...
            }
        }
    );
    Ok(())
}

#[gtest]
fn test_no_lifetime_returned_for_distinct_ref_parameters() -> Result<()> {
    let ir = ir_from_assumed_lifetimes_cc(
        &(with_full_lifetime_macros()
            + r#"
      int& f(int& i1, int& i2);
      "#),
    )?;
    let dir = lifetime_defaults_transform(&ir)?;
    assert_ir_matches!(
        dir,
        quote! {
            Func {
                cc_name: "f",
                rs_name: "f", ...
                return_type: CcType { ... explicit_lifetimes: [] ... }, ...
                params: [
                    FuncParam {
                        type_: CcType { ... explicit_lifetimes: ["i1"] ... },
                        identifier: "i1", ...
                    },
                    FuncParam {
                        type_: CcType { ... explicit_lifetimes: ["i2"] ... },
                        identifier: "i2", ...
                    },
                ],
                ...
                lifetime_inputs: ["i1", "i2"],
                ...
            }
        }
    );
    Ok(())
}

#[gtest]
fn test_no_lifetime_assigned_for_nullary_fn() -> Result<()> {
    let ir = ir_from_assumed_lifetimes_cc(
        &(with_full_lifetime_macros()
            + r#"
      int& f();
      "#),
    )?;
    let dir = lifetime_defaults_transform(&ir)?;
    assert_ir_matches!(
        dir,
        quote! {
            Func {
                cc_name: "f",
                rs_name: "f", ...
                return_type: CcType { ... explicit_lifetimes: [] ... }, ...
                params: [],
                lifetime_params: [],
                ...
            }
        }
    );
    Ok(())
}

#[gtest]
fn test_lifetimebound_param_with_decl_type() -> Result<()> {
    let ir = ir_from_assumed_lifetimes_cc(
        &(with_full_lifetime_macros()
            + r#"
      struct S {};
      S f(S i1 [[clang::lifetimebound]]);
      "#),
    )?;
    let dir = lifetime_defaults_transform(&ir);
    assert!(dir.is_err());
    Ok(())
}

#[gtest]
fn test_lifetimebound_param_with_fnptr_type() -> Result<()> {
    let ir = ir_from_assumed_lifetimes_cc(
        &(with_full_lifetime_macros()
            + r#"
      void (*f (void (*i1 [[clang::lifetimebound]])())) ();
      "#),
    )?;
    let dir = lifetime_defaults_transform(&ir);
    assert!(dir.is_err());
    Ok(())
}

#[gtest]
fn test_lifetimebound_param_is_returned_with_lifetime() -> Result<()> {
    let ir = ir_from_assumed_lifetimes_cc(
        &(with_full_lifetime_macros()
            + r#"
      int& f(int& i1 [[clang::lifetimebound]]);
      "#),
    )?;
    let dir = lifetime_defaults_transform(&ir)?;
    assert_ir_matches!(
        dir,
        quote! {
            Func {
                cc_name: "f",
                rs_name: "f", ...
                return_type: CcType { ... explicit_lifetimes: ["__rv"] ... }, ...
                ...
                params: [
                    FuncParam {
                       type_: CcType { ... explicit_lifetimes: ["__rv"] ... },
                       identifier: "i1", ...
                       ...
                       clang_lifetimebound: true,
                       ...
                    }
                ],
                ...
                lifetime_inputs: ["__rv"],
                ...
            }
        }
    );
    Ok(())
}

#[gtest]
fn test_lifetimebound_param_is_returned_with_lifetime_and_other_param() -> Result<()> {
    let ir = ir_from_assumed_lifetimes_cc(
        &(with_full_lifetime_macros()
            + r#"
      int& f(int& i1 [[clang::lifetimebound]], int& i2);
      "#),
    )?;
    let dir = lifetime_defaults_transform(&ir)?;
    assert_ir_matches!(
        dir,
        quote! {
            Func {
                cc_name: "f",
                rs_name: "f", ...
                return_type: CcType { ... explicit_lifetimes: ["__rv"] ... }, ...
                ...
                params: [
                    FuncParam {
                       type_: CcType { ... explicit_lifetimes: ["__rv"] ... },
                       identifier: "i1", ...
                       ...
                       clang_lifetimebound: true,
                       ...
                    },
                    FuncParam {
                       type_: CcType { ... explicit_lifetimes: ["i2"] ... },
                       identifier: "i2", ...
                       ...
                       clang_lifetimebound: false,
                       ...
                    },
                ],
                ...
                lifetime_inputs: ["__rv", "i2"],
                ...
            }
        }
    );
    Ok(())
}

#[gtest]
fn test_lifetimebound_param_is_returned_with_rv_unified_lifetime() -> Result<()> {
    let ir = ir_from_assumed_lifetimes_cc(
        &(with_full_lifetime_macros()
            + r#"
      int& $a f(int& i1 [[clang::lifetimebound]]);
      "#),
    )?;
    let dir = lifetime_defaults_transform(&ir)?;
    assert_ir_matches!(
        dir,
        quote! {
            Func {
                cc_name: "f",
                rs_name: "f", ...
                return_type: CcType { ... explicit_lifetimes: ["a"] ... }, ...
                ...
                params: [
                    FuncParam {
                       type_: CcType { ... explicit_lifetimes: ["a"] ... },
                       identifier: "i1", ...
                       ...
                       clang_lifetimebound: true,
                       ...
                    }
                ],
                ...
                lifetime_inputs: ["a"],
                ...
            }
        }
    );
    Ok(())
}

#[gtest]
fn test_lifetimebound_param_is_returned_with_param_unified_lifetime() -> Result<()> {
    let ir = ir_from_assumed_lifetimes_cc(
        &(with_full_lifetime_macros()
            + r#"
      int& f(int& i1 [[clang::lifetimebound]], int& $a i2 [[clang::lifetimebound]]);
      "#),
    )?;
    let dir = lifetime_defaults_transform(&ir)?;
    assert_ir_matches!(
        dir,
        quote! {
            Func {
                cc_name: "f",
                rs_name: "f", ...
                return_type: CcType { ... explicit_lifetimes: ["a"] ... }, ...
                ...
                params: [
                    FuncParam {
                       type_: CcType { ... explicit_lifetimes: ["a"] ... },
                       identifier: "i1", ...
                       ...
                       clang_lifetimebound: true,
                       ...
                    },
                    FuncParam {
                       type_: CcType { ... explicit_lifetimes: ["a"] ... },
                       identifier: "i2", ...
                       ...
                       clang_lifetimebound: true,
                       ...
                    },
                ],
                ...
                lifetime_inputs: ["a"],
                ...
            }
        }
    );
    Ok(())
}

#[gtest]
fn test_lifetimebound_param_is_returned_with_param_fresh_unified_lifetime() -> Result<()> {
    let ir = ir_from_assumed_lifetimes_cc(
        &(with_full_lifetime_macros()
            + r#"
      int& f(int& i1 [[clang::lifetimebound]], int& i2 [[clang::lifetimebound]]);
      "#),
    )?;
    let dir = lifetime_defaults_transform(&ir)?;
    assert_ir_matches!(
        dir,
        quote! {
            Func {
                cc_name: "f",
                rs_name: "f", ...
                return_type: CcType { ... explicit_lifetimes: ["__rv"] ... }, ...
                ...
                params: [
                    FuncParam {
                       type_: CcType { ... explicit_lifetimes: ["__rv"] ... },
                       identifier: "i1", ...
                       ...
                       clang_lifetimebound: true,
                       ...
                    },
                    FuncParam {
                       type_: CcType { ... explicit_lifetimes: ["__rv"] ... },
                       identifier: "i2", ...
                       ...
                       clang_lifetimebound: true,
                       ...
                    },
                ],
                ...
                lifetime_inputs: ["__rv"],
                ...
            }
        }
    );
    Ok(())
}

#[gtest]
fn test_this_lifetime_returned_for_nullary_member_function() -> Result<()> {
    let ir = ir_from_assumed_lifetimes_cc(
        &(with_full_lifetime_macros()
            + r#"
      struct S { int& f(); };
      "#),
    )?;
    let dir = lifetime_defaults_transform(&ir)?;
    assert_ir_matches!(
        dir,
        quote! {
            Func {
                cc_name: "f",
                rs_name: "f", ...
                return_type: CcType { ... explicit_lifetimes: ["__this"] ... }, ...
                params: [
                    FuncParam {
                        type_: CcType { ... explicit_lifetimes: ["__this"] ... },
                        identifier: "__this", ...
                    }
                ],
                lifetime_params: [],
                ...
                lifetime_inputs: ["__this"],
                ...
            }
        }
    );
    Ok(())
}

#[gtest]
fn test_explicit_this_lifetime_returned_for_nullary_member_function() -> Result<()> {
    let ir = ir_from_assumed_lifetimes_cc(
        &(with_full_lifetime_macros()
            + r#"
      struct S { int& f() $a; };
      "#),
    )?;
    let dir = lifetime_defaults_transform(&ir)?;
    assert_ir_matches!(
        dir,
        quote! {
            Func {
                cc_name: "f",
                rs_name: "f", ...
                return_type: CcType { ... explicit_lifetimes: ["a"] ... }, ...
                params: [
                    FuncParam {
                        type_: CcType { ... explicit_lifetimes: ["a"] ... },
                        identifier: "__this", ...
                    }
                ],
                lifetime_params: [],
                ...
                lifetime_inputs: ["a"],
                ...
            }
        }
    );
    Ok(())
}

#[gtest]
fn test_very_explicit_this_lifetime_returned_for_nullary_member_function() -> Result<()> {
    let ir = ir_from_assumed_lifetimes_cc(
        &(with_full_lifetime_macros()
            + r#"
      struct S { int& $a f() $a; };
      "#),
    )?;
    let dir = lifetime_defaults_transform(&ir)?;
    assert_ir_matches!(
        dir,
        quote! {
            Func {
                cc_name: "f",
                rs_name: "f", ...
                return_type: CcType { ... explicit_lifetimes: ["a"] ... }, ...
                params: [
                    FuncParam {
                        type_: CcType { ... explicit_lifetimes: ["a"] ... },
                        identifier: "__this", ...
                    }
                ],
                lifetime_params: [],
                ...
                lifetime_inputs: ["a"],
                ...
            }
        }
    );
    Ok(())
}

#[gtest]
fn test_this_lifetime_returned_for_member_function_with_reference_param() -> Result<()> {
    let ir = ir_from_assumed_lifetimes_cc(
        &(with_full_lifetime_macros()
            + r#"
      struct S { int& f(int& i1); };
      "#),
    )?;
    let dir = lifetime_defaults_transform(&ir)?;
    assert_ir_matches!(
        dir,
        quote! {
            Func {
                cc_name: "f",
                rs_name: "f", ...
                return_type: CcType { ... explicit_lifetimes: ["__this"] ... }, ...
                params: [
                    FuncParam {
                        type_: CcType { ... explicit_lifetimes: ["__this"] ... },
                        identifier: "__this", ...
                    },
                    FuncParam {
                        type_: CcType { ... explicit_lifetimes: ["i1"] ... },
                        identifier: "i1", ...
                    },
                ],
                lifetime_params: [],
                ...
                lifetime_inputs: ["__this", "i1"],
                ...
            }
        }
    );
    Ok(())
}

#[gtest]
fn test_binding_context_has_static() -> Result<()> {
    let mut ctx = BindingContext::new();
    let mut called = false;
    assert_eq!(
        ctx.get_or_push_new_binding(&Rc::from("static"), |_| called = true),
        "static".into()
    );
    assert!(!called);
    Ok(())
}

#[gtest]
fn test_binding_context_shadows_static() -> Result<()> {
    let mut ctx = BindingContext::new();
    assert_eq!(ctx.push_new_binding(&Rc::from("static")), "static_0".into());
    Ok(())
}

#[gtest]
fn test_binding_context_push_and_pop_unique_names() -> Result<()> {
    let mut ctx = BindingContext::new();
    let mut called = false;
    assert_eq!(ctx.get_or_push_new_binding(&Rc::from("a"), |a| called = **a == *"a"), "a".into());
    assert!(called);
    called = false;
    assert_eq!(ctx.get_or_push_new_binding(&Rc::from("a"), |_| called = true), "a".into());
    assert!(!called);
    assert_eq!(ctx.push_new_binding(&Rc::from("a")), "a_0".into());
    assert_eq!(ctx.get_or_push_new_binding(&Rc::from("a"), |_| called = true), "a_0".into());
    assert!(!called);
    ctx.pop_binding(&Rc::from("a"));
    assert_eq!(ctx.get_or_push_new_binding(&Rc::from("a"), |_| called = true), "a".into());
    assert!(!called);
    assert_eq!(ctx.push_new_binding(&Rc::from("a")), "a_1".into());
    ctx.pop_binding(&Rc::from("a"));
    ctx.pop_binding(&Rc::from("a"));
    assert_eq!(
        ctx.get_or_push_new_binding(&Rc::from("a"), |a| called = **a == *"a_2"),
        "a_2".into()
    );
    assert!(called);
    Ok(())
}

#[gtest]
fn test_binding_context_pushes_fresh_names() -> Result<()> {
    let mut ctx = BindingContext::new();
    assert_eq!(ctx.push_fresh_binding(None), "lt".into());
    assert_eq!(ctx.push_fresh_binding(None), "lt_0".into());
    assert_eq!(ctx.push_fresh_binding(None), "lt_1".into());
    let mut called = false;
    assert_eq!(ctx.get_or_push_new_binding(&Rc::from("lt"), |_| called = true), "lt".into());
    assert_eq!(ctx.get_or_push_new_binding(&Rc::from("lt_0"), |_| called = true), "lt_0".into());
    assert_eq!(ctx.get_or_push_new_binding(&Rc::from("lt_1"), |_| called = true), "lt_1".into());
    assert!(!called);
    Ok(())
}
