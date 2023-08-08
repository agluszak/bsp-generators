package com.jetbrains.bsp.bsp4kt

import kotlinx.serialization.Serializable
import kotlinx.serialization.SerialName
import kotlinx.serialization.json.JsonElement

@Serializable
data class MavenDependencyModuleArtifact(
  val uri: String,
  val classifier: String? = null
)
