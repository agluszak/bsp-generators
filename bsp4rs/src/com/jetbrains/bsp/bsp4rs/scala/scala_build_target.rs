use serde::{Deserialize, Serialize};

use crate::*;

/** `ScalaBuildTarget` is a basic data structure that contains scala-specific
metadata for compiling a target containing Scala sources. */
#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScalaBuildTarget {
    /** The Scala organization that is used for a target. */
    pub scala_organization: String,
    /** The scala version to compile this target */
    pub scala_version: String,
    /** The binary version of scalaVersion.
    For example, 2.12 if scalaVersion is 2.12.4. */
    pub scala_binary_version: String,
    /** The target platform for this target */
    pub platform: ScalaPlatform,
    /** A sequence of Scala jars such as scala-library, scala-compiler and scala-reflect. */
    pub jars: Vec<URI>,
    /** The jvm build target describing jdk to be used */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jvm_build_target: Option<JvmBuildTarget>,
}
