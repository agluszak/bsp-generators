package com.jetbrains.bsp.bsp4kt

import kotlinx.serialization.Serializable
import kotlinx.serialization.SerialName
import kotlinx.serialization.json.JsonElement

@Serializable
data class ScalaMainClassesParams(
val targets: List<BuildTargetIdentifier>,
val originId: String?)

