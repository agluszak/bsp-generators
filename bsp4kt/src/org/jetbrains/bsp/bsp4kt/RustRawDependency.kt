package org.jetbrains.bsp.bsp4kt

import kotlinx.serialization.Serializable
import kotlinx.serialization.SerialName
import kotlinx.serialization.json.JsonElement

@Serializable
data class RustRawDependency(
  val name: String,
  val rename: String? = null,
  val kind: String? = null,
  val target: String? = null,
  val optional: Boolean,
  val usesDefaultFeatures: Boolean,
  val features: Set<Feature>
)
