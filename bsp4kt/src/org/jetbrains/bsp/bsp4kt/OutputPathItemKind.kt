package org.jetbrains.bsp.bsp4kt

import com.jetbrains.jsonrpc4kt.IntEnum
import com.jetbrains.jsonrpc4kt.json.serializers.EnumAsIntSerializer
import kotlinx.serialization.KSerializer
import kotlinx.serialization.Serializable

@Serializable(with = OutputPathItemKind.Companion::class)
enum class OutputPathItemKind(override val value: Int) : IntEnum {
  File(1),
  Directory(2);

  companion object : KSerializer<OutputPathItemKind> by EnumAsIntSerializer(OutputPathItemKind::class)
}
