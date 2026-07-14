// // Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use std::collections::BTreeMap;
use std::sync::LazyLock;

use arc_anyhow::Result;
use itertools::Itertools;

use ffi_types::{FfiU8Slice, FfiU8SliceBox};
use ir::{
    self, make_ir_from_parts, Func, Identifier, Item, LifetimeId, LifetimeName, Record,
    TypeWithDeclId, IR,
};

/// Generates `IR` from a header containing `header_source`.
pub fn ir_from_cc(platform: multiplatform_testing::Platform, header_source: &str) -> Result<IR> {
    ir_from_cc_dependency(platform, header_source, "// empty header", None, false)
}

/// Generates `IR` from a header containing `header_source` with source annotations.
pub fn ir_from_cc_annotated(
    platform: multiplatform_testing::Platform,
    header_source: &str,
) -> Result<IR> {
    ir_from_cc_dependency(platform, header_source, "// empty header", None, true)
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
    result.push_str("#define $unknown $(unknown)\n");
    result
}

/// Name of the current target used by `ir_from_cc` and `ir_from_cc_dependency`.
pub const TESTING_TARGET: &str = "//test:testing_target";
static TESTING_FEATURES: LazyLock<flagset::FlagSet<crubit_feature::CrubitFeature>> =
    LazyLock::new(|| {
        crubit_feature::CrubitFeature::Experimental
            | crubit_feature::CrubitFeature::Wrapper
            | crubit_feature::CrubitFeature::Types
            | crubit_feature::CrubitFeature::Supported
            | crubit_feature::CrubitFeature::TemplateInstantiation
    });

/// Update the IR to have common test-only items.
///
/// This provides one place to update the IR that affects both
/// `make_ir_from_items` and `ir_from_cc_dependency`.
fn update_test_ir(ir: &mut IR, extra_feature: Option<&str>) {
    *ir.target_crubit_features_mut(&ir.current_target().clone()) = *TESTING_FEATURES;
    *ir.target_crubit_features_mut(&ir::BazelLabel::from(DEPENDENCY_TARGET)) = *TESTING_FEATURES;
    if let Some(s) = extra_feature {
        let feature = crubit_feature::named_features(s.as_bytes()).unwrap();
        *ir.target_crubit_features_mut(&ir.current_target().clone()) |= feature;
        *ir.target_crubit_features_mut(&ir::BazelLabel::from(DEPENDENCY_TARGET)) |= feature;
    }
}

/// Create a testing `IR` instance from given items, using mock values for other
/// fields.
pub fn make_ir_from_items(items: impl IntoIterator<Item = Item>) -> IR {
    let mut ir = make_ir_from_parts(
        items.into_iter().collect_vec(),
        /* public_headers= */ vec![],
        /* current_target= */ TESTING_TARGET.into(),
        /* crate_root_path= */ None,
        /* crubit_features= */
        <BTreeMap<ir::BazelLabel, flagset::FlagSet<crubit_feature::CrubitFeature>>>::new(),
        /* reexported_namespaces= */ vec![],
    );
    update_test_ir(&mut ir, None);
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
    extra_feature: Option<&str>,
    kythe_annotations: bool,
) -> Result<IR> {
    ir_proto_from_cc_dependency(
        platform,
        header_source,
        dependency_header_source,
        extra_feature,
        kythe_annotations,
    )
}

/// Creates an identifier
pub fn ir_id(name: &str) -> Identifier {
    Identifier::new(name)
}

/// Creates a simple `Item::Record` with a given name.
pub fn ir_record(platform: multiplatform_testing::Platform, name: &str) -> Record {
    let ir = ir_from_cc(platform, "struct REPLACEME final {};").unwrap();
    for item in ir.items() {
        if let Item::Record(record) = item {
            let mut record = (**record).clone();
            record.set_rs_name(Identifier::new(name));
            record.set_cc_name(Identifier::new(name));
            return record;
        }
    }
    panic!("Test IR doesn't contain a record");
}

pub fn retrieve_lifetime_param_id(names: &[LifetimeName], name: &str) -> LifetimeId {
    for param in names {
        if param.name() == name {
            return param.id();
        }
    }
    panic!("Didn't find lifetime param with name {}", name);
}

/// Retrieves the function with the given name.
/// Panics if no such function could be found.
pub fn retrieve_func<'a>(ir: &'a IR, name: &str) -> &'a Func {
    for func in ir.functions() {
        if *func.rs_name() == ir::UnqualifiedIdentifier::Identifier(ir_id(name)) {
            return func;
        }
    }
    panic!("Didn't find function with name {}", name);
}

/// Retrieves the `Record` with the given name.
/// Panics if no such record could be found.
pub fn retrieve_record<'a>(ir: &'a IR, cc_name: &str) -> &'a Record {
    for record in ir.records() {
        if *record.cc_name() == cc_name {
            return record;
        }
    }
    panic!("Didn't find record with cc_name {}", cc_name);
}

/// Retrieves the `Record` underlying the type alias with the given name.
/// Panics if no such type alias could be found or it did not refer to a record.
pub fn retrieve_type_alias_record<'a>(ir: &'a IR, cc_name: &str) -> &'a Record {
    for type_alias in ir.type_aliases() {
        if type_alias.cc_name().as_str() == cc_name {
            let Some(item_id) = type_alias.underlying_type().decl_id() else {
                panic!("Type alias with cc_name {cc_name} has an underlying type with no ItemId");
            };
            let Some(Item::Record(record)) = ir.get_decl(item_id) else {
                panic!("Type alias with cc_name {cc_name} underlying type not found or is not a record");
            };
            return record;
        }
    }

    panic!("Didn't find type alias with cc_name {}", cc_name);
}

#[cfg(test)]
mod tests {
    use super::*;
    use arc_anyhow::Result;
    use crubit_feature::CrubitFeature;
    use googletest::{expect_eq, gtest};
    use ir::ItemId;
    use multiplatform_testing::Platform;

    #[gtest]
    fn test_features_ir_from_cc() -> Result<()> {
        let ir = ir_from_cc(multiplatform_testing::Platform::X86Linux, "")?;
        let enabled_features = ir.target_crubit_features(&ir::BazelLabel::from(TESTING_TARGET));
        expect_eq!(
            enabled_features,
            CrubitFeature::Experimental
                | CrubitFeature::Wrapper
                | CrubitFeature::Types
                | CrubitFeature::Supported
                | CrubitFeature::TemplateInstantiation
        );
        Ok(())
    }
    #[gtest]
    fn test_features_ir_from_items() -> Result<()> {
        let ir = make_ir_from_items([]);
        let enabled_features = ir.target_crubit_features(&ir::BazelLabel::from(TESTING_TARGET));
        expect_eq!(
            enabled_features,
            CrubitFeature::Experimental
                | CrubitFeature::Wrapper
                | CrubitFeature::Types
                | CrubitFeature::Supported
                | CrubitFeature::TemplateInstantiation
        );
        Ok(())
    }
    #[gtest]
    #[should_panic(expected = "Duplicate decl_id found in")]
    fn test_duplicate_decl_ids_err() {
        let mut r1 = ir_record(Platform::X86Linux, "R1");
        r1.set_id(ItemId::new_for_testing(42));
        let mut r2 = ir_record(Platform::X86Linux, "R2");
        r2.set_id(ItemId::new_for_testing(42));
        let _ = make_ir_from_items([r1.into(), r2.into()]);
    }
}
