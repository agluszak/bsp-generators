package com.jetbrains.bsp.bsp4kt

import kotlinx.serialization.Serializable
import kotlinx.serialization.SerialName
import kotlinx.serialization.json.JsonElement

@Serializable
data class BuildTarget(
val id: BuildTargetIdentifier,
val displayName: String?,
val baseDirectory: String?,
val tags: List<String>,
val languageIds: List<String>,
val dependencies: List<BuildTargetIdentifier>,
val capabilities: BuildTargetCapabilities,
val dataKind: String?,
val data: JsonElement?)

