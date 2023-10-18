use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RustWorkspaceResult {
    /// Packages of given targets.
    pub packages: Vec<RustPackage>,
    /// Dependencies in `cargo metadata` as listed in the package `Cargo.toml`,
    /// without package resolution or any additional data.
    pub raw_dependencies: RustRawDependencies,
    /// Resolved dependencies of the build. Handles renamed dependencies.
    /// Correspond to dependencies from resolved dependency graph from `cargo metadata` that shows
    /// the actual dependencies that are being used in the build.
    pub dependencies: RustDependencies,
    /// A sequence of build targets taken into consideration during build process.
    pub resolved_targets: Vec<BuildTargetIdentifier>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;

    use insta::assert_json_snapshot;

    #[test]
    fn rust_workspace_result() {
        let test_data = RustWorkspaceResult {
            packages: vec![RustPackage::default()],
            raw_dependencies: RustRawDependencies::default(),
            dependencies: RustDependencies::default(),
            resolved_targets: vec![BuildTargetIdentifier::default()],
        };

        assert_json_snapshot!(test_data,
@r#"
{
  "packages": [
    {
      "id": "",
      "rootUrl": "",
      "name": "",
      "version": "",
      "origin": "",
      "edition": "",
      "resolvedTargets": [],
      "allTargets": [],
      "features": {},
      "enabledFeatures": []
    }
  ],
  "rawDependencies": {},
  "dependencies": {},
  "resolvedTargets": [
    {
      "uri": ""
    }
  ]
}
"#);

        test_deserialization(
            r#"{"packages": [{"id": "", "rootUrl": "", "name": "", "version": "", "origin": "", "edition": "", "resolvedTargets": [], "allTargets": [], "features": {}, "enabledFeatures": []}], "rawDependencies": {}, "dependencies": {}, "resolvedTargets": [{"uri": ""}]}"#,
            &test_data,
        );
    }
}
