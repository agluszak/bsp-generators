use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(
    Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize_repr, Deserialize_repr,
)]
#[repr(u8)]
pub enum DiagnosticSeverity {
    /// Reports an error.
    #[default]
    Error = 1,
    /// Reports a warning.
    Warning = 2,
    /// Reports an information.
    Information = 3,
    /// Reports a hint.
    Hint = 4,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;
    use insta::assert_compact_json_snapshot;

    #[test]
    fn diagnostic_severity() {
        assert_compact_json_snapshot!(
           DiagnosticSeverity::Error,
           @r#"1"#
        );
        test_deserialization(r#"1"#, &DiagnosticSeverity::Error);

        assert_compact_json_snapshot!(
           DiagnosticSeverity::Warning,
           @r#"2"#
        );
        test_deserialization(r#"2"#, &DiagnosticSeverity::Warning);

        assert_compact_json_snapshot!(
           DiagnosticSeverity::Information,
           @r#"3"#
        );
        test_deserialization(r#"3"#, &DiagnosticSeverity::Information);

        assert_compact_json_snapshot!(
           DiagnosticSeverity::Hint,
           @r#"4"#
        );
        test_deserialization(r#"4"#, &DiagnosticSeverity::Hint);
    }
}
