package com.jetbrains.bsp.bsp4kt

import kotlinx.serialization.Serializable
import kotlinx.serialization.SerialName
import kotlinx.serialization.json.JsonElement

@Serializable
data class BspConnectionDetails(
val name: String,
val argv: List<String>,
val version: String,
val bspVersion: String,
val languages: List<String>)

