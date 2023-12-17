use serde::{Deserialize, Serialize};

use crate::*;

#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "dataKind", content = "data")]
pub enum NamedDependencyModuleData {
    Maven(MavenDependencyModule),
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DependencyModuleData {
    Named(NamedDependencyModuleData),
    Other(OtherData),
}

impl DependencyModuleData {
    pub fn maven(data: MavenDependencyModule) -> Self {
        Self::Named(NamedDependencyModuleData::Maven(data))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use insta::assert_compact_json_snapshot;
    use insta::assert_json_snapshot;

    use serde_json::json;

    #[test]
    fn dependency_module_data() {
        assert_json_snapshot!(DependencyModuleData::maven(MavenDependencyModule::default()),
@r#"
{
  "dataKind": "maven",
  "data": {
    "organization": "",
    "name": "",
    "version": "",
    "artifacts": []
  }
}
"#);

        assert_compact_json_snapshot!(
           DependencyModuleData::Other(OtherData { data: json!({}), ..OtherData::default()}),
           @r#"{"dataKind": "", "data": {}}"#
        );
    }
}
