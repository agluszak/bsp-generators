package com.jetbrains.bsp.bsp4kt

import kotlinx.serialization.Serializable
import kotlinx.serialization.SerialName
import kotlinx.serialization.json.JsonElement

@Serializable
data class CppBuildTarget(
val version: String?,
val compiler: String?,
val cCompiler: String?,
val cppCompiler: String?)

