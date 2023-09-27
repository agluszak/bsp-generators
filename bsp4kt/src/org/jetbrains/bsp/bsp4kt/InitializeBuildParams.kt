package org.jetbrains.bsp.bsp4kt

import kotlinx.serialization.Serializable
import kotlinx.serialization.SerialName
import kotlinx.serialization.json.JsonElement

@Serializable
data class InitializeBuildParams(
  val displayName: String,
  val version: String,
  val bspVersion: String,
  val rootUri: URI,
  val capabilities: BuildClientCapabilities,
  val dataKind: String? = null,
  val data: JsonElement? = null
)
