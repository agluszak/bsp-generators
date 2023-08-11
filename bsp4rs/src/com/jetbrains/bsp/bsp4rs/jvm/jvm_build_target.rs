use serde::{Deserialize, Serialize};

/** `JvmBuildTarget` is a basic data structure that contains jvm-specific
metadata, specifically JDK reference. */
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct JvmBuildTarget {
    /** Uri representing absolute path to jdk
    For example: file:///usr/lib/jvm/java-8-openjdk-amd64 */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub java_home: Option<String>,
    /** The java version this target is supposed to use.
    For example: 1.8 */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub java_version: Option<String>,
}
