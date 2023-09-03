use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CppOptionsItem {
    /** The target identifier for which the options are requested. */
    #[serde(default)]
    pub target: BuildTargetIdentifier,
    /** Attributes added in the given order to COPTS
    before compiling the target.
    For example: -Iexternal/gtest/include */
    #[serde(default)]
    pub copts: Vec<String>,
    /** Attributes prepended with -D
    and added to the compile command line
    For example: BOOST_FALLTHROUGH */
    #[serde(default)]
    pub defines: Vec<String>,
    /** Attributes added to the linker command
    For example: -pthread */
    #[serde(default)]
    pub linkopts: Vec<String>,
    /** Create a shared library.
    The presence of this flag means that linking occurs with the -shared flag */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linkshared: Option<bool>,
}
