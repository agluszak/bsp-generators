use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScalaMainClass {
    /** The main class to run. */
    #[serde(default)]
    pub class_name: String,
    /** The user arguments to the main entrypoint. */
    #[serde(default)]
    pub arguments: Vec<String>,
    /** The jvm options for the application. */
    #[serde(default)]
    pub jvm_options: Vec<String>,
    /** The environment variables for the application. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub environment_variables: Vec<String>,
}
