use crate::*;

#[derive(Debug)]
pub enum BuildInitialize {}

/// Like the language server protocol, the initialize request is sent as the first request from the client to the server.
/// If the server receives a request or notification before the initialize request it should act as follows:
///
/// * For a request the response should be an error with code: -32002. The message can be picked by the server.
/// * Notifications should be dropped, except for the exit notification. This will allow the exit of a server without an initialize request.
///
/// Until the server has responded to the initialize request with an InitializeBuildResult, the client must not send any additional
/// requests or notifications to the server.
impl Request for BuildInitialize {
    type Params = InitializeBuildParams;
    type Result = InitializeBuildResult;
    const METHOD: &'static str = "build/initialize";
}

#[derive(Debug)]
pub enum OnBuildInitialized {}

/// Like the language server protocol, the initialized notification is sent from the
/// client to the server after the client received the result of the initialize
/// request but before the client is sending any other request or notification to
/// the server. The server can use the initialized notification for example to
/// initialize intensive computation such as dependency resolution or compilation.
/// The initialized notification may only be sent once.
impl Notification for OnBuildInitialized {
    type Params = ();
    const METHOD: &'static str = "build/initialized";
}

#[derive(Debug)]
pub enum BuildShutdown {}

/// Like the language server protocol, the shutdown build request is sent from the
/// client to the server. It asks the server to shut down, but to not exit
/// (otherwise the response might not be delivered correctly to the client). There
/// is a separate exit notification that asks the server to exit.
impl Request for BuildShutdown {
    type Params = ();
    type Result = ();
    const METHOD: &'static str = "build/shutdown";
}

#[derive(Debug)]
pub enum OnBuildExit {}

/// Like the language server protocol, a notification to ask the server to exit its process. The server should exit with success code 0
/// if the shutdown request has been received before; otherwise with error code 1.
impl Notification for OnBuildExit {
    type Params = ();
    const METHOD: &'static str = "build/exit";
}

#[derive(Debug)]
pub enum WorkspaceBuildTargets {}

/// The workspace build targets request is sent from the client to the server to ask
/// for the list of all available build targets in the workspace.
impl Request for WorkspaceBuildTargets {
    type Params = ();
    type Result = WorkspaceBuildTargetsResult;
    const METHOD: &'static str = "workspace/buildTargets";
}

#[derive(Debug)]
pub enum WorkspaceReload {}

/// The `reload` request is sent from the client to instruct the build server to reload
/// the build configuration. This request should be supported by build tools that keep
/// their state in memory. If the `reload` request returns with an error, it's expected
/// that other requests respond with the previously known "good" state.
impl Request for WorkspaceReload {
    type Params = ();
    type Result = ();
    const METHOD: &'static str = "workspace/reload";
}

#[derive(Debug)]
pub enum BuildTargetSources {}

/// The build target sources request is sent from the client to the server to query
/// for the list of text documents and directories that are belong to a build
/// target. The sources response must not include sources that are external to the
/// workspace, see `buildTarget/dependencySources`.
impl Request for BuildTargetSources {
    type Params = SourcesParams;
    type Result = SourcesResult;
    const METHOD: &'static str = "buildTarget/sources";
}

#[derive(Debug)]
pub enum BuildTargetInverseSources {}

/// The inverse sources request is sent from the client to the server to query for
/// the list of build targets containing a text document. The server communicates
/// during the initialize handshake whether this method is supported or not. This
/// request can be viewed as the inverse of `buildTarget/sources`, except it only
/// works for text documents and not directories.
impl Request for BuildTargetInverseSources {
    type Params = InverseSourcesParams;
    type Result = InverseSourcesResult;
    const METHOD: &'static str = "buildTarget/inverseSources";
}

#[derive(Debug)]
pub enum BuildTargetDependencySources {}

/// The build target dependency sources request is sent from the client to the
/// server to query for the sources of build target dependencies that are external
/// to the workspace. The dependency sources response must not include source files
/// that belong to a build target within the workspace, see `buildTarget/sources`.
///
/// The server communicates during the initialize handshake whether this method is
/// supported or not. This method can for example be used by a language server on
/// `textDocument/definition` to "Go to definition" from project sources to
/// dependency sources.
impl Request for BuildTargetDependencySources {
    type Params = DependencySourcesParams;
    type Result = DependencySourcesResult;
    const METHOD: &'static str = "buildTarget/dependencySources";
}

#[derive(Debug)]
pub enum BuildTargetDependencyModules {}

/// The build target dependency modules request is sent from the client to the
/// server to query for the libraries of build target dependencies that are external
/// to the workspace including meta information about library and their sources.
/// It's an extended version of `buildTarget/sources`.
impl Request for BuildTargetDependencyModules {
    type Params = DependencyModulesParams;
    type Result = DependencyModulesResult;
    const METHOD: &'static str = "buildTarget/dependencyModules";
}

#[derive(Debug)]
pub enum BuildTargetResources {}

/// The build target resources request is sent from the client to the server to
/// query for the list of resources of a given list of build targets.
///
/// A resource is a data dependency required to be present in the runtime classpath
/// when a build target is run or executed. The server communicates during the
/// initialize handshake whether this method is supported or not.
///
/// This request can be used by a client to highlight the resources in a project
/// view, for example.
impl Request for BuildTargetResources {
    type Params = ResourcesParams;
    type Result = ResourcesResult;
    const METHOD: &'static str = "buildTarget/resources";
}

#[derive(Debug)]
pub enum BuildTargetOutputPaths {}

/// The build target output paths request is sent from the client to the server to
/// query for the list of output paths of a given list of build targets.
///
/// An output path is a file or directory that contains output files such as build
/// artifacts which IDEs may decide to exclude from indexing. The server communicates
/// during the initialize handshake whether this method is supported or not.
impl Request for BuildTargetOutputPaths {
    type Params = OutputPathsParams;
    type Result = OutputPathsResult;
    const METHOD: &'static str = "buildTarget/outputPaths";
}

#[derive(Debug)]
pub enum BuildTargetCompile {}

/// The compile build target request is sent from the client to the server to
/// compile the given list of build targets. The server communicates during the
/// initialize handshake whether this method is supported or not. This method can
/// for example be used by a language server before `textDocument/rename` to ensure
/// that all workspace sources typecheck correctly and are up-to-date.
impl Request for BuildTargetCompile {
    type Params = CompileParams;
    type Result = CompileResult;
    const METHOD: &'static str = "buildTarget/compile";
}

#[derive(Debug)]
pub enum BuildTargetRun {}

/// The run request is sent from the client to the server to run a build target. The
/// server communicates during the initialize handshake whether this method is
/// supported or not.
///
/// This request may trigger a compilation on the selected build targets. The server
/// is free to send any number of `build/task*`, `build/publishDiagnostics` and
/// `build/logMessage` notifications during compilation before completing the
/// response.
///
/// The client will get a `originId` field in `RunResult` if the `originId` field in
/// the `RunParams` is defined.
///
/// Note that an empty run request is valid. Run will be executed in the target as
/// specified in the build tool.
impl Request for BuildTargetRun {
    type Params = RunParams;
    type Result = RunResult;
    const METHOD: &'static str = "buildTarget/run";
}

#[derive(Debug)]
pub enum BuildTargetTest {}

/// The test build target request is sent from the client to the server to test the
/// given list of build targets. The server communicates during the initialize
/// handshake whether this method is supported or not.
impl Request for BuildTargetTest {
    type Params = TestParams;
    type Result = TestResult;
    const METHOD: &'static str = "buildTarget/test";
}

#[derive(Debug)]
pub enum DebugSessionStart {}

/// The debug request is sent from the client to the server to debug build target(s). The
/// server launches a [Microsoft DAP](https://microsoft.github.io/debug-adapter-protocol/) server
/// and returns a connection URI for the client to interact with.
impl Request for DebugSessionStart {
    type Params = DebugSessionParams;
    type Result = DebugSessionAddress;
    const METHOD: &'static str = "debugSession/start";
}

#[derive(Debug)]
pub enum BuildTargetCleanCache {}

/// The clean cache request is sent from the client to the server to reset any state
/// associated with a given build target. The state can live either in the build
/// tool or in the file system.
///
/// The build tool defines the exact semantics of the clean cache request:
///
/// 1. Stateless build tools are free to ignore the request and respond with a
///    successful response.
/// 2. Stateful build tools must ensure that invoking compilation on a target that
///    has been cleaned results in a full compilation.
impl Request for BuildTargetCleanCache {
    type Params = CleanCacheParams;
    type Result = CleanCacheResult;
    const METHOD: &'static str = "buildTarget/cleanCache";
}

#[derive(Debug)]
pub enum OnRunReadStdin {}

/// Notification sent from the client to the server when the user wants to send
/// input to the stdin of the running target.
impl Notification for OnRunReadStdin {
    type Params = ReadParams;
    const METHOD: &'static str = "run/readStdin";
}
