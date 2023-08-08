package com.jetbrains.bsp.bsp4kt

import kotlinx.serialization.Serializable
import kotlinx.serialization.SerialName
import kotlinx.serialization.json.JsonElement

@Serializable
data class ScalaTestParams(
val testClasses: List<ScalaTestClassesItem>?,
val jvmOptions: List<String>?)

