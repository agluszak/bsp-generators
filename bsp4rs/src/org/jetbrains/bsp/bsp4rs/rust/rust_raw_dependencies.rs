use serde::{Deserialize, Serialize};

use crate::*;
use std::collections::BTreeMap;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(transparent)]
pub struct RustRawDependencies(pub BTreeMap<String, Vec<RustRawDependency>>);

impl RustRawDependencies {
    pub fn new(input: BTreeMap<String, Vec<RustRawDependency>>) -> Self {
        Self(input)
    }
}

impl std::ops::Deref for RustRawDependencies {
    type Target = BTreeMap<String, Vec<RustRawDependency>>;

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
    fn rust_raw_dependencies() {
        assert_compact_json_snapshot!(RustRawDependencies(BTreeMap::from([(TEST_STRING.to_string(), vec![RustRawDependency::default()])])), @r#"{"test_string": [{"name": "", "optional": false, "usesDefaultFeatures": false, "features": []}]}"#);
    }
}
