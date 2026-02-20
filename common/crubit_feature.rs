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

        /// Enable the native Rust std::vector<T> reimplementation.
        StdVector,

        /// Enable the native Rust std::unique_ptr<T> reimplementation.
        StdUniquePtr,

        /// Enable handling non-Unpin types with the `ctor` crate.
        NonUnpinCtor,

        /// Experimental is never *set* without also setting Supported, but we allow it to be
        /// *required* without also requiring Supported, so that error messages can be more direct.
        Experimental,

        /// Enable emitting custom ffi_11 types instead of `core::ffi` types.
        CustomFfiTypes,

        /// Use ergonomic lifetime defaults when interpreting lifetime annotations.
        AssumeLifetimes,

        /// Enable formatting to Rust via C++.
        Fmt,

        /// Enable experimental support for `rs_std::DynCallable` and `absl::AnyInvocable`.
        Callables,

        /// Mark C++ types with `[[gsl::Pointer]]` as unsafe.
        UnsafeView,
    }
}

impl CrubitFeature {
    /// The name of this feature.
    ///
    /// This is used for serialization/deserialization: an aspect hint maps to a
    /// list of short names, whicwhichbh are passed to Crubit to enable the
    /// corresponding feature bits.
    pub fn short_name(&self) -> &'static str {
        match self {
            Self::Supported => "supported",
            Self::Wrapper => "wrapper",
            Self::StdVector => "std_vector",
            Self::StdUniquePtr => "std_unique_ptr",
            Self::NonUnpinCtor => "non_unpin_ctor",
            Self::Experimental => "experimental",
            Self::CustomFfiTypes => "custom_ffi_types",
            Self::AssumeLifetimes => "assume_lifetimes",
            Self::Fmt => "fmt",
            Self::Callables => "callables",
            Self::UnsafeView => "unsafe_view",
        }
    }

    /// The aspect hint required to enable this feature.
    ///
    /// This should be a label in features/BUILD.
    pub fn aspect_hint(&self) -> &'static str {
        match self {
            Self::Supported => "//features:supported",
            Self::Wrapper => "//features:wrapper",
            Self::StdVector => "//features:std_vector",
            Self::StdUniquePtr => "//features:std_unique_ptr",
            Self::NonUnpinCtor => "//features:non_unpin_ctor",
            Self::Experimental => "//features:experimental",
            Self::CustomFfiTypes => "//features:custom_ffi_types",
            Self::AssumeLifetimes => "//features:assume_lifetimes",
            Self::Fmt => "//features:fmt",
            Self::Callables => "//features:callables",
            Self::UnsafeView => "//features:unsafe_view",
        }
    }
}

/// Returns the set of features named by this short name.
pub fn named_features(name: &[u8]) -> Option<flagset::FlagSet<CrubitFeature>> {
    let features = match name {
        // TODO(b/454627672): return AssumeLifetimes to `all` once it can build the goldens.
        b"all" => flagset::FlagSet::<CrubitFeature>::full() - CrubitFeature::AssumeLifetimes,
        b"supported" => CrubitFeature::Supported.into(),
        b"wrapper" => CrubitFeature::Wrapper.into(),
        b"std_vector" => CrubitFeature::StdVector.into(),
        b"std_unique_ptr" => CrubitFeature::StdUniquePtr.into(),
        b"non_unpin_ctor" => CrubitFeature::NonUnpinCtor.into(),
        b"experimental" => CrubitFeature::Experimental.into(),
        b"custom_ffi_types" => CrubitFeature::CustomFfiTypes.into(),
        b"assume_lifetimes" => CrubitFeature::AssumeLifetimes.into(),
        b"fmt" => CrubitFeature::Fmt.into(),
        b"callables" => CrubitFeature::Callables.into(),
        b"unsafe_view" => CrubitFeature::UnsafeView.into(),
        _ => return None,
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

                Ok(SerializedCrubitFeatures(result))
            }
        }

        deserializer.deserialize_seq(CrubitFeaturesVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use googletest::prelude::*;

    #[gtest]
    fn test_serialized_crubit_feature() {
        let SerializedCrubitFeature(features) = serde_json::from_str("\"supported\"").unwrap();
        assert_eq!(features, CrubitFeature::Supported);
    }

    #[gtest]
    fn test_serialized_crubit_feature_all() {
        let SerializedCrubitFeature(features) = serde_json::from_str("\"all\"").unwrap();
        assert_eq!(
            features,
            CrubitFeature::Supported
                | CrubitFeature::Wrapper
                | CrubitFeature::StdVector
                | CrubitFeature::StdUniquePtr
                | CrubitFeature::NonUnpinCtor
                | CrubitFeature::Experimental
                | CrubitFeature::CustomFfiTypes
                | CrubitFeature::Fmt
                | CrubitFeature::Callables
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
        assert_eq!(features, CrubitFeature::Supported | CrubitFeature::Experimental);
    }

    #[gtest]
    fn test_serialized_crubit_features_all() {
        let SerializedCrubitFeatures(features) = serde_json::from_str("[\"all\"]").unwrap();
        assert_eq!(
            features,
            CrubitFeature::Supported
                | CrubitFeature::Wrapper
                | CrubitFeature::StdVector
                | CrubitFeature::StdUniquePtr
                | CrubitFeature::NonUnpinCtor
                | CrubitFeature::Experimental
                | CrubitFeature::CustomFfiTypes
                | CrubitFeature::Fmt
                | CrubitFeature::Callables
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
                | CrubitFeature::StdVector
                | CrubitFeature::StdUniquePtr
                | CrubitFeature::NonUnpinCtor
                | CrubitFeature::Experimental
                | CrubitFeature::CustomFfiTypes
                | CrubitFeature::Fmt
                | CrubitFeature::Callables
        );
    }
}
