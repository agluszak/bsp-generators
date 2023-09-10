package org.jetbrains.bsp.bsp4kt

import org.jetbrains.jsonrpc4kt.services.JsonNotification
import org.jetbrains.jsonrpc4kt.services.JsonRequest
import kotlinx.serialization.SerialName

interface BuildClient {
  @JsonNotification("build/showMessage")
  fun onBuildShowMessage(params: ShowMessageParams): Unit

  @JsonNotification("build/logMessage")
  fun onBuildLogMessage(params: LogMessageParams): Unit

  @JsonNotification("build/publishDiagnostics")
  fun onBuildPublishDiagnostics(params: PublishDiagnosticsParams): Unit

  @JsonNotification("buildTarget/didChange")
  fun onBuildTargetDidChange(params: DidChangeBuildTarget): Unit

  @JsonNotification("build/taskStart")
  fun onBuildTaskStart(params: TaskStartParams): Unit

  @JsonNotification("build/taskProgress")
  fun onBuildTaskProgress(params: TaskProgressParams): Unit

  @JsonNotification("build/taskFinish")
  fun onBuildTaskFinish(params: TaskFinishParams): Unit

}
