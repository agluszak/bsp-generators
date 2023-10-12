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
    use insta::assert_json_snapshot;

    use super::*;

    #[test]
    fn scala_platform() {
        assert_json_snapshot!(ScalaPlatform::Jvm, @r#"1"#);
        assert_json_snapshot!(ScalaPlatform::Js, @r#"2"#);
        assert_json_snapshot!(ScalaPlatform::Native, @r#"3"#);
    }
}
