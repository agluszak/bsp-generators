package org.jetbrains.bsp.bsp4kt

import org.jetbrains.jsonrpc4kt.services.JsonNotification
import org.jetbrains.jsonrpc4kt.services.JsonRequest
import kotlinx.serialization.SerialName

interface CppBuildServer {
  @JsonRequest("buildTarget/cppOptions")
  fun suspend buildTargetCppOptions(params: CppOptionsParams): CppOptionsResult

}
