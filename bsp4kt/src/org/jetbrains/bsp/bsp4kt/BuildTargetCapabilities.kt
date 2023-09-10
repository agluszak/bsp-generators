package org.jetbrains.bsp.bsp4kt

import kotlinx.serialization.Serializable
import kotlinx.serialization.SerialName
import kotlinx.serialization.json.JsonElement

@Serializable
data class BuildTargetCapabilities(
  val canCompile: Boolean? = null,
  val canTest: Boolean? = null,
  val canRun: Boolean? = null,
  val canDebug: Boolean? = null
)
