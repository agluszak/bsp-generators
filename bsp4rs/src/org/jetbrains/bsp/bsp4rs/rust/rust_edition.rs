use serde::{Deserialize, Serialize};

/// The Rust edition.
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(transparent)]
pub struct RustEdition(pub std::borrow::Cow<'static, str>);

impl RustEdition {
    pub const E2015: RustEdition = RustEdition::new("2015");
    pub const E2018: RustEdition = RustEdition::new("2018");
    pub const E2021: RustEdition = RustEdition::new("2021");

    pub const fn new(tag: &'static str) -> Self {
        Self(std::borrow::Cow::Borrowed(tag))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;
    use insta::assert_compact_json_snapshot;

    #[test]
    fn rust_edition() {
        assert_compact_json_snapshot!(
           RustEdition::E2015,
           @r#""2015""#
        );
        test_deserialization(r#""2015""#, &RustEdition::E2015);

        assert_compact_json_snapshot!(
           RustEdition::E2018,
           @r#""2018""#
        );
        test_deserialization(r#""2018""#, &RustEdition::E2018);

        assert_compact_json_snapshot!(
           RustEdition::E2021,
           @r#""2021""#
        );
        test_deserialization(r#""2021""#, &RustEdition::E2021);
    }
}
