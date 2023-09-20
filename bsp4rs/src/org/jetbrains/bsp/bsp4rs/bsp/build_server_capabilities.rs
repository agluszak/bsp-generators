use serde::{Deserialize, Serialize};

use crate::*;

/// The capabilities of the build server.
/// Clients can use these capabilities to notify users what BSP endpoints can and
/// cannot be used and why.
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BuildServerCapabilities {
    /// The languages the server supports compilation via method buildTarget/compile.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compile_provider: Option<CompileProvider>,
    /// The languages the server supports test execution via method buildTarget/test.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub test_provider: Option<TestProvider>,
    /// The languages the server supports run via method buildTarget/run.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub run_provider: Option<RunProvider>,
    /// The languages the server supports debugging via method debugSession/start.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub debug_provider: Option<DebugProvider>,
    /// The server can provide a list of targets that contain a
    /// single text document via the method buildTarget/inverseSources
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inverse_sources_provider: Option<bool>,
    /// The server provides sources for library dependencies
    /// via method buildTarget/dependencySources
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dependency_sources_provider: Option<bool>,
    /// The server can provide a list of dependency modules (libraries with meta information)
    /// via method buildTarget/dependencyModules
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dependency_modules_provider: Option<bool>,
    /// The server provides all the resource dependencies
    /// via method buildTarget/resources
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources_provider: Option<bool>,
    /// The server provides all output paths
    /// via method buildTarget/outputPaths
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output_paths_provider: Option<bool>,
    /// The server sends notifications to the client on build
    /// target change events via buildTarget/didChange
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub build_target_changed_provider: Option<bool>,
    /// The server can respond to `buildTarget/jvmRunEnvironment` requests with the
    /// necessary information required to launch a Java process to run a main class.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub jvm_run_environment_provider: Option<bool>,
    /// The server can respond to `buildTarget/jvmTestEnvironment` requests with the
    /// necessary information required to launch a Java process for testing or
    /// debugging.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub jvm_test_environment_provider: Option<bool>,
    /// The server can respond to `workspace/cargoFeaturesState` and
    /// `setCargoFeatures` requests. In other words, supports Cargo Features extension.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cargo_features_provider: Option<bool>,
    /// Reloading the build state through workspace/reload is supported
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_reload: Option<bool>,
}
