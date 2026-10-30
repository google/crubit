// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! Supporting types to read and display Crubit feature flags
//! (crubit.rs-features)
use serde::Deserialize;

flagset::flags! {
    /// The Crubit feature flags enum.
    ///
    /// Every feature flag is also associated with a `short_name`, which is used to serialize and
    /// deserialize it, and an `aspect_hint`, which is presented to users in error messages. If
    /// a function requires a feature flag, the users will be told to add the corresponding
    /// `aspect_hint`.
    pub enum CrubitFeature : u64 {
        Supported,

        /// Enable support for forward-declared types: b/489120100
        /// We intend for this feature to be turned on by default.
        ForwardDeclarations,

        /// Enable support for types, but not necessarily functions.
        /// This is automatically enabled by `Supported`.
        Types,

        /// Experimental is never *set* without also setting Supported, but we allow it to be
        /// *required* without also requiring Supported, so that error messages can be more direct.
        Experimental,

        /// Use ergonomic lifetime defaults when interpreting lifetime annotations.
        AssumeLifetimes,

        /// Unconditionally assume that the `this` pointer is a reference, even in the absence of
        /// a lifetime annotation or other justification.
        AssumeThisLifetimes,

        /// Disable AssumeLifetimes (useful for :experimental).
        NoAssumeLifetimes,

        /// Mark C++ types with `[[gsl::Pointer]]` as unsafe.
        UnsafeView,

        /// Generate bindings using the protobuf IR.

        /// C++ default constructors are checked to fully initialize all public fields.
        CheckDefaultInitialized,

        /// Prepend `::` to `cpp_type` in annotations.
        LeadingColonsForCppType,

        /// Generate bindings for (non-Crubit special-cased) template instances.
        TemplateInstantiation,

        /// Disable TemplateInstantiation.
        NoTemplateInstantiation,

        /// Emit `rs_std::Tuple` everywhere instead of C++ `std::tuple`.
        LayoutCompatTuple,

        /// Emit layout-compatible `std::string` everywhere instead of `string_wrapper`.
        LayoutCompatString,


        /// Always specialize generics in cpp_api_from_rust, instead of doing composable bridging
        /// when possible.
        AlwaysSpecializeGenericsInCppApiFromRust,

        /// Enable custom Debug impls for records.
        RecordImplDebug,

        /// Converts `impl Ctor<Output=T>` parameters to plain `T` instead of `T&&` in the C++ API.
        CtorPlainValues,

        /// Reserve standard macros like stdin, stdout, stderr as keywords.
        ReserveStandardMacros,

        /// Directly generate (some) accessors instead of using thunks.
        ThunklessAccessors,

        /// Enable object-oriented casting between base and derived classes.
        OoCasting,

        /// Emit `*View` and `*Mut` directly for C++ proto references (`const Proto&` and `Proto&`).
        ProtoReferences,

        /// Generate bindings for generic types (templates).
        Generics,

        /// Map `std::unique_ptr` and `std::shared_ptr` by nullability (crubit.rs-nullable): a smart
        /// pointer known to be non-null (`absl_nonnull`, or unannotated under
        /// `ABSL_POINTERS_DEFAULT_NONNULL`) is a bare Rust `Ptr`, and any other is
        /// `cc_std::std::Nullable<Ptr>`. In the other direction, a bare Rust `Ptr` is spelled
        /// `Ptr crubit_nonnull` in C++.
        NonnullSmartPointers,

        /// Generate `::cxx::ExternType` implementations for records.
        /// This is automatically enabled by `Supported` and `Types`.
        ImplCxxExternType,

        /// Disable ImplCxxExternType.
        NoImplCxxExternType,

        /// Generate C++ bindings for Rust enums with one nested struct per variant, built on
        /// `support/rs_std/enum.h` (crubit.rs-enum-api-v2). Only affects cpp_api_from_rust.
        EnumApiV2,
        /// Let rustc infer `Send` and `Sync` for C++ records from their fields, instead of
        /// emitting `impl !Send` / `impl !Sync` for records without an explicit annotation.
        /// Opaque byte blobs which hold C++ data are marked as neither `Send` nor `Sync`.
        InferSendSync,

        /// Emit C++ references (`T const&` / `T&`) instead of pointers (`T const*` / `T*`) for
        /// `Iterator::Item` and `IntoIterator::Item` associated types. Only affects
        /// cpp_api_from_rust.
        IteratorItemReferences,

        /// Generate bindings for operators that are hidden friends of base classes (e.g. defined by
        /// CRTP mixins), as if they were hidden friends of the derived class.
        InheritedFriendOperators,

        /// Treat parameter types with unsafe template arguments as unsafe.
        UnsafeTemplateParams,

        /// Return non-C++-movable types by value as `rs::Movable<T>` (the same wrapper already
        /// used for by-value parameters), instead of as a bare `T` which can only be received via
        /// guaranteed copy elision. `rs::Movable<T>` implicitly converts to `T`, so
        /// `T x = f();` keeps compiling, and `g(f())` works when `g` takes `rs::Movable<T>`.
        /// Only affects cpp_api_from_rust.
        MovableReturnValues,

        /// Implement `Eq` and `Ord` for C++ types whose `operator<=>` returns
        /// `std::strong_ordering`. Only affects rust_api_from_cpp.
        StrongOrderingTraits,

        /// Generate Rust bindings for constant-size C-style arrays (`T[N]`) as `[T; N]` when
        /// layout-compatible, instead of an opaque byte blob (b/575922365).
        ConstantArrays,

        /// Generate Rust constants for object-like C++ macros which expand to simple constant
        /// expressions, e.g. `#define FOO 4` or `#define BAR (FOO + 3)`.
        MacroConstants,
    }
}

impl CrubitFeature {
    /// The name of this feature.
    ///
    /// This is used for serialization/deserialization: an aspect hint maps to a
    /// list of short names, which are passed to Crubit to enable the
    /// corresponding feature bits.
    pub fn short_name(&self) -> &'static str {
        match self {
            Self::Supported => "supported",
            Self::ForwardDeclarations => "forward_declare",
            Self::Types => "types",
            Self::Experimental => "experimental",
            Self::AssumeLifetimes => "assume_lifetimes",
            Self::AssumeThisLifetimes => "assume_this_lifetimes",
            Self::NoAssumeLifetimes => "no_assume_lifetimes",
            Self::NoTemplateInstantiation => "no_template_instantiation",
            Self::UnsafeView => "unsafe_view",
            Self::CheckDefaultInitialized => "check_default_initialized",
            Self::LeadingColonsForCppType => "leading_colons_for_cpp_type",
            Self::TemplateInstantiation => "template_instantiation",
            Self::LayoutCompatTuple => "layout_compat_tuple",
            Self::LayoutCompatString => "layout_compat_string",
            Self::AlwaysSpecializeGenericsInCppApiFromRust => {
                "always_specialize_generics_in_cpp_api_from_rust"
            }
            Self::RecordImplDebug => "record_impl_debug",
            Self::CtorPlainValues => "ctor_plain_values",
            Self::ReserveStandardMacros => "reserve_standard_macros",
            Self::ThunklessAccessors => "thunkless_accessors",
            Self::OoCasting => "oo_casting",
            Self::ProtoReferences => "proto_references",
            Self::Generics => "generics",
            Self::NonnullSmartPointers => "nonnull_smart_pointers",
            Self::ImplCxxExternType => "impl_cxx_extern_type",
            Self::NoImplCxxExternType => "no_impl_cxx_extern_type",
            Self::EnumApiV2 => "enum_api_v2",
            Self::InferSendSync => "infer_send_sync",
            Self::IteratorItemReferences => "iterator_item_references",
            Self::InheritedFriendOperators => "inherited_friend_operators",
            Self::UnsafeTemplateParams => "unsafe_template_params",
            Self::MovableReturnValues => "movable_return_values",
            Self::StrongOrderingTraits => "strong_ordering_traits",
            Self::ConstantArrays => "constant_arrays",
            Self::MacroConstants => "macro_constants",
        }
    }

    /// The aspect hint required to enable this feature.
    ///
    /// This should be a label in features/BUILD.
    pub fn aspect_hint(&self) -> &'static str {
        match self {
            Self::Supported => "//features:supported",
            Self::ForwardDeclarations => "//features:forward_declare",
            Self::Types => "//features:types",
            Self::Experimental => "//features:experimental",
            Self::AssumeLifetimes => "//features:assume_lifetimes",
            Self::AssumeThisLifetimes => "//features:assume_this_lifetimes",
            Self::NoAssumeLifetimes => "//features:no_assume_lifetimes",
            Self::NoTemplateInstantiation => {
                "//features:no_template_instantiation"
            }
            Self::UnsafeView => "//features:unsafe_view",
            Self::CheckDefaultInitialized => {
                "//features:check_default_initialized"
            }
            Self::LeadingColonsForCppType => {
                "//features:leading_colons_for_cpp_type"
            }
            Self::TemplateInstantiation => "//features:template_instantiation",
            Self::LayoutCompatTuple => "//features:layout_compat_tuple",
            Self::LayoutCompatString => "//features:layout_compat_string",
            Self::AlwaysSpecializeGenericsInCppApiFromRust => {
                "//features:always_specialize_generics_in_cpp_api_from_rust"
            }
            Self::RecordImplDebug => "//features:record_impl_debug",
            Self::CtorPlainValues => "//features:ctor_plain_values",
            Self::ReserveStandardMacros => "//features:reserve_standard_macros",
            Self::ThunklessAccessors => "//features:thunkless_accessors",
            Self::OoCasting => "//features:oo_casting",
            Self::ProtoReferences => "//features:proto_references",
            Self::Generics => "//features:generics",
            Self::NonnullSmartPointers => "//features:nonnull_smart_pointers",
            Self::ImplCxxExternType => "//features:impl_cxx_extern_type",
            Self::NoImplCxxExternType => "//features:no_impl_cxx_extern_type",
            Self::EnumApiV2 => "//features:enum_api_v2",
            Self::InferSendSync => "//features:infer_send_sync",
            Self::IteratorItemReferences => {
                "//features:iterator_item_references"
            }
            Self::InheritedFriendOperators => {
                "//features:inherited_friend_operators"
            }
            Self::UnsafeTemplateParams => "//features:unsafe_template_params",
            Self::MovableReturnValues => "//features:movable_return_values",
            Self::StrongOrderingTraits => "//features:strong_ordering_traits",
            Self::ConstantArrays => "//features:constant_arrays",
            Self::MacroConstants => "//features:macro_constants",
        }
    }
}

/// Returns the set of features named by this short name.
pub fn named_features(name: &[u8]) -> Option<flagset::FlagSet<CrubitFeature>> {
    let features = match name {
        // LINT.IfChange
        b"all" => {
            flagset::FlagSet::<CrubitFeature>::full()
                - CrubitFeature::NoAssumeLifetimes
                - CrubitFeature::NoTemplateInstantiation
                - CrubitFeature::NoImplCxxExternType
                - CrubitFeature::LayoutCompatTuple
                - CrubitFeature::LayoutCompatString
                - CrubitFeature::AlwaysSpecializeGenericsInCppApiFromRust
                - CrubitFeature::OoCasting
                - CrubitFeature::ProtoReferences
                - CrubitFeature::ForwardDeclarations
                - CrubitFeature::Generics
                - CrubitFeature::NonnullSmartPointers
                // Out of `all` until existing `E::MakeV` callers are migrated.
                - CrubitFeature::EnumApiV2
                // Standalone opt-in: changes the soundness defaults for Send/Sync.
                - CrubitFeature::InferSendSync
                // Out of `all` until callers expecting pointers from iterators are migrated.
                - CrubitFeature::IteratorItemReferences
                // Standalone opt-in: treats parameter types with unsafe template arguments as unsafe.
                - CrubitFeature::UnsafeTemplateParams
                // Out of `all` until callers which use a bare `T` return value as a temporary
                // (e.g. `f().method()`) are migrated to `rs::Movable<T>`.
                - CrubitFeature::MovableReturnValues
        }
        // `supported` automatically implies `types` and `impl_cxx_extern_type`.
        b"supported" => {
            CrubitFeature::Supported | CrubitFeature::Types | CrubitFeature::ImplCxxExternType
        }
        b"forward_declare" => CrubitFeature::ForwardDeclarations.into(),
        b"types" => CrubitFeature::Types | CrubitFeature::ImplCxxExternType,
        b"experimental" => CrubitFeature::Experimental.into(),
        b"assume_lifetimes" => CrubitFeature::AssumeLifetimes.into(),
        b"assume_this_lifetimes" => CrubitFeature::AssumeThisLifetimes.into(),
        b"no_assume_lifetimes" => CrubitFeature::NoAssumeLifetimes.into(),
        b"no_template_instantiation" => CrubitFeature::NoTemplateInstantiation.into(),
        b"unsafe_view" => CrubitFeature::UnsafeView.into(),
        b"check_default_initialized" => CrubitFeature::CheckDefaultInitialized.into(),
        b"leading_colons_for_cpp_type" => CrubitFeature::LeadingColonsForCppType.into(),
        b"template_instantiation" => CrubitFeature::TemplateInstantiation.into(),
        b"layout_compat_tuple" => CrubitFeature::LayoutCompatTuple.into(),
        b"layout_compat_string" => CrubitFeature::LayoutCompatString.into(),
        b"always_specialize_generics_in_cpp_api_from_rust" => {
            CrubitFeature::AlwaysSpecializeGenericsInCppApiFromRust.into()
        }
        b"record_impl_debug" => CrubitFeature::RecordImplDebug.into(),
        b"ctor_plain_values" => CrubitFeature::CtorPlainValues.into(),
        b"reserve_standard_macros" => CrubitFeature::ReserveStandardMacros.into(),
        b"thunkless_accessors" => CrubitFeature::ThunklessAccessors.into(),
        b"oo_casting" => CrubitFeature::OoCasting.into(),
        b"proto_references" => CrubitFeature::ProtoReferences.into(),
        b"generics" => CrubitFeature::Generics.into(),
        b"nonnull_smart_pointers" => CrubitFeature::NonnullSmartPointers.into(),
        b"impl_cxx_extern_type" => CrubitFeature::ImplCxxExternType.into(),
        b"no_impl_cxx_extern_type" => CrubitFeature::NoImplCxxExternType.into(),
        b"enum_api_v2" => CrubitFeature::EnumApiV2.into(),
        b"infer_send_sync" => CrubitFeature::InferSendSync.into(),
        b"iterator_item_references" => CrubitFeature::IteratorItemReferences.into(),
        b"inherited_friend_operators" => CrubitFeature::InheritedFriendOperators.into(),
        b"unsafe_template_params" => CrubitFeature::UnsafeTemplateParams.into(),
        b"movable_return_values" => CrubitFeature::MovableReturnValues.into(),
        b"strong_ordering_traits" => CrubitFeature::StrongOrderingTraits.into(),
        b"constant_arrays" => CrubitFeature::ConstantArrays.into(),
        b"macro_constants" => CrubitFeature::MacroConstants.into(),
        _ => return None,
        // importer.cc: make sure the logic for the "all" feature still makes sense: b/530193579
        // LINT.ThenChange(
        //     //depot/rs_bindings_from_cc/importer.cc,
        //     //depot/features/BUILD
        // )
    };
    Some(features)
}

/// A newtype around a single named feature flagset, so that it can be
/// deserialized from a string instead of an integer.
#[derive(Debug, Default, PartialEq, Eq, Clone)]
struct SerializedCrubitFeature(pub flagset::FlagSet<CrubitFeature>);

impl<'de> Deserialize<'de> for SerializedCrubitFeature {
    fn deserialize<D>(deserializer: D) -> Result<SerializedCrubitFeature, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // we can't just deserialize as a `&[u8]` or similar, because the bytes may be
        // ephemeral (e.g. from a `\u` escape). Aside from that, serde_json also
        // insists on handing out allocated strings sometimes. So we write our
        // own visitor, which can always handle even ephemeral bytestrings.
        struct CrubitFeatureVisitor;

        impl<'de> serde::de::Visitor<'de> for CrubitFeatureVisitor {
            type Value = SerializedCrubitFeature;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a Crubit feature flag name")
            }

            fn visit_bytes<E>(self, bytes: &[u8]) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let Some(features) = named_features(bytes) else {
                    return Err(E::custom(format!(
                        "Unexpected Crubit feature: {:?}",
                        String::from_utf8_lossy(bytes)
                    )));
                };

                Ok(SerializedCrubitFeature(features))
            }
        }

        deserializer.deserialize_bytes(CrubitFeatureVisitor)
    }
}

/// A newtype around a union of named feature flagsets, so that it can be
/// deserialized from an array of strings instead of an integer.
#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct SerializedCrubitFeatures(pub flagset::FlagSet<CrubitFeature>);

impl SerializedCrubitFeatures {
    /// Returns a new `SerializedCrubitFeatures` after resolving conflicts.
    pub fn resolved(mut features: flagset::FlagSet<CrubitFeature>) -> Self {
        if features.contains(CrubitFeature::NoAssumeLifetimes) {
            features -= CrubitFeature::AssumeLifetimes;
        }
        features -= CrubitFeature::NoAssumeLifetimes;
        if features.contains(CrubitFeature::NoTemplateInstantiation) {
            features -= CrubitFeature::TemplateInstantiation;
        }
        features -= CrubitFeature::NoTemplateInstantiation;
        if features.contains(CrubitFeature::NoImplCxxExternType) {
            features -= CrubitFeature::ImplCxxExternType;
        }
        features -= CrubitFeature::NoImplCxxExternType;
        Self(features)
    }
}

impl<'de> Deserialize<'de> for SerializedCrubitFeatures {
    fn deserialize<D>(deserializer: D) -> Result<SerializedCrubitFeatures, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct CrubitFeaturesVisitor;

        impl<'de> serde::de::Visitor<'de> for CrubitFeaturesVisitor {
            type Value = SerializedCrubitFeatures;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence of Crubit feature flag names")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut result = <flagset::FlagSet<CrubitFeature>>::default();

                while let Some(SerializedCrubitFeature(flags)) = seq.next_element()? {
                    result |= flags;
                }

                Ok(SerializedCrubitFeatures::resolved(result))
            }
        }

        deserializer.deserialize_seq(CrubitFeaturesVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use googletest::gtest;

    #[gtest]
    fn test_serialized_crubit_feature() {
        let SerializedCrubitFeature(features) = serde_json::from_str("\"supported\"").unwrap();
        assert_eq!(
            features,
            CrubitFeature::Supported | CrubitFeature::Types | CrubitFeature::ImplCxxExternType
        );
    }

    #[gtest]
    fn test_serialized_crubit_feature_types() {
        let SerializedCrubitFeature(features) = serde_json::from_str("\"types\"").unwrap();
        assert_eq!(features, CrubitFeature::Types | CrubitFeature::ImplCxxExternType);
    }

    #[gtest]
    fn test_serialized_crubit_feature_generics() {
        let SerializedCrubitFeature(features) = serde_json::from_str("\"generics\"").unwrap();
        assert_eq!(features, CrubitFeature::Generics);
    }

    #[gtest]
    fn test_serialized_crubit_feature_impl_cxx_extern_type() {
        let SerializedCrubitFeature(features) =
            serde_json::from_str("\"impl_cxx_extern_type\"").unwrap();
        assert_eq!(features, CrubitFeature::ImplCxxExternType);
    }

    #[gtest]
    fn test_serialized_crubit_feature_no_impl_cxx_extern_type() {
        let SerializedCrubitFeature(features) =
            serde_json::from_str("\"no_impl_cxx_extern_type\"").unwrap();
        assert_eq!(features, CrubitFeature::NoImplCxxExternType);
    }

    #[gtest]
    fn test_serialized_crubit_feature_enum_api_v2() {
        let SerializedCrubitFeature(features) = serde_json::from_str("\"enum_api_v2\"").unwrap();
        assert_eq!(features, CrubitFeature::EnumApiV2);
    }

    #[gtest]
    fn test_serialized_crubit_feature_infer_send_sync() {
        let SerializedCrubitFeature(features) =
            serde_json::from_str("\"infer_send_sync\"").unwrap();
        assert_eq!(features, CrubitFeature::InferSendSync);
    }

    #[gtest]
    fn test_serialized_crubit_feature_iterator_item_references() {
        let SerializedCrubitFeature(features) =
            serde_json::from_str("\"iterator_item_references\"").unwrap();
        assert_eq!(features, CrubitFeature::IteratorItemReferences);
    }

    #[gtest]
    fn test_serialized_crubit_feature_unsafe_template_params() {
        let SerializedCrubitFeature(features) =
            serde_json::from_str("\"unsafe_template_params\"").unwrap();
        assert_eq!(features, CrubitFeature::UnsafeTemplateParams);
    }

    #[gtest]
    fn test_serialized_crubit_feature_movable_return_values() {
        let SerializedCrubitFeature(features) =
            serde_json::from_str("\"movable_return_values\"").unwrap();
        assert_eq!(features, CrubitFeature::MovableReturnValues);
    }

    #[gtest]
    fn test_serialized_crubit_feature_strong_ordering_traits() {
        let SerializedCrubitFeature(features) =
            serde_json::from_str("\"strong_ordering_traits\"").unwrap();
        assert_eq!(features, CrubitFeature::StrongOrderingTraits);
    }

    #[gtest]
    fn test_serialized_crubit_feature_constant_arrays() {
        let SerializedCrubitFeature(features) =
            serde_json::from_str("\"constant_arrays\"").unwrap();
        assert_eq!(features, CrubitFeature::ConstantArrays);
    }

    #[gtest]
    fn test_serialized_crubit_feature_macro_constants() {
        let SerializedCrubitFeature(features) =
            serde_json::from_str("\"macro_constants\"").unwrap();
        assert_eq!(features, CrubitFeature::MacroConstants);
    }

    #[gtest]
    fn test_serialized_crubit_feature_all() {
        let SerializedCrubitFeature(features) = serde_json::from_str("\"all\"").unwrap();
        assert_eq!(
            features,
            CrubitFeature::Supported
                | CrubitFeature::Types
                | CrubitFeature::Experimental
                | CrubitFeature::AssumeLifetimes
                | CrubitFeature::AssumeThisLifetimes
                | CrubitFeature::UnsafeView
                | CrubitFeature::CheckDefaultInitialized
                | CrubitFeature::LeadingColonsForCppType
                | CrubitFeature::TemplateInstantiation
                | CrubitFeature::RecordImplDebug
                | CrubitFeature::CtorPlainValues
                | CrubitFeature::ReserveStandardMacros
                | CrubitFeature::ThunklessAccessors
                | CrubitFeature::ImplCxxExternType
                | CrubitFeature::InheritedFriendOperators
                | CrubitFeature::StrongOrderingTraits
                | CrubitFeature::ConstantArrays
                | CrubitFeature::MacroConstants
        );
    }

    #[gtest]
    fn test_serialized_crubit_features_empty() {
        let SerializedCrubitFeatures(features) = serde_json::from_str("[]").unwrap();
        assert!(features.is_empty());
    }

    #[gtest]
    fn test_serialized_crubit_features() {
        let SerializedCrubitFeatures(features) =
            serde_json::from_str("[\"supported\", \"experimental\"]").unwrap();
        assert_eq!(
            features,
            CrubitFeature::Supported
                | CrubitFeature::Types
                | CrubitFeature::Experimental
                | CrubitFeature::ImplCxxExternType
        );
    }

    #[gtest]
    fn test_serialized_crubit_features_all() {
        let SerializedCrubitFeatures(features) = serde_json::from_str("[\"all\"]").unwrap();
        assert_eq!(
            features,
            CrubitFeature::Supported
                | CrubitFeature::Types
                | CrubitFeature::Experimental
                | CrubitFeature::AssumeLifetimes
                | CrubitFeature::AssumeThisLifetimes
                | CrubitFeature::UnsafeView
                | CrubitFeature::CheckDefaultInitialized
                | CrubitFeature::LeadingColonsForCppType
                | CrubitFeature::TemplateInstantiation
                | CrubitFeature::RecordImplDebug
                | CrubitFeature::CtorPlainValues
                | CrubitFeature::ReserveStandardMacros
                | CrubitFeature::ThunklessAccessors
                | CrubitFeature::ImplCxxExternType
                | CrubitFeature::InheritedFriendOperators
                | CrubitFeature::StrongOrderingTraits
                | CrubitFeature::ConstantArrays
                | CrubitFeature::MacroConstants
        );
    }

    #[gtest]
    fn test_serialized_crubit_features_all_overlapping() {
        let SerializedCrubitFeatures(features) =
            serde_json::from_str("[\"all\", \"supported\", \"experimental\"]").unwrap();
        assert_eq!(
            features,
            CrubitFeature::Supported
                | CrubitFeature::Types
                | CrubitFeature::Experimental
                | CrubitFeature::AssumeLifetimes
                | CrubitFeature::AssumeThisLifetimes
                | CrubitFeature::UnsafeView
                | CrubitFeature::CheckDefaultInitialized
                | CrubitFeature::LeadingColonsForCppType
                | CrubitFeature::TemplateInstantiation
                | CrubitFeature::RecordImplDebug
                | CrubitFeature::CtorPlainValues
                | CrubitFeature::ReserveStandardMacros
                | CrubitFeature::ThunklessAccessors
                | CrubitFeature::ImplCxxExternType
                | CrubitFeature::InheritedFriendOperators
                | CrubitFeature::StrongOrderingTraits
                | CrubitFeature::ConstantArrays
                | CrubitFeature::MacroConstants
        );
    }

    #[gtest]
    fn test_serialized_crubit_features_all_overlapping_no_assume_lifetimes() {
        let SerializedCrubitFeatures(features) = serde_json::from_str(
            "[\"all\", \"supported\", \"experimental\", \"no_assume_lifetimes\"]",
        )
        .unwrap();
        assert_eq!(
            features,
            CrubitFeature::Supported
                | CrubitFeature::Types
                | CrubitFeature::Experimental
                | CrubitFeature::AssumeThisLifetimes
                | CrubitFeature::UnsafeView
                | CrubitFeature::CheckDefaultInitialized
                | CrubitFeature::LeadingColonsForCppType
                | CrubitFeature::TemplateInstantiation
                | CrubitFeature::RecordImplDebug
                | CrubitFeature::CtorPlainValues
                | CrubitFeature::ReserveStandardMacros
                | CrubitFeature::ThunklessAccessors
                | CrubitFeature::ImplCxxExternType
                | CrubitFeature::InheritedFriendOperators
                | CrubitFeature::StrongOrderingTraits
                | CrubitFeature::ConstantArrays
                | CrubitFeature::MacroConstants
        );
    }

    #[gtest]
    fn test_serialized_crubit_features_all_overlapping_no_template_instantiation() {
        let SerializedCrubitFeatures(features) = serde_json::from_str(
            "[\"all\", \"supported\", \"experimental\", \"no_template_instantiation\"]",
        )
        .unwrap();
        assert_eq!(
            features,
            CrubitFeature::Supported
                | CrubitFeature::Types
                | CrubitFeature::Experimental
                | CrubitFeature::AssumeLifetimes
                | CrubitFeature::AssumeThisLifetimes
                | CrubitFeature::UnsafeView
                | CrubitFeature::CheckDefaultInitialized
                | CrubitFeature::LeadingColonsForCppType
                | CrubitFeature::RecordImplDebug
                | CrubitFeature::CtorPlainValues
                | CrubitFeature::ReserveStandardMacros
                | CrubitFeature::ThunklessAccessors
                | CrubitFeature::ImplCxxExternType
                | CrubitFeature::InheritedFriendOperators
                | CrubitFeature::StrongOrderingTraits
                | CrubitFeature::ConstantArrays
                | CrubitFeature::MacroConstants
        );
    }

    #[gtest]
    fn test_serialized_crubit_features_all_overlapping_no_impl_cxx_extern_type() {
        let SerializedCrubitFeatures(features) = serde_json::from_str(
            "[\"all\", \"supported\", \"experimental\", \"no_impl_cxx_extern_type\"]",
        )
        .unwrap();
        assert_eq!(
            features,
            CrubitFeature::Supported
                | CrubitFeature::Types
                | CrubitFeature::Experimental
                | CrubitFeature::AssumeLifetimes
                | CrubitFeature::AssumeThisLifetimes
                | CrubitFeature::UnsafeView
                | CrubitFeature::CheckDefaultInitialized
                | CrubitFeature::LeadingColonsForCppType
                | CrubitFeature::TemplateInstantiation
                | CrubitFeature::RecordImplDebug
                | CrubitFeature::CtorPlainValues
                | CrubitFeature::ReserveStandardMacros
                | CrubitFeature::ThunklessAccessors
                | CrubitFeature::InheritedFriendOperators
                | CrubitFeature::StrongOrderingTraits
                | CrubitFeature::ConstantArrays
                | CrubitFeature::MacroConstants
        );
    }
}
