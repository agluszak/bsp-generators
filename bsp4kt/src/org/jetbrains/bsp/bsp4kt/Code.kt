package org.jetbrains.bsp.bsp4kt

import bsp4kt.util.stringIntUnionSerializer
import kotlinx.serialization.DeserializationStrategy
import kotlinx.serialization.Serializable
import kotlinx.serialization.json.JsonContentPolymorphicSerializer
import kotlinx.serialization.json.JsonElement

@Serializable(with = Code.Companion::class)
sealed interface Code {
  @Serializable
  @JvmInline
  value class StringValue(val value: String): Code {}

  @Serializable
  @JvmInline
  value class IntValue(val value: Int): Code {}

  companion object : JsonContentPolymorphicSerializer<Code>(Code::class) {
    override fun selectDeserializer(element: JsonElement): DeserializationStrategy<Code> {
      return stringIntUnionSerializer<Code>(stringSerializer = StringValue.serializer(),intSerializer = IntValue.serializer())(element)
    }
  }
}
