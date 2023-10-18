use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DiagnosticCode {
    String(String),
    I32(i32),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;
    use insta::assert_compact_json_snapshot;

    #[test]
    fn diagnostic_code() {
        assert_compact_json_snapshot!(
           DiagnosticCode::String(TEST_STRING.to_string()),
           @r#""test_string""#
        );
        test_deserialization(
            r#""test_string""#,
            &DiagnosticCode::String(TEST_STRING.to_string()),
        );

        assert_compact_json_snapshot!(
           DiagnosticCode::I32(TEST_INT),
           @r#"1"#
        );
        test_deserialization(r#"1"#, &DiagnosticCode::I32(TEST_INT));
    }
}
