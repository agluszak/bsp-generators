package com.jetbrains.bsp.bsp4kt

import kotlinx.serialization.Serializable
import kotlinx.serialization.SerialName
import kotlinx.serialization.json.JsonElement

@Serializable
data class TaskProgressParams(
val taskId: TaskId,
val eventTime: Long?,
val message: String?,
val total: Long?,
val progress: Long?,
val unit: String?,
val dataKind: String?,
val data: JsonElement?)

