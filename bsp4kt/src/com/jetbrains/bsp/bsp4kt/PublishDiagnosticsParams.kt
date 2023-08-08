package com.jetbrains.bsp.bsp4kt

import kotlinx.serialization.Serializable
import kotlinx.serialization.SerialName
import kotlinx.serialization.json.JsonElement

@Serializable
data class PublishDiagnosticsParams(
  val textDocument: TextDocumentIdentifier,
  val buildTarget: BuildTargetIdentifier,
  val originId: String? = null,
  val diagnostics: List<Diagnostic>,
  val reset: Boolean
)
