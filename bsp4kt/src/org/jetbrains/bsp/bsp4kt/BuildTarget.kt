package org.jetbrains.bsp.bsp4kt

import kotlinx.serialization.Serializable
import kotlinx.serialization.SerialName
import kotlinx.serialization.json.JsonElement

@Serializable
data class BuildTarget(
  val id: BuildTargetIdentifier,
  val displayName: String? = null,
  val baseDirectory: URI? = null,
  val tags: List<String>,
  val languageIds: List<LanguageId>,
  val dependencies: List<BuildTargetIdentifier>,
  val capabilities: BuildTargetCapabilities,
  val dataKind: String? = null,
  val data: JsonElement? = null
)
