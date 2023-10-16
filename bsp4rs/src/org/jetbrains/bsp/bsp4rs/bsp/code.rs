use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Code {
    String(String),
    I32(i32),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;
    use insta::assert_compact_json_snapshot;

    #[test]
    fn code() {
        assert_compact_json_snapshot!(
           Code::String(TEST_STRING.to_string()),
           @r#""test_string""#
        );
        test_deserialization(r#""test_string""#, &Code::String(TEST_STRING.to_string()));

        assert_compact_json_snapshot!(
           Code::I32(TEST_INT),
           @r#"1"#
        );
        test_deserialization(r#"1"#, &Code::I32(TEST_INT));
    }
}
