package org.jetbrains.bsp.bsp4kt

import kotlinx.serialization.Serializable
import kotlinx.serialization.SerialName
import kotlinx.serialization.json.JsonElement

@Serializable
data class RunParams(
  val target: BuildTargetIdentifier,
  val originId: Identifier? = null,
  val arguments: List<String>? = null,
  val environmentVariables: EnvironmentVariables? = null,
  val workingDirectory: URI? = null,
  val data: RunParamsData? = null
)
