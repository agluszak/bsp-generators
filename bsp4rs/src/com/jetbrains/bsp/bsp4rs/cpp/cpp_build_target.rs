use serde::{Deserialize, Serialize};

use crate::*;

/** `CppBuildTarget` is a basic data structure that contains c++-specific
metadata, specifically compiler reference. */
#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CppBuildTarget {
    /** The c++ version this target is supposed to use.
    For example: C++11 */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /** The type of compiler this target is supposed to use.
    For example: gcc */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compiler: Option<String>,
    /** Uri representating path to the c compiler.
    For example: file:///usr/bin/gcc */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_compiler: Option<URI>,
    /** Uri representating path to the c++ compiler.
    For example: file:///usr/bin/g++ */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpp_compiler: Option<URI>,
}
