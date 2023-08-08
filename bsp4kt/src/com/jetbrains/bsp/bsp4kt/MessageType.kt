package com.jetbrains.bsp.bsp4kt

import com.jetbrains.jsonrpc4kt.IntEnum
import com.jetbrains.jsonrpc4kt.json.serializers.EnumAsIntSerializer
import kotlinx.serialization.KSerializer
import kotlinx.serialization.Serializable

@Serializable(with = MessageType.Companion::class)
enum class MessageType(override val value: Int) : IntEnum {
  ERROR(1),
  WARNING(2),
  INFO(3),
  LOG(4);

  companion object : KSerializer<MessageType> by EnumAsIntSerializer(MessageType::class)
}
