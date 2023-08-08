package com.jetbrains.bsp.bsp4kt

import kotlinx.serialization.Serializable
import kotlinx.serialization.SerialName
import kotlinx.serialization.json.JsonElement

@Serializable
data class MavenDependencyModule(
  val organization: String,
  val name: String,
  val version: String,
  val artifacts: List<MavenDependencyModuleArtifact>,
  val scope: String?
)
