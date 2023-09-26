package org.jetbrains.bsp.bsp4kt

import kotlinx.serialization.Serializable
import kotlinx.serialization.SerialName
import kotlinx.serialization.json.JsonElement

@Serializable
data class RustWorkspaceResult(
  val packages: List<RustPackage>,
  val rawDependencies: RustRawDependencies,
  val dependencies: RustDependencies,
  val resolvedTargets: List<BuildTargetIdentifier>
)
