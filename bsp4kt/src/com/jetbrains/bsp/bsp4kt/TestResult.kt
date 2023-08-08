package com.jetbrains.bsp.bsp4kt

import kotlinx.serialization.Serializable
import kotlinx.serialization.SerialName
import kotlinx.serialization.json.JsonElement

@Serializable
data class TestResult(
val originId: String?,
val statusCode: StatusCode,
val dataKind: String?,
val data: JsonElement?)

