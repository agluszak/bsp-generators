package org.jetbrains.bsp.bsp4kt

import kotlinx.serialization.Serializable
import kotlinx.serialization.SerialName
import kotlinx.serialization.json.JsonElement

@Serializable
data class ScalaMainClass(
  @SerialName("class")
val className: String,
  val arguments: List<String>,
  val jvmOptions: List<String>,
  val environmentVariables: List<String>? = null
)
