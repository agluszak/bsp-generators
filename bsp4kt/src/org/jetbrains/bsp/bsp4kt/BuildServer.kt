package org.jetbrains.bsp.bsp4kt

import org.jetbrains.jsonrpc4kt.services.JsonNotification
import org.jetbrains.jsonrpc4kt.services.JsonRequest
import kotlinx.serialization.SerialName

interface BuildServer {
  @JsonRequest("build/initialize")
  fun suspend buildInitialize(params: InitializeBuildParams): InitializeBuildResult

  @JsonNotification("build/initialized")
  fun onBuildInitialized(): Unit

  @JsonRequest("build/shutdown")
  fun suspend buildShutdown(): Unit

  @JsonNotification("build/exit")
  fun onBuildExit(): Unit

  @JsonRequest("workspace/buildTargets")
  fun suspend workspaceBuildTargets(): WorkspaceBuildTargetsResult

  @JsonRequest("workspace/reload")
  fun suspend workspaceReload(): Unit

  @JsonRequest("buildTarget/sources")
  fun suspend buildTargetSources(params: SourcesParams): SourcesResult

  @JsonRequest("buildTarget/inverseSources")
  fun suspend buildTargetInverseSources(params: InverseSourcesParams): InverseSourcesResult

  @JsonRequest("buildTarget/dependencySources")
  fun suspend buildTargetDependencySources(params: DependencySourcesParams): DependencySourcesResult

  @JsonRequest("buildTarget/dependencyModules")
  fun suspend buildTargetDependencyModules(params: DependencyModulesParams): DependencyModulesResult

  @JsonRequest("buildTarget/resources")
  fun suspend buildTargetResources(params: ResourcesParams): ResourcesResult

  @JsonRequest("buildTarget/outputPaths")
  fun suspend buildTargetOutputPaths(params: OutputPathsParams): OutputPathsResult

  @JsonRequest("buildTarget/compile")
  fun suspend buildTargetCompile(params: CompileParams): CompileResult

  @JsonRequest("buildTarget/run")
  fun suspend buildTargetRun(params: RunParams): RunResult

  @JsonRequest("buildTarget/test")
  fun suspend buildTargetTest(params: TestParams): TestResult

  @JsonRequest("debugSession/start")
  fun suspend debugSessionStart(params: DebugSessionParams): DebugSessionAddress

  @JsonRequest("buildTarget/cleanCache")
  fun suspend buildTargetCleanCache(params: CleanCacheParams): CleanCacheResult

}
