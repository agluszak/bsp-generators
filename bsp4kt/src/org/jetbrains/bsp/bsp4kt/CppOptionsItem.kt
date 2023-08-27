package org.jetbrains.bsp.bsp4kt

import kotlinx.serialization.Serializable
import kotlinx.serialization.SerialName
import kotlinx.serialization.json.JsonElement

@Serializable
data class CppOptionsItem(
  val target: BuildTargetIdentifier,
  val copts: List<String>,
  val defines: List<String>,
  val linkopts: List<String>,
  val linkshared: Boolean? = null
)
