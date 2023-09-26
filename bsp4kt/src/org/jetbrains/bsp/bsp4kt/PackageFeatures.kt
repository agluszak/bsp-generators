package org.jetbrains.bsp.bsp4kt

import kotlinx.serialization.Serializable
import kotlinx.serialization.SerialName
import kotlinx.serialization.json.JsonElement

@Serializable
data class PackageFeatures(
  val packageId: String,
  val targets: List<BuildTargetIdentifier>,
  val availableFeatures: FeatureDependencyGraph,
  val enabledFeatures: Set<Feature>
)
