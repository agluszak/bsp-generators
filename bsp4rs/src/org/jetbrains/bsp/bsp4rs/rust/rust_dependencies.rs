use serde::{Deserialize, Serialize};

use crate::*;
use std::collections::BTreeMap;

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
            TEST_STRING.to_string(),
            vec![RustDependency::default()],
        )]));

        assert_compact_json_snapshot!(
           test_data,
           @r#"{"test_string": [{"pkg": ""}]}"#
        );

        test_deserialization(r#"{"test_string": [{"pkg": ""}]}"#, &test_data);
    }
}
