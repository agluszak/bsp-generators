package org.jetbrains.bsp.bsp4kt

import org.jetbrains.jsonrpc4kt.services.JsonNotification
import org.jetbrains.jsonrpc4kt.services.JsonRequest
import kotlinx.serialization.SerialName

interface ScalaBuildServer {
  @JsonRequest("buildTarget/scalacOptions")
  fun suspend buildTargetScalacOptions(params: ScalacOptionsParams): ScalacOptionsResult

  @Deprecated("Use buildTarget/jvmTestEnvironment instead")
  @JsonRequest("buildTarget/scalaTestClasses")
  fun suspend buildTargetScalaTestClasses(params: ScalaTestClassesParams): ScalaTestClassesResult

  @Deprecated("Use buildTarget/jvmRunEnvironment instead")
  @JsonRequest("buildTarget/scalaMainClasses")
  fun suspend buildTargetScalaMainClasses(params: ScalaMainClassesParams): ScalaMainClassesResult

}
