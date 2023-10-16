use serde_repr::{Deserialize_repr, Serialize_repr};

/// Included in notifications of tasks or requests to signal the completion state.
#[derive(
    Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize_repr, Deserialize_repr,
)]
#[repr(u8)]
pub enum StatusCode {
    #[default]
    /// Execution was successful.
    Ok = 1,
    /// Execution failed.
    Error = 2,
    /// Execution was cancelled.
    Cancelled = 3,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;
    use insta::assert_compact_json_snapshot;

    #[test]
    fn status_code() {
        assert_compact_json_snapshot!(
           StatusCode::Ok,
           @r#"1"#
        );
        test_deserialization(r#"1"#, &StatusCode::Ok);

        assert_compact_json_snapshot!(
           StatusCode::Error,
           @r#"2"#
        );
        test_deserialization(r#"2"#, &StatusCode::Error);

        assert_compact_json_snapshot!(
           StatusCode::Cancelled,
           @r#"3"#
        );
        test_deserialization(r#"3"#, &StatusCode::Cancelled);
    }
}
