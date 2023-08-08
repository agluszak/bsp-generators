package com.jetbrains.bsp.bsp4kt

import kotlinx.serialization.Serializable
import kotlinx.serialization.SerialName
import kotlinx.serialization.json.JsonElement

@Serializable
data class TaskStartParams(
  val taskId: TaskId,
  val eventTime: Long? = null,
  val message: String? = null,
  val dataKind: String? = null,
  val data: JsonElement? = null
)
