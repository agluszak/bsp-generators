package org.jetbrains.bsp.bsp4kt

import com.jetbrains.jsonrpc4kt.services.JsonNotification
import com.jetbrains.jsonrpc4kt.services.JsonRequest
import kotlinx.serialization.SerialName
import java.util.concurrent.CompletableFuture

interface PythonBuildServer {
  @JsonRequest("buildTarget/pythonOptions")
  fun buildTargetPythonOptions(params: PythonOptionsParams): CompletableFuture<PythonOptionsResult>

}
