package org.jetbrains.bsp.bsp4kt

import org.jetbrains.jsonrpc4kt.IntEnum
import org.jetbrains.jsonrpc4kt.json.serializers.EnumAsIntSerializer
import kotlinx.serialization.KSerializer
import kotlinx.serialization.Serializable

@Serializable(with = RustCrateType.Companion::class)
enum class RustCrateType(override val value: Int) : IntEnum {
  Bin(1),
  Lib(2),
  Rlib(3),
  Dylib(4),
  Cdylib(5),
  Staticlib(6),
  ProcMacro(7),
  Unknown(8);

  companion object : KSerializer<RustCrateType> by EnumAsIntSerializer(RustCrateType::class)
}
