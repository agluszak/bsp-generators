package org.jetbrains.bsp.bsp4kt

import kotlinx.serialization.Serializable
import kotlinx.serialization.SerialName
import kotlinx.serialization.json.JsonElement

@Serializable
data class JvmEnvironmentItem(
  val target: BuildTargetIdentifier,
  val classpath: List<String>,
  val jvmOptions: List<String>,
  val workingDirectory: String,
  val environmentVariables: EnvironmentVariables,
  val mainClasses: List<JvmMainClass>? = null
)
