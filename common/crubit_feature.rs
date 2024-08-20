// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! Supporting types to read and display Crubit feature flags
//! (<internal link>)
use serde::Deserialize;

flagset::flags! {
    pub enum CrubitFeature : u8 {
        Supported,
        /// Experimental is never *set* without also setting Supported, but we allow it to be
        /// *required* without also requiring Supported, so that error messages can be more direct.
        Experimental,
    }
}

impl CrubitFeature {
    /// The name of this feature.
    pub fn short_name(&self) -> &'static str {
        match self {
            Self::Supported => "supported",
            Self::Experimental => "experimental",
        }
    }

    /// The aspect hint required to enable this feature.
    pub fn aspect_hint(&self) -> &'static str {
        match self {
            Self::Supported => "//features:supported",
            Self::Experimental => "//features:experimental",
        }
    }
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
                let features = match bytes {
                    b"all" => flagset::FlagSet::<CrubitFeature>::full(),
                    b"supported" => CrubitFeature::Supported.into(),
                    b"experimental" => CrubitFeature::Experimental.into(),
                    other => {
                        return Err(E::custom(format!(
                            "Unexpected Crubit feature: {:?}",
                            String::from_utf8_lossy(other)
                        )));
                    }
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
        assert_eq!(features, CrubitFeature::Supported | CrubitFeature::Experimental);
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
        assert_eq!(features, CrubitFeature::Supported | CrubitFeature::Experimental);
    }

    #[gtest]
    fn test_serialized_crubit_features_all_overlapping() {
        let SerializedCrubitFeatures(features) =
            serde_json::from_str("[\"all\", \"supported\", \"experimental\"]").unwrap();
        assert_eq!(features, CrubitFeature::Supported | CrubitFeature::Experimental);
    }
}
