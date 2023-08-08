package com.jetbrains.bsp.bsp4kt

import kotlinx.serialization.Serializable
import kotlinx.serialization.SerialName
import kotlinx.serialization.json.JsonElement

@Serializable
data class ScalaMainClass(
  val className: String,
  val arguments: List<String>,
  val jvmOptions: List<String>,
  val environmentVariables: List<String>?
)
