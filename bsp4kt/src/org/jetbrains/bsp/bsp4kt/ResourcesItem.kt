package org.jetbrains.bsp.bsp4kt

import kotlinx.serialization.Serializable
import kotlinx.serialization.SerialName
import kotlinx.serialization.json.JsonElement

@Serializable
data class ResourcesItem(
  val target: BuildTargetIdentifier,
  val resources: List<URI>
)
