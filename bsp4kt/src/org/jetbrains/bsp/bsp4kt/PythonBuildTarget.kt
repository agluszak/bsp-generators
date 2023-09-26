package org.jetbrains.bsp.bsp4kt

import kotlinx.serialization.Serializable
import kotlinx.serialization.SerialName
import kotlinx.serialization.json.JsonElement

@Serializable
data class PythonBuildTarget(
  val version: String? = null,
  val interpreter: URI? = null
)
