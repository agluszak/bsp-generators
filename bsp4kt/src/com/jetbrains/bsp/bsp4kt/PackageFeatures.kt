package com.jetbrains.bsp.bsp4kt

import kotlinx.serialization.Serializable
import kotlinx.serialization.SerialName
import kotlinx.serialization.json.JsonElement

@Serializable
data class PackageFeatures(
val packageId: String,
val targets: List<BuildTargetIdentifier>,
val availableFeatures: List<String>,
val enabledFeatures: List<String>)

