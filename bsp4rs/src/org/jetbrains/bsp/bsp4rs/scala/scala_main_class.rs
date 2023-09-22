use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScalaMainClass {
    /// The main class to run.
    #[serde(rename = "class")]
    pub class_name: String,
    /// The user arguments to the main entrypoint.
    #[deprecated(note = "Use `buildTarget/run` params instead")]
    pub arguments: Vec<String>,
    /// The jvm options for the application.
    pub jvm_options: Vec<String>,
    /// The environment variables for the application.
    #[deprecated(note = "Use `buildTarget/run` params instead")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub environment_variables: Vec<String>,
}
