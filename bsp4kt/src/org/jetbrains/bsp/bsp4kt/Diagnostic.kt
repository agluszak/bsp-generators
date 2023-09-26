package org.jetbrains.bsp.bsp4kt

import kotlinx.serialization.Serializable
import kotlinx.serialization.SerialName
import kotlinx.serialization.json.JsonElement

@Serializable
data class Diagnostic(
  val range: Range,
  val severity: DiagnosticSeverity? = null,
  val code: String? = null,
  val source: String? = null,
  val message: String,
  val relatedInformation: List<DiagnosticRelatedInformation>? = null,
  val data: DiagnosticData? = null
)
