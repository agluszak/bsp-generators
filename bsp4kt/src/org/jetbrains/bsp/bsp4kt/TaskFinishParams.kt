package org.jetbrains.bsp.bsp4kt

import kotlinx.serialization.Serializable
import kotlinx.serialization.SerialName
import kotlinx.serialization.json.JsonElement

@Serializable
data class TaskFinishParams(
  val taskId: TaskId,
  val eventTime: Long? = null,
  val message: String? = null,
  val status: StatusCode,
  val dataKind: String? = null,
  val data: JsonElement? = null
)
