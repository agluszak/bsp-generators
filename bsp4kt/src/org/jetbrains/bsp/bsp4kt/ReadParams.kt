package org.jetbrains.bsp.bsp4kt

import kotlinx.serialization.Serializable
import kotlinx.serialization.SerialName
import kotlinx.serialization.json.JsonElement

@Serializable
data class ReadParams(
  val originId: String,
  val task: TaskId? = null,
  val message: String
)
