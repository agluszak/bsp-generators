package com.jetbrains.bsp.bsp4kt

import kotlinx.serialization.Serializable
import kotlinx.serialization.SerialName
import kotlinx.serialization.json.JsonElement

@Serializable
data class Diagnostic(
val range: Range,
val severity: DiagnosticSeverity?,
val code: String?,
val source: String?,
val message: String,
val relatedInformation: List<DiagnosticRelatedInformation>?,
val dataKind: String?,
val data: JsonElement?)

