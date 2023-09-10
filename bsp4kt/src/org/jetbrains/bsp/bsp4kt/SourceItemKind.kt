package org.jetbrains.bsp.bsp4kt

import org.jetbrains.jsonrpc4kt.IntEnum
import org.jetbrains.jsonrpc4kt.json.serializers.EnumAsIntSerializer
import kotlinx.serialization.KSerializer
import kotlinx.serialization.Serializable

@Serializable(with = SourceItemKind.Companion::class)
enum class SourceItemKind(override val value: Int) : IntEnum {
  File(1),
  Directory(2);

  companion object : KSerializer<SourceItemKind> by EnumAsIntSerializer(SourceItemKind::class)
}
