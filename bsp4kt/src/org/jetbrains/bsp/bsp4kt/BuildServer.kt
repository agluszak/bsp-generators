package org.jetbrains.bsp.bsp4kt

import org.jetbrains.jsonrpc4kt.services.JsonNotification
import org.jetbrains.jsonrpc4kt.services.JsonRequest
import kotlinx.serialization.SerialName

interface BuildServer {
  @JsonRequest("build/initialize")
  suspend fun buildInitialize(params: InitializeBuildParams): InitializeBuildResult

  @JsonNotification("build/initialized")
  fun onBuildInitialized(): Unit

  @JsonRequest("build/shutdown")
  suspend fun buildShutdown(): Unit

  @JsonNotification("build/exit")
  fun onBuildExit(): Unit

  @JsonRequest("workspace/buildTargets")
  suspend fun workspaceBuildTargets(): WorkspaceBuildTargetsResult

  @JsonRequest("workspace/reload")
  suspend fun workspaceReload(): Unit

  @JsonRequest("buildTarget/sources")
  suspend fun buildTargetSources(params: SourcesParams): SourcesResult

  @JsonRequest("buildTarget/inverseSources")
  suspend fun buildTargetInverseSources(params: InverseSourcesParams): InverseSourcesResult

  @JsonRequest("buildTarget/dependencySources")
  suspend fun buildTargetDependencySources(params: DependencySourcesParams): DependencySourcesResult

  @JsonRequest("buildTarget/dependencyModules")
  suspend fun buildTargetDependencyModules(params: DependencyModulesParams): DependencyModulesResult

  @JsonRequest("buildTarget/resources")
  suspend fun buildTargetResources(params: ResourcesParams): ResourcesResult

  @JsonRequest("buildTarget/outputPaths")
  suspend fun buildTargetOutputPaths(params: OutputPathsParams): OutputPathsResult

  @JsonRequest("buildTarget/compile")
  suspend fun buildTargetCompile(params: CompileParams): CompileResult

  @JsonRequest("buildTarget/run")
  suspend fun buildTargetRun(params: RunParams): RunResult

  @JsonRequest("buildTarget/test")
  suspend fun buildTargetTest(params: TestParams): TestResult

  @JsonRequest("debugSession/start")
  suspend fun debugSessionStart(params: DebugSessionParams): DebugSessionAddress

  @JsonRequest("buildTarget/cleanCache")
  suspend fun buildTargetCleanCache(params: CleanCacheParams): CleanCacheResult

  @JsonNotification("run/readStdin")
  fun onRunReadStdin(params: ReadParams): Unit

}
