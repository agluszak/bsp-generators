use serde::{Deserialize, Serialize};

/// Represents the identifier of a BSP request.
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(transparent)]
pub struct RequestId(pub String);

impl RequestId {
    pub fn new(input: String) -> Self {
        Self(input)
    }
}

impl std::ops::Deref for RequestId {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<&str> for RequestId {
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
    fn request_id() {
        let test_data = RequestId(TEST_STRING.to_string());

        assert_compact_json_snapshot!(
           test_data,
           @r#""test_string""#
        );

        test_deserialization(r#""test_string""#, &test_data);
    }
}
