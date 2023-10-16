use serde::{Deserialize, Serialize};

/// A resource identifier that is a valid URI according to rfc3986:
/// https://tools.ietf.org/html/rfc3986
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(transparent)]
pub struct URI(pub String);

impl URI {
    pub fn new(input: String) -> Self {
        Self(input)
    }
}

impl std::ops::Deref for URI {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<&str> for URI {
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
    fn uri() {
        let test_data = URI(TEST_STRING.to_string());

        assert_compact_json_snapshot!(
           test_data,
           @r#""test_string""#
        );

        test_deserialization(r#""test_string""#, &test_data);
    }
}
