package org.jetbrains.bsp.bsp4kt

import org.jetbrains.jsonrpc4kt.services.JsonNotification
import org.jetbrains.jsonrpc4kt.services.JsonRequest
import kotlinx.serialization.SerialName
import java.util.concurrent.CompletableFuture

interface JavaBuildServer {
  @JsonRequest("buildTarget/javacOptions")
  fun buildTargetJavacOptions(params: JavacOptionsParams): CompletableFuture<JavacOptionsResult>

}
