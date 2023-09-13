package org.jetbrains.bsp.bsp4kt

import kotlinx.serialization.Serializable
import kotlinx.serialization.SerialName
import kotlinx.serialization.json.JsonElement

@Serializable
data class RustDependency(
  val pkg: String,
  val name: String? = null,
  val depKinds: List<RustDepKindInfo>? = null
)
