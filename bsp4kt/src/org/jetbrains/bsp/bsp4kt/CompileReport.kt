package org.jetbrains.bsp.bsp4kt

import kotlinx.serialization.Serializable
import kotlinx.serialization.SerialName
import kotlinx.serialization.json.JsonElement

@Serializable
data class CompileReport(
  val target: BuildTargetIdentifier,
  val originId: String? = null,
  val errors: Int,
  val warnings: Int,
  val time: Long? = null,
  val noOp: Boolean? = null
)
