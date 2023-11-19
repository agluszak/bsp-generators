package org.jetbrains.bsp.bsp4kt

import org.jetbrains.bsp.util.StringIntUnionSerializer
import kotlinx.serialization.Serializable

@Serializable(with = RequestId.Companion::class)
sealed interface RequestId {
  @Serializable
  @JvmInline
  value class StringValue(val value: String): RequestId {}

  @Serializable
  @JvmInline
  value class IntValue(val value: Int): RequestId {}

  companion object : StringIntUnionSerializer<RequestId>(
    clazz = RequestId::class,
    stringSerializer = StringValue.serializer(),
    intSerializer = IntValue.serializer(),
  )
}
