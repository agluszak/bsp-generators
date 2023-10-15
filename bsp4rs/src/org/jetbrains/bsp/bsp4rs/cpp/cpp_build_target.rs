use serde::{Deserialize, Serialize};

use crate::*;

/// `CppBuildTarget` is a basic data structure that contains c++-specific
/// metadata, specifically compiler reference.
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CppBuildTarget {
    /// The c++ version this target is supposed to use.
    /// For example: C++11
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// The type of compiler this target is supposed to use.
    /// For example: gcc
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compiler: Option<String>,
    /// Uri representating path to the c compiler.
    /// For example: file:///usr/bin/gcc
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_compiler: Option<URI>,
    /// Uri representating path to the c++ compiler.
    /// For example: file:///usr/bin/g++
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpp_compiler: Option<URI>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;
    use insta::assert_json_snapshot;

    #[test]
    fn cpp_build_target() {
        assert_json_snapshot!(
           CppBuildTarget {version: Some(TEST_STRING.to_string()), compiler: Some(TEST_STRING.to_string()), c_compiler: Some(URI::default()), cpp_compiler: Some(URI::default())},
           @r#"
{
  "version": "test_string",
  "compiler": "test_string",
  "cCompiler": "",
  "cppCompiler": ""
}
   "#
        );
    }
}
