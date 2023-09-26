package org.jetbrains.bsp.bsp4kt

import kotlinx.serialization.Serializable
import kotlinx.serialization.SerialName
import kotlinx.serialization.json.JsonElement

@Serializable
data class RustBuildTarget(
  val name: String,
  val crateRootUrl: URI,
  val kind: RustTargetKind,
  val crateTypes: List<RustCrateType>? = null,
  val edition: String,
  val doctest: Boolean,
  val requiredFeatures: Set<Feature>? = null
)
