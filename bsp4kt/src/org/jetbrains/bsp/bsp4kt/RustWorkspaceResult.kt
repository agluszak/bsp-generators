package org.jetbrains.bsp.bsp4kt

import kotlinx.serialization.Serializable
import kotlinx.serialization.SerialName
import kotlinx.serialization.json.JsonElement

@Serializable
data class RustWorkspaceResult(
  val packages: List<RustPackage>,
  val rawDependencies: Map<String, List<RustRawDependency>>,
  val dependencies: Map<String, List<RustDependency>>,
  val resolvedTargets: List<BuildTargetIdentifier>
)
