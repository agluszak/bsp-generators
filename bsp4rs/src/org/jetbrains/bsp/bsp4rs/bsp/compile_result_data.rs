use serde::{Deserialize, Serialize};

use crate::*;

#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "dataKind", content = "data")]
pub enum NamedCompileResultData {}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CompileResultData {
    Named(NamedCompileResultData),
    Other(OtherData),
}

impl CompileResultData {}

#[cfg(test)]
mod tests {
    use insta::assert_json_snapshot;

    use super::*;

    #[test]
    fn compile_result_data() {
        assert_json_snapshot!(
           CompileResultData::Other(OtherData::default()),
           @r#"
{
  "dataKind": "",
  "data": null
}
   "#
        );
    }
}
