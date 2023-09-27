package org.jetbrains.bsp.bsp4kt

import kotlinx.serialization.Serializable
import kotlinx.serialization.SerialName
import kotlinx.serialization.json.JsonElement

@Serializable
data class TaskProgressParams(
  val taskId: TaskId,
  val originId: String? = null,
  val eventTime: Long? = null,
  val message: String? = null,
  val total: Long? = null,
  val progress: Long? = null,
  val unit: String? = null,
  val dataKind: String? = null,
  val data: JsonElement? = null
)
