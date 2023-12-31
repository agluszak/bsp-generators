package org.jetbrains.bsp.bsp4kt

import kotlinx.serialization.Serializable
import kotlinx.serialization.SerialName
import kotlinx.serialization.json.JsonElement

@Serializable
data class ScalaMainClassesResult(
  val items: List<ScalaMainClassesItem>,
  val originId: String? = null
)
