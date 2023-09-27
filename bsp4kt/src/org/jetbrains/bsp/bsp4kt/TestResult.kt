package org.jetbrains.bsp.bsp4kt

import kotlinx.serialization.Serializable
import kotlinx.serialization.SerialName
import kotlinx.serialization.json.JsonElement

@Serializable
data class TestResult(
  val originId: Identifier? = null,
  val statusCode: StatusCode,
  val dataKind: String? = null,
  val data: JsonElement? = null
)
