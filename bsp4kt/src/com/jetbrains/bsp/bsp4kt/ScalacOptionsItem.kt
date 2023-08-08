package com.jetbrains.bsp.bsp4kt

import kotlinx.serialization.Serializable
import kotlinx.serialization.SerialName
import kotlinx.serialization.json.JsonElement

@Serializable
data class ScalacOptionsItem(
  val target: BuildTargetIdentifier,
  val options: List<String>,
  val classpath: List<String>,
  val classDirectory: String
)
