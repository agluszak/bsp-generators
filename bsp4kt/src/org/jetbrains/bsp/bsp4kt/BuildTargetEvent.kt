package org.jetbrains.bsp.bsp4kt

import kotlinx.serialization.Serializable
import kotlinx.serialization.SerialName
import kotlinx.serialization.json.JsonElement

@Serializable
data class BuildTargetEvent(
  val target: BuildTargetIdentifier,
  val kind: BuildTargetEventKind? = null,
  val dataKind: String? = null,
  val data: JsonElement? = null
)
