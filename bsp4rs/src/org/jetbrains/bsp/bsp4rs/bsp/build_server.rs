use crate::*;

#[derive(Debug)]
pub enum BuildInitialize {}

impl Request for BuildInitialize {
    type Params = InitializeBuildParams;
    type Result = InitializeBuildResult;
    const METHOD: &'static str = "build/initialize";
}

#[derive(Debug)]
pub enum OnBuildInitialized {}

impl Notification for OnBuildInitialized {
    type Params = ();
    const METHOD: &'static str = "build/initialized";
}

#[derive(Debug)]
pub enum BuildShutdown {}

impl Request for BuildShutdown {
    type Params = ();
    type Result = ();
    const METHOD: &'static str = "build/shutdown";
}

#[derive(Debug)]
pub enum OnBuildExit {}

impl Notification for OnBuildExit {
    type Params = ();
    const METHOD: &'static str = "build/exit";
}

#[derive(Debug)]
pub enum WorkspaceBuildTargets {}

impl Request for WorkspaceBuildTargets {
    type Params = ();
    type Result = WorkspaceBuildTargetsResult;
    const METHOD: &'static str = "workspace/buildTargets";
}

#[derive(Debug)]
pub enum WorkspaceReload {}

impl Request for WorkspaceReload {
    type Params = ();
    type Result = ();
    const METHOD: &'static str = "workspace/reload";
}

#[derive(Debug)]
pub enum BuildTargetSources {}

impl Request for BuildTargetSources {
    type Params = SourcesParams;
    type Result = SourcesResult;
    const METHOD: &'static str = "buildTarget/sources";
}

#[derive(Debug)]
pub enum BuildTargetInverseSources {}

impl Request for BuildTargetInverseSources {
    type Params = InverseSourcesParams;
    type Result = InverseSourcesResult;
    const METHOD: &'static str = "buildTarget/inverseSources";
}

#[derive(Debug)]
pub enum BuildTargetDependencySources {}

impl Request for BuildTargetDependencySources {
    type Params = DependencySourcesParams;
    type Result = DependencySourcesResult;
    const METHOD: &'static str = "buildTarget/dependencySources";
}

#[derive(Debug)]
pub enum BuildTargetDependencyModules {}

impl Request for BuildTargetDependencyModules {
    type Params = DependencyModulesParams;
    type Result = DependencyModulesResult;
    const METHOD: &'static str = "buildTarget/dependencyModules";
}

#[derive(Debug)]
pub enum BuildTargetResources {}

impl Request for BuildTargetResources {
    type Params = ResourcesParams;
    type Result = ResourcesResult;
    const METHOD: &'static str = "buildTarget/resources";
}

#[derive(Debug)]
pub enum BuildTargetOutputPaths {}

impl Request for BuildTargetOutputPaths {
    type Params = OutputPathsParams;
    type Result = OutputPathsResult;
    const METHOD: &'static str = "buildTarget/outputPaths";
}

#[derive(Debug)]
pub enum BuildTargetCompile {}

impl Request for BuildTargetCompile {
    type Params = CompileParams;
    type Result = CompileResult;
    const METHOD: &'static str = "buildTarget/compile";
}

#[derive(Debug)]
pub enum BuildTargetRun {}

impl Request for BuildTargetRun {
    type Params = RunParams;
    type Result = RunResult;
    const METHOD: &'static str = "buildTarget/run";
}

#[derive(Debug)]
pub enum BuildTargetTest {}

impl Request for BuildTargetTest {
    type Params = TestParams;
    type Result = TestResult;
    const METHOD: &'static str = "buildTarget/test";
}

#[derive(Debug)]
pub enum DebugSessionStart {}

impl Request for DebugSessionStart {
    type Params = DebugSessionParams;
    type Result = DebugSessionAddress;
    const METHOD: &'static str = "debugSession/start";
}

#[derive(Debug)]
pub enum BuildTargetCleanCache {}

impl Request for BuildTargetCleanCache {
    type Params = CleanCacheParams;
    type Result = CleanCacheResult;
    const METHOD: &'static str = "buildTarget/cleanCache";
}
