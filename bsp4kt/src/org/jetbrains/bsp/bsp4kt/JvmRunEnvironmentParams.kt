package org.jetbrains.bsp.bsp4kt

import kotlinx.serialization.Serializable
import kotlinx.serialization.SerialName
import kotlinx.serialization.json.JsonElement

@Serializable
data class JvmRunEnvironmentParams(
  val targets: List<BuildTargetIdentifier>,
  val originId: Identifier? = null
)
