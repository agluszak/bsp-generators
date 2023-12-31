use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RunParams {
    /// The build target to run.
    pub target: BuildTargetIdentifier,
    /// A unique identifier generated by the client to identify this request.
    /// The server may include this id in triggered notifications or responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_id: Option<Identifier>,
    /// Optional arguments to the executed application.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<Vec<String>>,
    /// Optional environment variables to set before running the application.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_variables: Option<EnvironmentVariables>,
    /// Optional working directory
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_directory: Option<URI>,
    /// Language-specific metadata for this execution.
    /// See ScalaMainClass as an example.
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub data: Option<RunParamsData>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;

    use insta::assert_json_snapshot;

    #[test]
    fn run_params() {
        let test_data = RunParams {
            target: BuildTargetIdentifier::default(),
            origin_id: Some(Identifier::default()),
            arguments: Some(vec![TEST_STRING.to_string()]),
            environment_variables: Some(EnvironmentVariables::default()),
            working_directory: Some(URI::default()),
            data: Some(RunParamsData::Other(OtherData::default())),
        };

        assert_json_snapshot!(test_data,
@r#"
{
  "target": {
    "uri": ""
  },
  "originId": "",
  "arguments": [
    "test_string"
  ],
  "environmentVariables": {},
  "workingDirectory": "",
  "dataKind": "",
  "data": null
}
"#);

        test_deserialization(
            r#"{"target": {"uri": ""}, "originId": "", "arguments": ["test_string"], "environmentVariables": {}, "workingDirectory": "", "dataKind": "", "data": null}"#,
            &test_data,
        );
    }
}
