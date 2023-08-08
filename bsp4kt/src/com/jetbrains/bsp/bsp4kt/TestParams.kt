package com.jetbrains.bsp.bsp4kt

import kotlinx.serialization.Serializable
import kotlinx.serialization.SerialName
import kotlinx.serialization.json.JsonElement

@Serializable
data class TestParams(
val targets: List<BuildTargetIdentifier>,
val originId: String?,
val arguments: List<String>?,
val dataKind: String?,
val data: JsonElement?)

