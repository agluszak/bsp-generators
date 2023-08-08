package com.jetbrains.bsp.bsp4kt

import com.jetbrains.jsonrpc4kt.IntEnum
import com.jetbrains.jsonrpc4kt.json.serializers.EnumAsIntSerializer
import kotlinx.serialization.KSerializer
import kotlinx.serialization.Serializable

@Serializable(with = OutputPathItemKind.Companion::class)
enum class OutputPathItemKind(override val value: Int) : IntEnum {
  FILE(1),
  DIRECTORY(2);

  companion object : KSerializer<OutputPathItemKind> by EnumAsIntSerializer(OutputPathItemKind::class)
}
