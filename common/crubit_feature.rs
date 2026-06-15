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
    pub enum CrubitFeature : u16 {
        Supported,

        Wrapper,

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
        UseProtobufIR,

        /// C++ default constructors are checked to fully initialize all public fields.
        CheckDefaultInitialized,

        /// Prepend `::` to `cpp_type` in annotations.
        LeadingColonsForCppType,

        /// Generate bindings for (non-Crubit special-cased) template instances.
        TemplateInstantiation,

        /// Emit `rs_std::Tuple` everywhere instead of C++ `std::tuple`.
        LayoutCompatTuple,

        /// Always specialize generics in cpp_api_from_rust, instead of doing composable bridging
        /// when possible.
        AlwaysSpecializeGenericsInCppApiFromRust,

        /// Generate bindings using the nested IR.
        UseNestedIr,
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
            Self::Wrapper => "wrapper",
            Self::Types => "types",
            Self::Experimental => "experimental",
            Self::AssumeLifetimes => "assume_lifetimes",
            Self::AssumeThisLifetimes => "assume_this_lifetimes",
            Self::NoAssumeLifetimes => "no_assume_lifetimes",
            Self::UseProtobufIR => "use_protobuf_ir",
            Self::UnsafeView => "unsafe_view",
            Self::CheckDefaultInitialized => "check_default_initialized",
            Self::LeadingColonsForCppType => "leading_colons_for_cpp_type",
            Self::TemplateInstantiation => "template_instantiation",
            Self::LayoutCompatTuple => "layout_compat_tuple",
            Self::AlwaysSpecializeGenericsInCppApiFromRust => {
                "always_specialize_generics_in_cpp_api_from_rust"
            }
            Self::UseNestedIr => "use_nested_ir",
        }
    }

    /// The aspect hint required to enable this feature.
    ///
    /// This should be a label in features/BUILD.
    pub fn aspect_hint(&self) -> &'static str {
        match self {
            Self::Supported => "//features:supported",
            Self::Wrapper => "//features:wrapper",
            Self::Types => "//features:types",
            Self::Experimental => "//features:experimental",
            Self::AssumeLifetimes => "//features:assume_lifetimes",
            Self::AssumeThisLifetimes => "//features:assume_this_lifetimes",
            Self::NoAssumeLifetimes => "//features:no_assume_lifetimes",
            Self::UseProtobufIR => "//features:use_protobuf_ir",
            Self::UnsafeView => "//features:unsafe_view",
            Self::CheckDefaultInitialized => {
                "//features:check_default_initialized"
            }
            Self::LeadingColonsForCppType => {
                "//features:leading_colons_for_cpp_type"
            }
            Self::TemplateInstantiation => "//features:template_instantiation",
            Self::LayoutCompatTuple => "//features:layout_compat_tuple",
            Self::AlwaysSpecializeGenericsInCppApiFromRust => {
                "//features:always_specialize_generics_in_cpp_api_from_rust"
            }
            Self::UseNestedIr => "//features:use_nested_ir",
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
                - CrubitFeature::LayoutCompatTuple
                - CrubitFeature::AlwaysSpecializeGenericsInCppApiFromRust
        }
        // `supported` automatically implies `types`.
        b"supported" => CrubitFeature::Supported | CrubitFeature::Types,
        b"wrapper" => CrubitFeature::Wrapper.into(),
        b"types" => CrubitFeature::Types.into(),
        b"experimental" => CrubitFeature::Experimental.into(),
        b"assume_lifetimes" => CrubitFeature::AssumeLifetimes.into(),
        b"assume_this_lifetimes" => CrubitFeature::AssumeThisLifetimes.into(),
        b"no_assume_lifetimes" => CrubitFeature::NoAssumeLifetimes.into(),
        b"use_protobuf_ir" => CrubitFeature::UseProtobufIR.into(),
        b"unsafe_view" => CrubitFeature::UnsafeView.into(),
        b"check_default_initialized" => CrubitFeature::CheckDefaultInitialized.into(),
        b"leading_colons_for_cpp_type" => CrubitFeature::LeadingColonsForCppType.into(),
        b"template_instantiation" => CrubitFeature::TemplateInstantiation.into(),
        b"layout_compat_tuple" => CrubitFeature::LayoutCompatTuple.into(),
        b"always_specialize_generics_in_cpp_api_from_rust" => {
            CrubitFeature::AlwaysSpecializeGenericsInCppApiFromRust.into()
        }
        b"use_nested_ir" => CrubitFeature::UseNestedIr.into(),
        _ => return None,
        // LINT.ThenChange(//depot/rs_bindings_from_cc/importer.cc, //depot/features/BUILD)
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
        assert_eq!(features, CrubitFeature::Supported | CrubitFeature::Types);
    }

    #[gtest]
    fn test_serialized_crubit_feature_all() {
        let SerializedCrubitFeature(features) = serde_json::from_str("\"all\"").unwrap();
        assert_eq!(
            features,
            CrubitFeature::Supported
                | CrubitFeature::Wrapper
                | CrubitFeature::Types
                | CrubitFeature::Experimental
                | CrubitFeature::AssumeLifetimes
                | CrubitFeature::AssumeThisLifetimes
                | CrubitFeature::UnsafeView
                | CrubitFeature::UseProtobufIR
                | CrubitFeature::CheckDefaultInitialized
                | CrubitFeature::LeadingColonsForCppType
                | CrubitFeature::TemplateInstantiation
                | CrubitFeature::UseNestedIr
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
            CrubitFeature::Supported | CrubitFeature::Types | CrubitFeature::Experimental
        );
    }

    #[gtest]
    fn test_serialized_crubit_features_all() {
        let SerializedCrubitFeatures(features) = serde_json::from_str("[\"all\"]").unwrap();
        assert_eq!(
            features,
            CrubitFeature::Supported
                | CrubitFeature::Wrapper
                | CrubitFeature::Types
                | CrubitFeature::Experimental
                | CrubitFeature::AssumeLifetimes
                | CrubitFeature::AssumeThisLifetimes
                | CrubitFeature::UnsafeView
                | CrubitFeature::UseProtobufIR
                | CrubitFeature::CheckDefaultInitialized
                | CrubitFeature::LeadingColonsForCppType
                | CrubitFeature::TemplateInstantiation
                | CrubitFeature::UseNestedIr
        );
    }

    #[gtest]
    fn test_serialized_crubit_features_all_overlapping() {
        let SerializedCrubitFeatures(features) =
            serde_json::from_str("[\"all\", \"supported\", \"experimental\"]").unwrap();
        assert_eq!(
            features,
            CrubitFeature::Supported
                | CrubitFeature::Wrapper
                | CrubitFeature::Types
                | CrubitFeature::Experimental
                | CrubitFeature::AssumeLifetimes
                | CrubitFeature::AssumeThisLifetimes
                | CrubitFeature::UnsafeView
                | CrubitFeature::UseProtobufIR
                | CrubitFeature::CheckDefaultInitialized
                | CrubitFeature::LeadingColonsForCppType
                | CrubitFeature::TemplateInstantiation
                | CrubitFeature::UseNestedIr
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
                | CrubitFeature::Wrapper
                | CrubitFeature::Types
                | CrubitFeature::Experimental
                | CrubitFeature::AssumeThisLifetimes
                | CrubitFeature::UnsafeView
                | CrubitFeature::UseProtobufIR
                | CrubitFeature::CheckDefaultInitialized
                | CrubitFeature::LeadingColonsForCppType
                | CrubitFeature::TemplateInstantiation
                | CrubitFeature::UseNestedIr
        );
    }
}
