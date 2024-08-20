// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! Supporting types to read and display Crubit feature flags
//! (<internal link>)

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

/// A newtype around a flagset of features, so that it can be deserialized from
/// an array of strings instead of an integer.
#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct SerializedCrubitFeatures(pub flagset::FlagSet<CrubitFeature>);

impl<'de> serde::Deserialize<'de> for SerializedCrubitFeatures {
    fn deserialize<D>(deserializer: D) -> Result<SerializedCrubitFeatures, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let mut features = flagset::FlagSet::<CrubitFeature>::default();
        for feature in <Vec<String> as serde::Deserialize<'de>>::deserialize(deserializer)? {
            features |= match &*feature {
                "all" => flagset::FlagSet::<CrubitFeature>::full(),
                "supported" => CrubitFeature::Supported.into(),
                "experimental" => CrubitFeature::Experimental.into(),
                other => {
                    return Err(<D::Error as serde::de::Error>::custom(format!(
                        "Unexpected Crubit feature: {other}"
                    )));
                }
            };
        }
        Ok(SerializedCrubitFeatures(features))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use googletest::prelude::*;

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
