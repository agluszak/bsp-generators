package org.jetbrains.bsp.bsp4kt

import bsp4kt.util.StringIntUnionSerializer
import kotlinx.serialization.Serializable

@Serializable(with = Code.Companion::class)
sealed interface Code {
  @Serializable
  @JvmInline
  value class StringValue(val value: String): Code {}

  @Serializable
  @JvmInline
  value class IntValue(val value: Int): Code {}

  companion object : StringIntUnionSerializer<Code>(
    clazz = Code::class,
    stringSerializer = StringValue.serializer(),
    intSerializer = IntValue.serializer(),
  )
}
