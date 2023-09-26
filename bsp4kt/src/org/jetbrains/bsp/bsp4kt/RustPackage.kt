package org.jetbrains.bsp.bsp4kt

import kotlinx.serialization.Serializable
import kotlinx.serialization.SerialName
import kotlinx.serialization.json.JsonElement

@Serializable
data class RustPackage(
  val id: String,
  val rootUrl: URI,
  val name: String,
  val version: String,
  val origin: String,
  val edition: String,
  val source: String? = null,
  val resolvedTargets: List<RustBuildTarget>,
  val allTargets: List<RustBuildTarget>,
  val features: FeatureDependencyGraph,
  val enabledFeatures: Set<Feature>,
  val cfgOptions: RustCfgOptions? = null,
  val env: EnvironmentVariables? = null,
  val outDirUrl: URI? = null,
  val procMacroArtifact: String? = null
)
