package org.jetbrains.bsp.bsp4kt

import kotlinx.serialization.Serializable
import kotlinx.serialization.SerialName
import kotlinx.serialization.json.JsonElement

@Serializable
data class TestReport(
  val originId: String? = null,
  val target: BuildTargetIdentifier,
  val passed: Int,
  val failed: Int,
  val ignored: Int,
  val cancelled: Int,
  val skipped: Int,
  val time: Long? = null
)
