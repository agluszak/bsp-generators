package org.jetbrains.bsp.bsp4kt

import org.jetbrains.jsonrpc4kt.IntEnum
import org.jetbrains.jsonrpc4kt.json.serializers.EnumAsIntSerializer
import kotlinx.serialization.KSerializer
import kotlinx.serialization.Serializable

@Serializable(with = RustTargetKind.Companion::class)
enum class RustTargetKind(override val value: Int) : IntEnum {
  Lib(1),
  Bin(2),
  Test(3),
  Example(4),
  Bench(5),
  Custombuild(6),
  Unknown(7);

  companion object : KSerializer<RustTargetKind> by EnumAsIntSerializer(RustTargetKind::class)
}
