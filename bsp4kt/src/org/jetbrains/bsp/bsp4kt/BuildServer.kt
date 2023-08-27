package org.jetbrains.bsp.bsp4kt

import org.jetbrains.jsonrpc4kt.services.JsonNotification
import org.jetbrains.jsonrpc4kt.services.JsonRequest
import kotlinx.serialization.SerialName
import java.util.concurrent.CompletableFuture

interface BuildServer {
  @JsonRequest("build/initialize")
  fun buildInitialize(params: InitializeBuildParams): CompletableFuture<InitializeBuildResult>

  @JsonNotification("build/initialized")
  fun onBuildInitialized(): Unit

  @JsonRequest("build/shutdown")
  fun buildShutdown(): CompletableFuture<Unit>

  @JsonNotification("build/exit")
  fun onBuildExit(): Unit

  @JsonRequest("workspace/buildTargets")
  fun workspaceBuildTargets(): CompletableFuture<WorkspaceBuildTargetsResult>

  @JsonRequest("workspace/reload")
  fun workspaceReload(): CompletableFuture<Unit>

  @JsonRequest("buildTarget/sources")
  fun buildTargetSources(params: SourcesParams): CompletableFuture<SourcesResult>

  @JsonRequest("buildTarget/inverseSources")
  fun buildTargetInverseSources(params: InverseSourcesParams): CompletableFuture<InverseSourcesResult>

  @JsonRequest("buildTarget/dependencySources")
  fun buildTargetDependencySources(params: DependencySourcesParams): CompletableFuture<DependencySourcesResult>

  @JsonRequest("buildTarget/dependencyModules")
  fun buildTargetDependencyModules(params: DependencyModulesParams): CompletableFuture<DependencyModulesResult>

  @JsonRequest("buildTarget/resources")
  fun buildTargetResources(params: ResourcesParams): CompletableFuture<ResourcesResult>

  @JsonRequest("buildTarget/outputPaths")
  fun buildTargetOutputPaths(params: OutputPathsParams): CompletableFuture<OutputPathsResult>

  @JsonRequest("buildTarget/compile")
  fun buildTargetCompile(params: CompileParams): CompletableFuture<CompileResult>

  @JsonRequest("buildTarget/run")
  fun buildTargetRun(params: RunParams): CompletableFuture<RunResult>

  @JsonRequest("buildTarget/test")
  fun buildTargetTest(params: TestParams): CompletableFuture<TestResult>

  @JsonRequest("debugSession/start")
  fun debugSessionStart(params: DebugSessionParams): CompletableFuture<DebugSessionAddress>

  @JsonRequest("buildTarget/cleanCache")
  fun buildTargetCleanCache(params: CleanCacheParams): CompletableFuture<CleanCacheResult>

}
