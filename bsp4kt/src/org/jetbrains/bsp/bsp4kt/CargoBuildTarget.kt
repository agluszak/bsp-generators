package org.jetbrains.bsp.bsp4kt

import kotlinx.serialization.Serializable
import kotlinx.serialization.SerialName
import kotlinx.serialization.json.JsonElement

@Serializable
data class CargoBuildTarget(
  val edition: String,
  val requiredFeatures: Set<String>
)
