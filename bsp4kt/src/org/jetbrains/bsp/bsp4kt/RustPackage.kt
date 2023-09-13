package org.jetbrains.bsp.bsp4kt

import kotlinx.serialization.Serializable
import kotlinx.serialization.SerialName
import kotlinx.serialization.json.JsonElement

@Serializable
data class RustPackage(
  val id: String,
  val rootUrl: String,
  val name: String,
  val version: String,
  val origin: String,
  val edition: String,
  val source: String? = null,
  val resolvedTargets: List<RustBuildTarget>,
  val allTargets: List<RustBuildTarget>,
  val features: Set<RustFeature>,
  val enabledFeatures: List<String>,
  val cfgOptions: Map<String, List<String>>? = null,
  val env: Map<String, String>? = null,
  val outDirUrl: String? = null,
  val procMacroArtifact: String? = null
)
