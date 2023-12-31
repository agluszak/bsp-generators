package org.jetbrains.bsp.bsp4kt

import kotlinx.serialization.Serializable
import kotlinx.serialization.SerialName
import kotlinx.serialization.json.JsonElement

@Serializable
data class LogMessageParams(
  val type: MessageType,
  val task: TaskId? = null,
  val originId: String? = null,
  val message: String
)
