use serde::{Deserialize, Serialize};

/// Represents the identifier of a JsonRpc request id.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RequestId {
    String(String),
    I32(i32),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;
    use insta::assert_compact_json_snapshot;

    #[test]
    fn request_id() {
        assert_compact_json_snapshot!(
           RequestId::String(TEST_STRING.to_string()),
           @r#""test_string""#
        );
        test_deserialization(
            r#""test_string""#,
            &RequestId::String(TEST_STRING.to_string()),
        );

        assert_compact_json_snapshot!(
           RequestId::I32(TEST_INT),
           @r#"1"#
        );
        test_deserialization(r#"1"#, &RequestId::I32(TEST_INT));
    }
}
