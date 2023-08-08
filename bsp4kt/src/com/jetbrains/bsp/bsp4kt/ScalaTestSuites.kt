package com.jetbrains.bsp.bsp4kt

import kotlinx.serialization.Serializable
import kotlinx.serialization.SerialName
import kotlinx.serialization.json.JsonElement

@Serializable
data class ScalaTestSuites(
  val suites: List<ScalaTestSuiteSelection>,
  val jvmOptions: List<String>,
  val environmentVariables: List<String>
)
