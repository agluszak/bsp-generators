use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DiagnosticTag(pub i32);

impl DiagnosticTag {
    /// Unused or unnecessary code.
    ///
    /// Clients are allowed to render diagnostics with this tag faded out instead of having an error squiggle.
    pub const UNNECESSARY: DiagnosticTag = DiagnosticTag::new(1);
    /// Deprecated or obsolete code.
    ///
    /// Clients are allowed to rendered diagnostics with this tag strike through.
    pub const DEPRECATED: DiagnosticTag = DiagnosticTag::new(2);

    pub const fn new(tag: i32) -> Self {
        Self(tag)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;
    use insta::assert_compact_json_snapshot;

    #[test]
    fn diagnostic_tag() {
        assert_compact_json_snapshot!(
           DiagnosticTag::UNNECESSARY,
           @r#"1"#
        );
        test_deserialization(r#"1"#, &DiagnosticTag::UNNECESSARY);

        assert_compact_json_snapshot!(
           DiagnosticTag::DEPRECATED,
           @r#"2"#
        );
        test_deserialization(r#"2"#, &DiagnosticTag::DEPRECATED);
    }
}
