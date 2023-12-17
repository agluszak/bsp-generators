use serde::{Deserialize, Serialize};

use crate::*;
use std::collections::{BTreeMap, BTreeSet};

/// The feature dependency graph is a mapping between
/// feature and the features it turns on
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(transparent)]
pub struct FeaturesDependencyGraph(pub BTreeMap<Feature, BTreeSet<Feature>>);

impl FeaturesDependencyGraph {
    pub fn new(input: BTreeMap<Feature, BTreeSet<Feature>>) -> Self {
        Self(input)
    }
}

impl std::ops::Deref for FeaturesDependencyGraph {
    type Target = BTreeMap<Feature, BTreeSet<Feature>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;
    use insta::assert_compact_json_snapshot;

    #[test]
    fn features_dependency_graph() {
        let test_data = FeaturesDependencyGraph(BTreeMap::from([(
            Feature::default(),
            BTreeSet::<Feature>::default(),
        )]));

        assert_compact_json_snapshot!(
           test_data,
           @r#"{"": []}"#
        );

        test_deserialization(r#"{"": []}"#, &test_data);
    }
}
