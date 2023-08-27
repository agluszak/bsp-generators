package org.jetbrains.bsp.bsp4kt

import org.jetbrains.jsonrpc4kt.services.JsonNotification
import org.jetbrains.jsonrpc4kt.services.JsonRequest
import kotlinx.serialization.SerialName
import java.util.concurrent.CompletableFuture

interface CppBuildServer {
  @JsonRequest("buildTarget/cppOptions")
  fun buildTargetCppOptions(params: CppOptionsParams): CompletableFuture<CppOptionsResult>

}
