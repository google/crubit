// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::{expect_eq, expect_ne, gtest};
use ir::{
    ir_generated, proto_to_ir, rs_imported_crate_name, BazelLabel, CcTypeVariant, FfiMap, FfiRc,
    FfiString, GenericItem, HeaderName, ItemId, Primitive, TemplateSpecializationKind,
    TraitImplPolarity, TreeIR, UnqualifiedIdentifier,
};
use std::collections::hash_map::DefaultHasher;
use std::collections::BTreeMap;
use std::hash::{BuildHasher, Hash, Hasher};

fn hash_of<T: Hash>(val: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    val.hash(&mut hasher);
    hasher.finish()
}

#[gtest]
fn test_ffi_rc_clone_and_drop() {
    let root = FfiRc::new(FfiString::from_str("shared_payload"));
    let mut clones = Vec::with_capacity(256);
    for _ in 0..256 {
        clones.push(root.clone());
    }
    for item in &clones {
        expect_eq!(item.as_str(), "shared_payload");
    }
    drop(clones);
    expect_eq!(root.as_str(), "shared_payload");
}

#[gtest]
fn test_ffi_rc_cow_and_null_default_equality() {
    let empty_null: FfiRc<FfiString> = FfiRc::EMPTY;
    let explicit_empty = FfiRc::new(FfiString::EMPTY);
    expect_eq!(empty_null, explicit_empty);
    expect_eq!(hash_of(&empty_null), hash_of(&explicit_empty));

    let shared = FfiRc::new(FfiString::from_str("original"));
    let mut copy = shared.clone();
    *copy.make_mut() = FfiString::from_str("mutated");
    expect_eq!(shared.as_str(), "original");
    expect_eq!(copy.as_str(), "mutated");
}

#[gtest]
fn test_ffi_map_sorted_order_and_equality() {
    let mut map_a: FfiMap<FfiString, FfiString> = FfiMap::new();
    map_a.insert("zeta".into(), "3".into());
    map_a.insert("alpha".into(), "1".into());
    map_a.insert("mu".into(), "2".into());

    let mut map_b: FfiMap<FfiString, FfiString> = FfiMap::new();
    map_b.insert("alpha".into(), "0".into());
    map_b.insert("mu".into(), "2".into());
    map_b.insert("zeta".into(), "3".into());
    map_b.insert("alpha".into(), "1".into());

    let keys: Vec<&str> = map_a.iter().map(|(k, _)| k.as_str()).collect();
    expect_eq!(keys, vec!["alpha", "mu", "zeta"]);
    expect_eq!(map_a.get("alpha").map(FfiString::as_str), Some("1"));
    expect_eq!(map_a.get("mu").map(FfiString::as_str), Some("2"));
    expect_eq!(map_a.get("zeta").map(FfiString::as_str), Some("3"));
    expect_eq!(map_a.get("missing"), None);

    expect_eq!(map_a, map_b);
    expect_eq!(hash_of(&map_a), hash_of(&map_b));
}

#[gtest]
fn test_identifier_debug_print() {
    let identifier = ir_generated::proto!(ir_generated::Identifier { identifier: "hello" });
    assert_eq!(format!("{identifier:?}"), "\"hello\"");
}

#[gtest]
fn test_unqualified_identifier_debug_print() {
    let identifier = ir_generated::proto!(ir_generated::Identifier { identifier: "hello" });
    assert_eq!(format!("{:?}", UnqualifiedIdentifier::Identifier(identifier)), "\"hello\"");
    assert_eq!(format!("{:?}", UnqualifiedIdentifier::Constructor), "Constructor");
    assert_eq!(format!("{:?}", UnqualifiedIdentifier::Destructor), "Destructor");
}

#[gtest]
fn test_used_headers() {
    let proto = ir_generated::proto!(ir_generated::IR {
        public_headers: [__ { name: "foo/bar.h" }],
        current_target: "//foo:bar",
    });
    let ir = proto_to_ir(&proto).unwrap();
    let expected = TreeIR {
        public_headers: vec![HeaderName::from_str("foo/bar.h")],
        current_target: "//foo:bar".into(),
        crate_root_path: None,
        crubit_features: Default::default(),
        crate_names: BTreeMap::new(),
        unstable_rust_features: vec![],
        reexported_namespaces: vec![],
        top_level_items: BTreeMap::new(),
    };
    assert_eq!(ir.tree_ir(), &expected);
}

#[gtest]
fn test_empty_crate_root_path() {
    let proto = ir_generated::proto!(ir_generated::IR { current_target: "//foo:bar" });
    let ir = proto_to_ir(&proto).unwrap();
    assert_eq!(ir.crate_root_path(), None);
}

#[gtest]
fn test_crate_root_path() {
    let proto = ir_generated::proto!(ir_generated::IR {
        crate_root_path: "__cc_template_instantiations_rs_api",
        current_target: "//foo:bar",
    });
    let ir = proto_to_ir(&proto).unwrap();
    assert_eq!(ir.crate_root_path(), Some("__cc_template_instantiations_rs_api"));
}

#[gtest]
fn test_bazel_label_target() {
    let label: BazelLabel = "//foo:bar".into();
    assert_eq!(label.target_name(), "bar");
}

#[gtest]
fn test_bazel_label_is_cc_std() {
    // Internal HEAD layout (and also the target label used in Chromium's repository
    // layout).
    assert!(BazelLabel::from("//support/cc_std:cc_std").is_cc_std());
    // Internal toolchain stable release snapshot.
    assert!(BazelLabel::from("//third_party/crosstool/rust/stable/crubit/support/cc_std:cc_std")
        .is_cc_std());
    // Internal toolchain stable_nonprod release snapshot.
    assert!(BazelLabel::from(
        "//third_party/crosstool/rust/stable_nonprod/crubit/support/cc_std:cc_std"
    )
    .is_cc_std());
    // Open-source Crubit repository layout (GitHub: google/crubit, rewritten by Copybara).
    assert!(BazelLabel::from("//support/cc_std:cc_std").is_cc_std());
    // Open-source Crubit repository layout using Bazel's shorthand package syntax.
    assert!(BazelLabel::from("//support/cc_std").is_cc_std());

    // Negative cases:
    // A different target within the cc_std package.
    assert!(!BazelLabel::from("//support/cc_std:other").is_cc_std());
    // An unrelated target.
    assert!(!BazelLabel::from("//rs_bindings_from_cc:ir").is_cc_std());
}

#[gtest]
fn test_bazel_label_target_dotless() {
    let label: BazelLabel = "//foo".into();
    assert_eq!(label.target_name(), "foo");
}

#[gtest]
fn test_bazel_label_implicit_target_equals_explicit_target() {
    let implicit: BazelLabel = "//foo".into();
    let explicit: BazelLabel = "//foo:foo".into();
    assert_eq!(implicit, explicit);
}

#[gtest]
fn test_bazel_label_canonical_main_repo_equals_shorthand_repo() {
    let canonical: BazelLabel = "@@//foo:bar".into();
    let shorthand: BazelLabel = "//foo:bar".into();
    assert_eq!(canonical, shorthand);
    assert_eq!(canonical.as_str(), "//foo:bar");
    assert_eq!(canonical.convert_to_cc_identifier(), shorthand.convert_to_cc_identifier());
}

#[gtest]
fn test_bazel_label_dotless_slashless() {
    let label: BazelLabel = "foo".into();
    assert_eq!(label.target_name(), "foo");
}

/// These are not labels, but there is an unambiguous interpretation of
/// what their target should be that lets us keep going.
#[gtest]
fn test_bazel_label_empty_target() {
    for s in ["foo:", "foo/", ""] {
        let label: BazelLabel = s.into();
        assert_eq!(label.target_name(), "", "label={s:?}");
    }
}

#[gtest]
fn test_bazel_label_escape_target_name_with_relative_label() {
    let label: BazelLabel = "foo".into();
    assert_eq!(label.target_name_escaped(), "foo");
}

#[gtest]
fn test_bazel_label_escape_target_name_with_invalid_characters() {
    let label: BazelLabel = "//:!./%-@^#$&()*-+,;<=>?[]{|}~".into();
    assert_eq!(label.target_name_escaped(), "___________________________");
}

#[gtest]
fn test_bazel_label_escape_target_name_core() {
    let label: BazelLabel = "//foo~:core".into();
    assert_eq!(label.target_name_escaped(), "core_foo_");
}

#[gtest]
fn test_bazel_label_escape_target_name_with_no_target_name() {
    let label: BazelLabel = "//foo/bar~".into();
    assert_eq!(label.target_name_escaped(), "bar_");
}

#[gtest]
fn test_bazel_label_escape_target_name_with_no_package_name() {
    let label: BazelLabel = "//:foo~".into();
    assert_eq!(label.target_name_escaped(), "foo_");
}

#[gtest]
fn test_bazel_label_escape_target_name_core_with_no_package_name_with_no_target_name() {
    let label: BazelLabel = "core".into();
    assert_eq!(label.target_name_escaped(), "core_");
}

#[gtest]
fn test_bazel_label_escape_target_name_starting_with_digit() {
    let label: BazelLabel = "12345".into();
    assert_eq!(label.target_name_escaped(), "n12345");
}

#[gtest]
fn test_bazel_to_cc_identifier_empty() {
    assert_eq!(BazelLabel::from("").convert_to_cc_identifier(), "_");
}

#[gtest]
fn test_bazel_to_cc_identifier_alphanumeric_not_transformed() {
    assert_eq!(BazelLabel::from("abc").convert_to_cc_identifier(), "_abc");
    assert_eq!(BazelLabel::from("foo123").convert_to_cc_identifier(), "_foo123");
    assert_eq!(BazelLabel::from("123foo").convert_to_cc_identifier(), "_123foo");
}

#[gtest]
fn test_bazel_to_cc_identifier_simple_targets() {
    assert_eq!(
        BazelLabel::from("//foo/bar:baz_abc").convert_to_cc_identifier(),
        "__2f_2ffoo_2fbar_3abaz_5fabc"
    );
}

#[gtest]
fn test_bazel_to_cc_identifier_conflict() {
    assert_ne!(
        BazelLabel::from("//foo_bar:baz").convert_to_cc_identifier(),
        BazelLabel::from("//foo/bar:baz").convert_to_cc_identifier()
    );
}

#[gtest]
fn test_make_ir_happy_path() {
    let proto = ir_generated::proto!(ir_generated::IR {
        current_target: "//foo:bar",
        top_level_items: [(
            "//foo:bar",
            [__ {
                namespace_decl: __ {
                    cc_name: __ { identifier: "nsA" },
                    rs_name: __ { identifier: "nsA" },
                    unique_name: "nsA",
                    id: 1u64,
                    canonical_namespace_id: 1u64,
                    owning_target: "//foo:bar",
                    is_inline: false,
                    must_bind: false,
                    children: [__ {
                        namespace_decl: __ {
                            cc_name: __ { identifier: "nsB" },
                            rs_name: __ { identifier: "nsB" },
                            unique_name: "nsB",
                            id: 2u64,
                            canonical_namespace_id: 2u64,
                            owning_target: "//foo:bar",
                            is_inline: false,
                            must_bind: false,
                            children: [__ {
                                comment: __ { text: "hello", id: 3u64, must_bind: false }
                            }]
                        }
                    }]
                }
            }]
        )]
    });
    let ir = proto_to_ir(&proto).unwrap();

    let ids: Vec<_> = ir.items().map(|item| item.id().as_u64()).collect();
    assert_eq!(ids, vec![1, 2, 3]);
}

#[gtest]
fn test_make_ir_redeclarations() {
    let proto = ir_generated::proto!(ir_generated::IR {
        current_target: "//foo:bar",
        top_level_items: [(
            "//foo:bar",
            [
                __ {
                    namespace_decl: __ {
                        cc_name: __ { identifier: "ns1" },
                        rs_name: __ { identifier: "ns1" },
                        unique_name: "ns1",
                        id: 100u64,
                        canonical_namespace_id: 100u64,
                        owning_target: "//foo:bar",
                        is_inline: false,
                        must_bind: false,
                        children: [__ {
                            comment: __ { text: "hello", id: 200u64, must_bind: false }
                        }]
                    }
                },
                __ {
                    namespace_decl: __ {
                        cc_name: __ { identifier: "ns1" },
                        rs_name: __ { identifier: "ns1" },
                        unique_name: "ns1",
                        id: 101u64,
                        canonical_namespace_id: 100u64,
                        owning_target: "//foo:bar",
                        is_inline: false,
                        must_bind: false,
                        children: [__ {
                            comment: __ { text: "hello", id: 200u64, must_bind: false }
                        }]
                    }
                }
            ]
        )]
    });
    let ir = proto_to_ir(&proto).unwrap();

    let comment_items: Vec<_> = ir.items().filter(|item| item.id() == ItemId(200)).collect();

    assert_eq!(comment_items.len(), 1);
}

#[gtest]
fn test_proto_crate_names() {
    let proto = ir_generated::proto!(ir_generated::IR {
        current_target: "//foo:bar",
        crate_names: [("//dep:target", "custom_crate")]
    });
    let ir = proto_to_ir(&proto).unwrap();
    assert_eq!(
        ir.crate_name(&"//dep:target".into()).map(|i| i.to_string()),
        Some("custom_crate".to_string())
    );
}

#[gtest]
fn test_proto_crate_names_invalid_ident() {
    let proto = ir_generated::proto!(ir_generated::IR {
        current_target: "//foo:bar",
        crate_names: [("//dep:target", "invalid*crate")]
    });
    let result = proto_to_ir(&proto);
    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_string();
    assert!(
        err_msg.contains("Invalid crate name identifier: \"invalid*crate\""),
        "error: {}",
        err_msg
    );
}

#[gtest]
fn test_rs_imported_crate_name_with_custom_name() {
    let proto = ir_generated::proto!(ir_generated::IR {
        current_target: "//foo:bar",
        crate_names: [("//dep:target", "custom_crate")],
    });
    let ir = proto_to_ir(&proto).unwrap();
    let crate_ident = rs_imported_crate_name(&"//dep:target".into(), &ir).unwrap();
    assert_eq!(crate_ident.to_string(), "custom_crate");
}

#[gtest]
fn test_rs_imported_crate_name_without_custom_name() {
    let proto = ir_generated::proto!(ir_generated::IR { current_target: "//foo:bar" });
    let ir = proto_to_ir(&proto).unwrap();
    let crate_ident = rs_imported_crate_name(&"//dep:target".into(), &ir).unwrap();
    assert_eq!(crate_ident.to_string(), "target");
}

#[gtest]
fn test_record_is_string_view() {
    for (defining_target, expected) in [
        // Internal HEAD layout (and also the target label used in Chromium's repository
        // layout).
        ("//support/cc_std:cc_std", true),
        // Internal toolchain stable release snapshot.
        ("//third_party/crosstool/rust/stable/crubit/support/cc_std:cc_std", true),
        // Internal toolchain stable_nonprod release snapshot.
        ("//third_party/crosstool/rust/stable_nonprod/crubit/support/cc_std:cc_std", true),
        // Open-source Crubit repository layout (GitHub: google/crubit, rewritten by Copybara).
        ("//support/cc_std:cc_std", true),
        // Open-source Crubit repository layout using Bazel's shorthand package syntax.
        ("//support/cc_std", true),
        // Negative case: a different target within the cc_std package.
        ("//support/cc_std:other", false),
        // Negative case: an unrelated target.
        ("//rs_bindings_from_cc:ir", false),
    ] {
        let record = ir_generated::proto!(ir_generated::Record {
            rs_name: __ { identifier: "string_view" },
            trait_derives: __ {
                clone: TraitImplPolarity::None,
                copy: TraitImplPolarity::None,
                debug: TraitImplPolarity::None,
                send: TraitImplPolarity::None,
                sync: TraitImplPolarity::None,
            },
            template_specialization: __ {
                defining_target: defining_target,
                kind: TemplateSpecializationKind::StdStringView,
            },
        });
        assert_eq!(record.is_string_view(), expected, "target={defining_target}");
    }
}

#[gtest]
fn test_constant_array_type_validate_num_elements() {
    let make_proto = |num_elements: usize| {
        ir_generated::proto!(ir_generated::IR {
            current_target: "//foo:bar",
            top_level_items: [(
                "//foo:bar",
                [__ {
                    type_alias: __ {
                        cc_name: __ { identifier: "Arr" },
                        rs_name: __ { identifier: "Arr" },
                        owning_target: "//foo:bar",
                        id: 1u64,
                        underlying_type: __ {
                            variant: __ {
                                constant_array: __ {
                                    num_elements: num_elements,
                                    element_type: __ { variant: __ { primitive: Primitive::Int } },
                                },
                            },
                        },
                    },
                }]
            )],
        })
    };

    let err = proto_to_ir(&make_proto(0)).unwrap_err();
    assert!(
        err.to_string().contains("ConstantArrayType has non-positive num_elements"),
        "unexpected error for num_elements=0: {err}"
    );

    let valid_ir = proto_to_ir(&make_proto(4)).unwrap();
    let ta = valid_ir.type_aliases().next().unwrap();
    let CcTypeVariant::ConstantArray(arr) = ta.underlying_type().variant() else {
        panic!("expected ConstantArray, got {:?}", ta.underlying_type());
    };
    assert_eq!(arr.num_elements(), 4);
}

#[gtest]
fn test_item_hash_by_id_and_partial_eq() {
    let r1 = ir_generated::proto!(ir_generated::Record {
        id: 42u64,
        rs_name: __ { identifier: "Foo" },
        cc_name: __ { identifier: "Foo" },
        children: [__ { comment: __ { text: "child comment", id: 43u64, must_bind: false } }],
    });
    let r2 = ir_generated::proto!(ir_generated::Record {
        id: 42u64,
        rs_name: __ { identifier: "Bar" },
        cc_name: __ { identifier: "Bar" },
    });
    let r3 = ir_generated::proto!(ir_generated::Record {
        id: 99u64,
        rs_name: __ { identifier: "Foo" },
        cc_name: __ { identifier: "Foo" },
        children: [__ { comment: __ { text: "child comment", id: 43u64, must_bind: false } }],
    });

    let hasher = std::collections::hash_map::RandomState::new();
    expect_eq!(hasher.hash_one(&r1), hasher.hash_one(&r2));
    expect_ne!(hasher.hash_one(&r1), hasher.hash_one(&r3));
    expect_eq!(r1, r1.clone());
    expect_ne!(r1, r2);
    expect_ne!(r1, r3);
}
