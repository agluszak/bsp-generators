package org.jetbrains.bsp.bsp4kt

import org.jetbrains.jsonrpc4kt.services.JsonNotification
import org.jetbrains.jsonrpc4kt.services.JsonRequest
import kotlinx.serialization.SerialName

interface JvmBuildServer {
  @JsonRequest("buildTarget/jvmTestEnvironment")
  fun suspend buildTargetJvmTestEnvironment(params: JvmTestEnvironmentParams): JvmTestEnvironmentResult

  @JsonRequest("buildTarget/jvmRunEnvironment")
  fun suspend buildTargetJvmRunEnvironment(params: JvmRunEnvironmentParams): JvmRunEnvironmentResult

}
