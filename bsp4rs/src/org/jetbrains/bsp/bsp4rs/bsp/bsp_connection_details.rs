use serde::{Deserialize, Serialize};

/// Structure describing how to start a BSP server and the capabilities it supports.
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BspConnectionDetails {
    /// The name of the BSP server.
    pub name: String,
    /// Arguments to pass to the BSP server.
    pub argv: Vec<String>,
    /// The version of the BSP server.
    pub version: String,
    /// Supported BSP version.
    pub bsp_version: String,
    /// The languages supported by the BSP server.
    pub languages: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;

    use insta::assert_json_snapshot;

    #[test]
    fn bsp_connection_details() {
        let test_data = BspConnectionDetails {
            name: TEST_STRING.to_string(),
            argv: vec![String::default()],
            version: TEST_STRING.to_string(),
            bsp_version: TEST_STRING.to_string(),
            languages: vec![String::default()],
        };

        assert_json_snapshot!(test_data,
@r#"
{
  "name": "test_string",
  "argv": [
    ""
  ],
  "version": "test_string",
  "bspVersion": "test_string",
  "languages": [
    ""
  ]
}
"#);

        test_deserialization(
            r#"{"name": "test_string", "argv": [""], "version": "test_string", "bspVersion": "test_string", "languages": [""]}"#,
            &test_data,
        );
    }
}
