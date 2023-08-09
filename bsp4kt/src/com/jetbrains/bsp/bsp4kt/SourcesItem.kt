package com.jetbrains.bsp.bsp4kt

import kotlinx.serialization.Serializable
import kotlinx.serialization.SerialName
import kotlinx.serialization.json.JsonElement

@Serializable
data class SourcesItem(
  val target: BuildTargetIdentifier,
  val sources: List<SourceItem>,
  val roots: List<String>? = null
)
