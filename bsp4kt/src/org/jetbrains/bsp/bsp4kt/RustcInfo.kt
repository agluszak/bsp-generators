package org.jetbrains.bsp.bsp4kt

import kotlinx.serialization.Serializable
import kotlinx.serialization.SerialName
import kotlinx.serialization.json.JsonElement

@Serializable
data class RustcInfo(
  val sysrootPath: String,
  val srcSysrootPath: String,
  val version: String,
  val host: String
)
