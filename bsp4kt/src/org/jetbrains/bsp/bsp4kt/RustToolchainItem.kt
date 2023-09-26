package org.jetbrains.bsp.bsp4kt

import kotlinx.serialization.Serializable
import kotlinx.serialization.SerialName
import kotlinx.serialization.json.JsonElement

@Serializable
data class RustToolchainItem(
  val rustStdLib: RustcInfo? = null,
  val cargoBinPath: URI,
  val procMacroSrvPath: URI
)
