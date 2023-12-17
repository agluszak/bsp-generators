use serde::{Deserialize, Serialize};

use crate::*;
use std::collections::BTreeMap;

/// The RustDependencies is a mapping between
/// package id and the package's dependencies info.
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(transparent)]
pub struct RustDependencies(pub BTreeMap<String, Vec<RustDependency>>);

impl RustDependencies {
    pub fn new(input: BTreeMap<String, Vec<RustDependency>>) -> Self {
        Self(input)
    }
}

impl std::ops::Deref for RustDependencies {
    type Target = BTreeMap<String, Vec<RustDependency>>;

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
    fn rust_dependencies() {
        let test_data = RustDependencies(BTreeMap::from([(
            String::default(),
            Vec::<RustDependency>::default(),
        )]));

        assert_compact_json_snapshot!(
           test_data,
           @r#"{"": []}"#
        );

        test_deserialization(r#"{"": []}"#, &test_data);
    }
}
