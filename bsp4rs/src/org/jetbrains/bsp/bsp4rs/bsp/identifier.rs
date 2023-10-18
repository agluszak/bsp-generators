use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Identifier(pub String);

impl Identifier {
    pub fn new(input: String) -> Self {
        Self(input)
    }
}

impl std::ops::Deref for Identifier {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<&str> for Identifier {
    fn from(input: &str) -> Self {
        Self(input.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;
    use insta::assert_compact_json_snapshot;

    #[test]
    fn identifier() {
        let test_data = Identifier(TEST_STRING.to_string());

        assert_compact_json_snapshot!(
           test_data,
           @r#""test_string""#
        );

        test_deserialization(r#""test_string""#, &test_data);
    }
}
