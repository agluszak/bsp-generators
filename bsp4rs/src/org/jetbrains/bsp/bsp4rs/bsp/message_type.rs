use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(
    Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize_repr, Deserialize_repr,
)]
#[repr(u8)]
pub enum MessageType {
    /// An error message.
    #[default]
    Error = 1,
    /// A warning message.
    Warning = 2,
    /// An information message.
    Info = 3,
    /// A log message.
    Log = 4,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;
    use insta::assert_compact_json_snapshot;

    #[test]
    fn message_type() {
        assert_compact_json_snapshot!(
           MessageType::Error,
           @r#"1"#
        );
        test_deserialization(r#"1"#, &MessageType::Error);

        assert_compact_json_snapshot!(
           MessageType::Warning,
           @r#"2"#
        );
        test_deserialization(r#"2"#, &MessageType::Warning);

        assert_compact_json_snapshot!(
           MessageType::Info,
           @r#"3"#
        );
        test_deserialization(r#"3"#, &MessageType::Info);

        assert_compact_json_snapshot!(
           MessageType::Log,
           @r#"4"#
        );
        test_deserialization(r#"4"#, &MessageType::Log);
    }
}
