package org.jetbrains.bsp.bsp4kt

import kotlinx.serialization.Serializable
import kotlinx.serialization.SerialName
import kotlinx.serialization.json.JsonElement

@Serializable
data class ScalaBuildTarget(
  val scalaOrganization: String,
  val scalaVersion: String,
  val scalaBinaryVersion: String,
  val platform: ScalaPlatform,
  val jars: List<String>,
  val jvmBuildTarget: JvmBuildTarget? = null
)
