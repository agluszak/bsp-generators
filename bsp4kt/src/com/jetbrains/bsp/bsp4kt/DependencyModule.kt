package com.jetbrains.bsp.bsp4kt

import kotlinx.serialization.Serializable
import kotlinx.serialization.SerialName
import kotlinx.serialization.json.JsonElement

@Serializable
data class DependencyModule(
  val name: String,
  val version: String,
  val dataKind: String?,
  val data: JsonElement?
)
