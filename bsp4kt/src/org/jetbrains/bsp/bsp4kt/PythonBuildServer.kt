package org.jetbrains.bsp.bsp4kt

import org.jetbrains.jsonrpc4kt.services.JsonNotification
import org.jetbrains.jsonrpc4kt.services.JsonRequest
import kotlinx.serialization.SerialName

interface PythonBuildServer {
  @JsonRequest("buildTarget/pythonOptions")
  suspend fun buildTargetPythonOptions(params: PythonOptionsParams): PythonOptionsResult

}
