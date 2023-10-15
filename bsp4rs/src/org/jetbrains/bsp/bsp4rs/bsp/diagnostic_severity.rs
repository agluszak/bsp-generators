use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(
    Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize_repr, Deserialize_repr,
)]
#[repr(u8)]
pub enum DiagnosticSeverity {
    #[default]
    Error = 1,
    Warning = 2,
    Information = 3,
    Hint = 4,
}

#[cfg(test)]
mod tests {
    use insta::assert_json_snapshot;

    use super::*;

    #[test]
    fn diagnostic_severity() {
        assert_json_snapshot!(
           DiagnosticSeverity::Error,
           @r#"
1
   "#
        );
        assert_json_snapshot!(
           DiagnosticSeverity::Warning,
           @r#"
2
   "#
        );
        assert_json_snapshot!(
           DiagnosticSeverity::Information,
           @r#"
3
   "#
        );
        assert_json_snapshot!(
           DiagnosticSeverity::Hint,
           @r#"
4
   "#
        );
    }
}
