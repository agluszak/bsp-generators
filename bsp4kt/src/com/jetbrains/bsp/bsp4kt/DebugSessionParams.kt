package com.jetbrains.bsp.bsp4kt

import kotlinx.serialization.Serializable
import kotlinx.serialization.SerialName
import kotlinx.serialization.json.JsonElement

@Serializable
data class DebugSessionParams(
val targets: List<BuildTargetIdentifier>,
val dataKind: String?,
val data: JsonElement?)

