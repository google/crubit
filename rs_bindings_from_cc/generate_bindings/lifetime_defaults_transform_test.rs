// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
#![cfg(test)]

use arc_anyhow::Result;
use googletest::prelude::*;
use ir_matchers::assert_ir_matches;
use ir_testing::{retrieve_func, retrieve_lifetime_param_id, with_full_lifetime_macros};
use lifetime_defaults_transform::lifetime_defaults_transform;
use multiplatform_ir_testing::ir_from_cc;
use quote::quote;

#[gtest]
fn test_fn_with_no_unbound_lifetimes_is_unchanged() -> Result<()> {
    let ir = ir_from_cc(
        &(with_full_lifetime_macros()
            + r#"
      LIFETIME_PARAMS("a")
      int& $a f(int& $a i1, int& $a i2);
      "#),
    )?;
    let dir = lifetime_defaults_transform(&ir);
    assert_eq!(dir, ir);
    Ok(())
}

#[gtest]
fn test_no_change_if_binder_is_already_added_to_function() -> Result<()> {
    let ir = ir_from_cc(
        &(with_full_lifetime_macros()
            + r#"
      int& $a f(int& $a i1, int& $a i2);
      "#),
    )?;
    let dir = lifetime_defaults_transform(&ir);
    let func = retrieve_func(&dir, "f");
    let a_id = retrieve_lifetime_param_id(&func.lifetime_params, "a");
    assert_ir_matches!(
        dir,
        quote! {
            Func {
                cc_name: "f",
                rs_name: "f", ...
                params: [
                    FuncParam {
                        ... identifier: "i1", ...
                    },
                    FuncParam {
                        ... identifier: "i2", ...
                    },
                ],
                lifetime_params: [LifetimeName { name: "a", id: LifetimeId (#a_id) ... }],
                ...
            }
        }
    );
    Ok(())
}

#[gtest]
fn test_unique_lifetime_ascribed_to_single_ref() -> Result<()> {
    let ir = ir_from_cc(
        &(with_full_lifetime_macros()
            + r#"
      void f(int& i1);
      "#),
    )?;
    let dir = lifetime_defaults_transform(&ir);
    let func = retrieve_func(&dir, "f");
    let i1_id = retrieve_lifetime_param_id(&func.lifetime_params, "i1");
    assert_ir_matches!(
        dir,
        quote! {
            Func {
                cc_name: "f",
                rs_name: "f", ...
                params: [
                    FuncParam {
                        type_: CcType { variant: Pointer (
                            PointerType { ... lifetime: Some(LifetimeId(#i1_id)) ... }), ... },
                        ... identifier: "i1", ...
                    }
                ],
                lifetime_params: [LifetimeName { name: "i1", id: LifetimeId (#i1_id) ... }],
                ...
            }
        }
    );
    Ok(())
}

#[gtest]
fn test_distinct_lifetime_returned_for_annotated_ref() -> Result<()> {
    let ir = ir_from_cc(
        &(with_full_lifetime_macros()
            + r#"
      int& $b f(int& $a i1);
      "#),
    )?;
    let dir = lifetime_defaults_transform(&ir);
    let func = retrieve_func(&dir, "f");
    let a_id = retrieve_lifetime_param_id(&func.lifetime_params, "a");
    let b_id = retrieve_lifetime_param_id(&func.lifetime_params, "b");
    assert_ir_matches!(
        dir,
        quote! {
            Func {
                cc_name: "f",
                rs_name: "f", ...
                return_type: CcType { variant: Pointer (
                    PointerType { ... lifetime: Some(LifetimeId(#b_id)) ... }), ... },
                params: [
                    FuncParam {
                        type_: CcType { variant: Pointer (
                            PointerType { ... lifetime: Some(LifetimeId(#a_id)) ... }), ... },
                        ... identifier: "i1", ...
                    }
                ],
                lifetime_params: [LifetimeName { name: "a", id: LifetimeId (#a_id) ... },
                    LifetimeName { name: "b", id: LifetimeId (#b_id) ... } ... ],
                ...
            }
        }
    );
    Ok(())
}

#[gtest]
fn test_unique_lifetime_returned_for_single_ref() -> Result<()> {
    let ir = ir_from_cc(
        &(with_full_lifetime_macros()
            + r#"
      int& f(int& i1);
      "#),
    )?;
    let dir = lifetime_defaults_transform(&ir);
    let func = retrieve_func(&dir, "f");
    let i1_id = retrieve_lifetime_param_id(&func.lifetime_params, "i1");
    assert_ir_matches!(
        dir,
        quote! {
            Func {
                cc_name: "f",
                rs_name: "f", ...
                return_type: CcType { variant: Pointer (
                    PointerType { ... lifetime: Some(LifetimeId(#i1_id)) ... }), ... },
                params: [
                    FuncParam {
                        type_: CcType { variant: Pointer (
                            PointerType { ... lifetime: Some(LifetimeId(#i1_id)) ... }), ... },
                        ... identifier: "i1", ...
                    }
                ],
                lifetime_params: [LifetimeName { name: "i1", id: LifetimeId (#i1_id) ... }],
                ...
            }
        }
    );
    Ok(())
}

#[gtest]
fn test_no_lifetime_returned_for_distinct_ref_parameters() -> Result<()> {
    let ir = ir_from_cc(
        &(with_full_lifetime_macros()
            + r#"
      int& f(int& i1, int& i2);
      "#),
    )?;
    let dir = lifetime_defaults_transform(&ir);
    let func = retrieve_func(&dir, "f");
    let i1_id = retrieve_lifetime_param_id(&func.lifetime_params, "i1");
    let i2_id = retrieve_lifetime_param_id(&func.lifetime_params, "i2");
    assert_ir_matches!(
        dir,
        quote! {
            Func {
                cc_name: "f",
                rs_name: "f", ...
                return_type: CcType { variant: Pointer (
                    PointerType { ... lifetime: None ... }), ... },
                params: [
                    FuncParam {
                        type_: CcType { variant: Pointer (
                            PointerType { ... lifetime: Some(LifetimeId(#i1_id)) ... }), ... },
                        ... identifier: "i1", ...
                    },
                    FuncParam {
                        type_: CcType { variant: Pointer (
                            PointerType { ... lifetime: Some(LifetimeId(#i2_id)) ... }), ... },
                        ... identifier: "i2", ...
                    },
                ],
                lifetime_params: [
                    LifetimeName { name: "i1", id: LifetimeId (#i1_id) ... },
                    LifetimeName { name: "i2", id: LifetimeId (#i2_id) ... },
                ],
                ...
            }
        }
    );
    Ok(())
}

#[gtest]
fn test_no_lifetime_assigned_for_nullary_fn() -> Result<()> {
    let ir = ir_from_cc(
        &(with_full_lifetime_macros()
            + r#"
      int& f();
      "#),
    )?;
    let dir = lifetime_defaults_transform(&ir);
    assert_ir_matches!(
        dir,
        quote! {
            Func {
                cc_name: "f",
                rs_name: "f", ...
                return_type: CcType { variant: Pointer (
                    PointerType { ... lifetime: None ... }), ... },
                params: [],
                lifetime_params: [],
                ...
            }
        }
    );
    Ok(())
}
