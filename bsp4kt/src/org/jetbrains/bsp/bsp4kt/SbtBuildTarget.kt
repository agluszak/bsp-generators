package org.jetbrains.bsp.bsp4kt

import kotlinx.serialization.Serializable
import kotlinx.serialization.SerialName
import kotlinx.serialization.json.JsonElement

@Serializable
data class SbtBuildTarget(
  val sbtVersion: String,
  val autoImports: List<String>,
  val scalaBuildTarget: ScalaBuildTarget,
  val parent: BuildTargetIdentifier? = null,
  val children: List<BuildTargetIdentifier>
)
