use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(
    Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize_repr, Deserialize_repr,
)]
#[repr(u8)]
pub enum ScalaPlatform {
    #[default]
    Jvm = 1,
    Js = 2,
    Native = 3,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;
    use insta::assert_compact_json_snapshot;

    #[test]
    fn scala_platform() {
        assert_compact_json_snapshot!(
           ScalaPlatform::Jvm,
           @r#"1"#
        );
        test_deserialization(r#"1"#, &ScalaPlatform::Jvm);

        assert_compact_json_snapshot!(
           ScalaPlatform::Js,
           @r#"2"#
        );
        test_deserialization(r#"2"#, &ScalaPlatform::Js);

        assert_compact_json_snapshot!(
           ScalaPlatform::Native,
           @r#"3"#
        );
        test_deserialization(r#"3"#, &ScalaPlatform::Native);
    }
}
