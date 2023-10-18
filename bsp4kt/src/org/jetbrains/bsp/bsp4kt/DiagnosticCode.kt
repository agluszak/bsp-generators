package org.jetbrains.bsp.bsp4kt

import org.jetbrains.bsp.util.StringIntUnionSerializer
import kotlinx.serialization.Serializable

@Serializable(with = DiagnosticCode.Companion::class)
sealed interface DiagnosticCode {
  @Serializable
  @JvmInline
  value class StringValue(val value: String): DiagnosticCode {}

  @Serializable
  @JvmInline
  value class IntValue(val value: Int): DiagnosticCode {}

  companion object : StringIntUnionSerializer<DiagnosticCode>(
    clazz = DiagnosticCode::class,
    stringSerializer = StringValue.serializer(),
    intSerializer = IntValue.serializer(),
  )
}
