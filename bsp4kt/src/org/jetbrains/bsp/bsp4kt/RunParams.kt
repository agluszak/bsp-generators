package org.jetbrains.bsp.bsp4kt

import kotlinx.serialization.Serializable
import kotlinx.serialization.SerialName
import kotlinx.serialization.json.JsonElement

@Serializable
data class RunParams(
  val target: BuildTargetIdentifier,
  val originId: String? = null,
  val arguments: List<String>? = null,
  val environmentVariables: Map<String, String>? = null,
  val workingDirectory: String? = null,
  val dataKind: String? = null,
  val data: JsonElement? = null
)
