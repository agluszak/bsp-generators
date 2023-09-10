package org.jetbrains.bsp.bsp4kt

import org.jetbrains.jsonrpc4kt.services.JsonNotification
import org.jetbrains.jsonrpc4kt.services.JsonRequest
import kotlinx.serialization.SerialName

interface CppBuildServer {
  @JsonRequest("buildTarget/cppOptions")
  suspend fun buildTargetCppOptions(params: CppOptionsParams): CppOptionsResult

}
