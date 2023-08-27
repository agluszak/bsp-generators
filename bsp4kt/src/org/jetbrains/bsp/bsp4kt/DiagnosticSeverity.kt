package org.jetbrains.bsp.bsp4kt

import com.jetbrains.jsonrpc4kt.IntEnum
import com.jetbrains.jsonrpc4kt.json.serializers.EnumAsIntSerializer
import kotlinx.serialization.KSerializer
import kotlinx.serialization.Serializable

@Serializable(with = DiagnosticSeverity.Companion::class)
enum class DiagnosticSeverity(override val value: Int) : IntEnum {
  Error(1),
  Warning(2),
  Information(3),
  Hint(4);

  companion object : KSerializer<DiagnosticSeverity> by EnumAsIntSerializer(DiagnosticSeverity::class)
}
