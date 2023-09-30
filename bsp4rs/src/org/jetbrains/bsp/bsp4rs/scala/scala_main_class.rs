use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScalaMainClass {
    /// The main class to run.
    #[serde(rename = "class")]
    pub class_name: String,
    /// The user arguments to the main entrypoint.
    pub arguments: Vec<String>,
    /// The jvm options for the application.
    pub jvm_options: Vec<String>,
    /// The environment variables for the application.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_variables: Option<Vec<String>>,
}
