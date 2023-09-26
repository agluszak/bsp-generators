package org.jetbrains.bsp.bsp4kt

import kotlinx.serialization.Serializable
import kotlinx.serialization.SerialName
import kotlinx.serialization.json.JsonElement

@Serializable
data class CppBuildTarget(
  val version: String? = null,
  val compiler: String? = null,
  val cCompiler: URI? = null,
  val cppCompiler: URI? = null
)
