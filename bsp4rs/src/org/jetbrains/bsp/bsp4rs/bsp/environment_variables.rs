use serde::{Deserialize, Serialize};

use std::collections::BTreeMap;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(transparent)]
pub struct EnvironmentVariables(pub BTreeMap<String, String>);

impl EnvironmentVariables {
    pub fn new(input: BTreeMap<String, String>) -> Self {
        Self(input)
    }
}

impl std::ops::Deref for EnvironmentVariables {
    type Target = BTreeMap<String, String>;

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
    fn environment_variables() {
        let test_data = EnvironmentVariables(BTreeMap::from([(
            TEST_STRING.to_string(),
            TEST_STRING.to_string(),
        )]));

        assert_compact_json_snapshot!(
           test_data,
           @r#"{"test_string": "test_string"}"#
        );

        test_deserialization(r#"{"test_string": "test_string"}"#, &test_data);
    }
}
