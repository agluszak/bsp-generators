package com.jetbrains.bsp.bsp4kt

import kotlinx.serialization.Serializable
import kotlinx.serialization.SerialName
import kotlinx.serialization.json.JsonElement

@Serializable
data class TaskStartParams(
val taskId: TaskId,
val eventTime: Long?,
val message: String?,
val dataKind: String?,
val data: JsonElement?)

