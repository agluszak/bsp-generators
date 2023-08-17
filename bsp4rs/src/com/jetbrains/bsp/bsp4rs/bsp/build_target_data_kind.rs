use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct BuildTargetDataKind(pub std::borrow::Cow<'static, str>);
impl BuildTargetDataKind {
    /** `data` field must contain a CppBuildTarget object. */
    pub const CPP: BuildTargetDataKind = BuildTargetDataKind::new("cpp");
    /** `data` field must contain a JvmBuildTarget object. */
    pub const JVM: BuildTargetDataKind = BuildTargetDataKind::new("jvm");
    /** `data` field must contain a PythonBuildTarget object. */
    pub const PYTHON: BuildTargetDataKind = BuildTargetDataKind::new("python");
    /** `data` field must contain a SbtBuildTarget object. */
    pub const SBT: BuildTargetDataKind = BuildTargetDataKind::new("sbt");
    /** `data` field must contain a ScalaBuildTarget object. */
    pub const SCALA: BuildTargetDataKind = BuildTargetDataKind::new("scala");

    pub const fn new(tag: &'static str) -> Self {
        BuildTargetDataKind(std::borrow::Cow::Borrowed(tag))
    }
}
