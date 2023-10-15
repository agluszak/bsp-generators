use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RustWorkspaceResult {
    /// Packages of given targets.
    pub packages: Vec<RustPackage>,
    /// Dependencies as listed in the package `Cargo.toml`,
    /// without package resolution or any additional data.
    pub raw_dependencies: RustRawDependencies,
    /// Resolved dependencies of the package. Handles renamed dependencies.
    pub dependencies: RustDependencies,
    /// A sequence of build targets taken into consideration during build process.
    pub resolved_targets: Vec<BuildTargetIdentifier>,
}

#[cfg(test)]
mod tests {
    use insta::assert_json_snapshot;

    use super::*;

    #[test]
    fn rust_workspace_result() {
        assert_json_snapshot!(
           RustWorkspaceResult {packages: vec![RustPackage::default()], raw_dependencies: RustRawDependencies::default(), dependencies: RustDependencies::default(), resolved_targets: vec![BuildTargetIdentifier::default()]},
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
   "#
        );
    }
}
