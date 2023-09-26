package org.jetbrains.bsp.bsp4kt

import kotlinx.serialization.Serializable
import kotlinx.serialization.SerialName
import kotlinx.serialization.json.JsonElement

@Serializable
data class JvmBuildTarget(
  val javaHome: URI? = null,
  val javaVersion: String? = null
)
