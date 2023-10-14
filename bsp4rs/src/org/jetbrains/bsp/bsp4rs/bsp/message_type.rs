use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(
    Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize_repr, Deserialize_repr,
)]
#[repr(u8)]
pub enum MessageType {
    #[default]
    /// An error message.
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
    use insta::assert_compact_json_snapshot;

    use super::*;

    #[test]
    fn message_type() {
        assert_compact_json_snapshot!(MessageType::Error, @r#"1"#);
        assert_compact_json_snapshot!(MessageType::Warning, @r#"2"#);
        assert_compact_json_snapshot!(MessageType::Info, @r#"3"#);
        assert_compact_json_snapshot!(MessageType::Log, @r#"4"#);
    }
}
