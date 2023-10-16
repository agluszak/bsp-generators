use serde::{Deserialize, Serialize};

use crate::*;

/// `SbtBuildTarget` is a basic data structure that contains sbt-specific metadata
/// for providing editor support for sbt build files.
///
/// For example, say we have a project in `/foo/bar` defining projects `A` and `B`
/// and two meta builds `M1` (defined in `/foo/bar/project`) and `M2` (defined in
/// `/foo/bar/project/project`).
///
/// The sbt build target for `M1` will have `A` and `B` as the defined targets and
/// `M2` as the parent. Similarly, the sbt build target for `M2` will have `M1` as
/// the defined target and no parent.
///
/// Clients can use this information to reconstruct the tree of sbt meta builds. The
/// `parent` information can be defined from `children` but it's provided by the
/// server to simplify the data processing on the client side.
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SbtBuildTarget {
    pub sbt_version: String,
    pub auto_imports: Vec<String>,
    pub scala_build_target: ScalaBuildTarget,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<BuildTargetIdentifier>,
    pub children: Vec<BuildTargetIdentifier>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;

    use insta::assert_json_snapshot;

    #[test]
    fn sbt_build_target() {
        let test_data = SbtBuildTarget {
            sbt_version: TEST_STRING.to_string(),
            auto_imports: vec![TEST_STRING.to_string()],
            scala_build_target: ScalaBuildTarget::default(),
            parent: Some(BuildTargetIdentifier::default()),
            children: vec![BuildTargetIdentifier::default()],
        };

        assert_json_snapshot!(test_data,
@r#"
{
  "sbtVersion": "test_string",
  "autoImports": [
    "test_string"
  ],
  "scalaBuildTarget": {
    "scalaOrganization": "",
    "scalaVersion": "",
    "scalaBinaryVersion": "",
    "platform": 1,
    "jars": []
  },
  "parent": {
    "uri": ""
  },
  "children": [
    {
      "uri": ""
    }
  ]
}
"#);

        test_deserialization(
            r#"{"sbtVersion": "test_string", "autoImports": ["test_string"], "scalaBuildTarget": {"scalaOrganization": "", "scalaVersion": "", "scalaBinaryVersion": "", "platform": 1, "jars": []}, "parent": {"uri": ""}, "children": [{"uri": ""}]}"#,
            &test_data,
        );
    }
}
