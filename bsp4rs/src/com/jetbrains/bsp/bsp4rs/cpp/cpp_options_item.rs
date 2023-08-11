use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CppOptionsItem {
    /** The target identifier for which the options are requested. */
    pub target: BuildTargetIdentifier,
    /** Attributes added in the given order to COPTS
    before compiling the target.
    For example: -Iexternal/gtest/include */
    pub copts: Vec<String>,
    /** Attributes prepended with -D
    and added to the compile command line
    For example: BOOST_FALLTHROUGH */
    pub defines: Vec<String>,
    /** Attributes added to the linker command
    For example: -pthread */
    pub linkopts: Vec<String>,
    /** Create a shared library.
    The presence of this flag means that linking occurs with the -shared flag */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linkshared: Option<bool>,
}
