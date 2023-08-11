use serde::{Deserialize, Serialize};

/** A list of predefined tags that can be used to categorize build targets. */
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum BuildTargetTag {
    /** A list of predefined tags that can be used to categorize build targets. */
    Application,
    /** A list of predefined tags that can be used to categorize build targets. */
    Benchmark,
    /** A list of predefined tags that can be used to categorize build targets. */
    IntegrationTest,
    /** A list of predefined tags that can be used to categorize build targets. */
    Library,
    /** A list of predefined tags that can be used to categorize build targets. */
    Manual,
    /** A list of predefined tags that can be used to categorize build targets. */
    NoIde,
    /** A list of predefined tags that can be used to categorize build targets. */
    Test,
}
