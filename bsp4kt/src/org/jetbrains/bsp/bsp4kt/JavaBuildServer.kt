package org.jetbrains.bsp.bsp4kt

import org.jetbrains.jsonrpc4kt.services.JsonNotification
import org.jetbrains.jsonrpc4kt.services.JsonRequest
import kotlinx.serialization.SerialName

interface JavaBuildServer {
  @JsonRequest("buildTarget/javacOptions")
  suspend fun buildTargetJavacOptions(params: JavacOptionsParams): JavacOptionsResult

}
