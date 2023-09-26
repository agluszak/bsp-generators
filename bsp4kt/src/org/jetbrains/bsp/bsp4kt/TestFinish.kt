package org.jetbrains.bsp.bsp4kt

import kotlinx.serialization.Serializable
import kotlinx.serialization.SerialName
import kotlinx.serialization.json.JsonElement

@Serializable
data class TestFinish(
  val displayName: String,
  val message: String? = null,
  val status: TestStatus,
  val location: Location? = null,
  val data: TestFinishData? = null
)
