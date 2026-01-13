// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use std::collections::BTreeMap;
use std::sync::LazyLock;

use arc_anyhow::Result;
use itertools::Itertools;

use ffi_types::{FfiU8Slice, FfiU8SliceBox};
use ir::{self, make_ir_from_parts, Func, Identifier, Item, LifetimeId, LifetimeName, Record, IR};

/// Generates `IR` from a header containing `header_source`.
pub fn ir_from_cc(platform: multiplatform_testing::Platform, header_source: &str) -> Result<IR> {
    ir_from_cc_dependency(platform, header_source, "// empty header")
}

/// Prepends definitions for lifetime annotation macros to the code.
pub fn with_lifetime_macros(source: &str) -> String {
    let mut result = String::from(
        r#"
    #define $(l) [[clang::annotate_type("lifetime", #l)]]
    "#,
    );
    for l in 'a'..='z' {
        result.push_str(&format!("#define ${} $({})\n", l, l));
    }
    result.push_str("#define $static $(static)\n");
    result.push_str(source);
    result
}

pub fn with_full_lifetime_macros() -> String {
    // TODO: b/454627672 - it's not immediately clear why test_impl_clone_that_propagates_lifetime
    // fails when these additional macros are defined.
    let mut result = String::from(
        r#"
    #define $(l) [[clang::annotate_type("lifetime", #l)]]
    #define LIFETIME_PARAMS(...) [[clang::annotate("lifetime_params", __VA_ARGS__)]]
    #define MEMBER_LIFETIMES(...) [[clang::annotate("member_lifetimes", __VA_ARGS__)]]
    "#,
    );
    for l in 'a'..='z' {
        result.push_str(&format!("#define ${} $({})\n", l, l));
    }
    result.push_str("#define $static $(static)\n");
    result
}

/// Name of the current target used by `ir_from_cc` and `ir_from_cc_dependency`.
pub const TESTING_TARGET: &str = "//test:testing_target";
static TESTING_FEATURES: LazyLock<flagset::FlagSet<crubit_feature::CrubitFeature>> =
    LazyLock::new(|| {
        crubit_feature::CrubitFeature::Experimental
            | crubit_feature::CrubitFeature::Wrapper
            | crubit_feature::CrubitFeature::NonUnpinCtor
            | crubit_feature::CrubitFeature::Supported
    });

/// Update the IR to have common test-only items.
///
/// This provides one place to update the IR that affects both
/// `make_ir_from_items` and `ir_from_cc_dependency`.
fn update_test_ir(ir: &mut IR) {
    *ir.target_crubit_features_mut(&ir.current_target().clone()) = *TESTING_FEATURES;
    *ir.target_crubit_features_mut(&ir::BazelLabel(DEPENDENCY_TARGET.into())) = *TESTING_FEATURES;
}

/// Create a testing `IR` instance from given items, using mock values for other
/// fields.
pub fn make_ir_from_items(items: impl IntoIterator<Item = Item>) -> IR {
    let mut ir = make_ir_from_parts(
        items.into_iter().collect_vec(),
        /* public_headers= */ vec![],
        /* current_target= */ TESTING_TARGET.into(),
        /* top_level_item_ids= */ BTreeMap::new(),
        /* crate_root_path= */ None,
        /* crubit_features= */
        <BTreeMap<ir::BazelLabel, flagset::FlagSet<crubit_feature::CrubitFeature>>>::new(),
    );
    update_test_ir(&mut ir);
    ir
}

/// Target of the dependency used by `ir_from_cc_dependency`.
/// Needs to be kept in sync with `kDependencyTarget` in `json_from_cc.cc`.
pub const DEPENDENCY_TARGET: &str = "//test:dependency";

/// Generates `IR` from a header that depends on another header.
///
/// `header_source` of the header will be updated to contain the `#include` line
/// for the header with `dependency_header_source`. The name of the dependency
/// target is exposed as `DEPENDENCY_TARGET`.
pub fn ir_from_cc_dependency(
    platform: multiplatform_testing::Platform,
    header_source: &str,
    dependency_header_source: &str,
) -> Result<IR> {
    const DEPENDENCY_HEADER_NAME: &str = "test/dependency_header.h";

    unsafe extern "C" {
        fn json_from_cc_dependency(
            target_triple: FfiU8Slice,
            header_source: FfiU8Slice,
            dependency_header_source: FfiU8Slice,
        ) -> FfiU8SliceBox;
    }

    let header_source_with_include =
        format!("#include \"{}\"\n\n{}", DEPENDENCY_HEADER_NAME, header_source);
    let header_source_with_include_u8 = header_source_with_include.as_bytes();
    let dependency_header_source_u8 = dependency_header_source.as_bytes();
    let json_utf8 = unsafe {
        json_from_cc_dependency(
            FfiU8Slice::from_slice(platform.target_triple().as_ref()),
            FfiU8Slice::from_slice(header_source_with_include_u8),
            FfiU8Slice::from_slice(dependency_header_source_u8),
        )
        .into_boxed_slice()
    };
    let mut ir = ir::deserialize_ir(&*json_utf8)?;
    update_test_ir(&mut ir);
    Ok(ir)
}

/// Creates an identifier
pub fn ir_id(name: &str) -> Identifier {
    Identifier { identifier: name.into() }
}

/// Creates a simple `Item::Record` with a given name.
pub fn ir_record(platform: multiplatform_testing::Platform, name: &str) -> Record {
    let ir = ir_from_cc(platform, "struct REPLACEME final {};").unwrap();
    for item in ir.items() {
        if let Item::Record(record) = item {
            let mut record = (**record).clone();
            record.rs_name = Identifier { identifier: name.into() };
            record.cc_name = Identifier { identifier: name.into() };
            return record;
        }
    }
    panic!("Test IR doesn't contain a record");
}

pub fn retrieve_lifetime_param_id(names: &[LifetimeName], name: &str) -> LifetimeId {
    for param in names {
        if *param.name == *name {
            return param.id;
        }
    }
    panic!("Didn't find lifetime param with name {}", name);
}

/// Retrieves the function with the given name.
/// Panics if no such function could be found.
pub fn retrieve_func<'a>(ir: &'a IR, name: &str) -> &'a Func {
    for func in ir.functions() {
        if func.rs_name == ir::UnqualifiedIdentifier::Identifier(ir_id(name)) {
            return func;
        }
    }
    panic!("Didn't find function with name {}", name);
}

/// Retrieves the `Record` with the given name.
/// Panics if no such record could be found.
pub fn retrieve_record<'a>(ir: &'a IR, cc_name: &str) -> &'a Record {
    for record in ir.records() {
        if record.cc_name == cc_name {
            return record;
        }
    }
    panic!("Didn't find record with cc_name {}", cc_name);
}

#[cfg(test)]
mod tests {
    use super::*;
    use arc_anyhow::Result;
    use googletest::prelude::*;
    use ir::ItemId;
    use ir_matchers::assert_ir_matches;
    use multiplatform_testing::Platform;
    use quote::quote;

    #[gtest]
    fn test_features_ir_from_cc() -> Result<()> {
        assert_ir_matches!(
            ir_from_cc(multiplatform_testing::Platform::X86Linux, "")?,
            quote! {
                crubit_features: map! {
                    ...
                    BazelLabel(#TESTING_TARGET): SerializedCrubitFeatures(FlagSet(Supported|Wrapper|NonUnpinCtor|Experimental))
                    ...
                }
            }
        );
        Ok(())
    }
    #[gtest]
    fn test_features_ir_from_items() -> Result<()> {
        assert_ir_matches!(
            make_ir_from_items([]),
            quote! {
                crubit_features: map! {
                    ...
                    BazelLabel(#TESTING_TARGET): SerializedCrubitFeatures(FlagSet(Supported|Wrapper|NonUnpinCtor|Experimental))
                    ...
                }
            }
        );
        Ok(())
    }

    #[gtest]
    #[should_panic(expected = "Duplicate decl_id found in")]
    fn test_duplicate_decl_ids_err() {
        let mut r1 = ir_record(Platform::X86Linux, "R1");
        r1.id = ItemId::new_for_testing(42);
        let mut r2 = ir_record(Platform::X86Linux, "R2");
        r2.id = ItemId::new_for_testing(42);
        let _ = make_ir_from_items([r1.into(), r2.into()]);
    }
}
