package org.jetbrains.bsp.bsp4kt

import org.jetbrains.jsonrpc4kt.services.JsonNotification
import org.jetbrains.jsonrpc4kt.services.JsonRequest
import kotlinx.serialization.SerialName

interface RustBuildServer {
  @JsonRequest("buildTarget/rustWorkspace")
  suspend fun rustWorkspace(params: RustWorkspaceParams): RustWorkspaceResult

  @JsonRequest("buildTarget/rustToolchain")
  suspend fun rustToolchain(params: RustToolchainParams): RustToolchainResult

}
