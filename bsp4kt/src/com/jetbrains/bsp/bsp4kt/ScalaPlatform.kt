package com.jetbrains.bsp.bsp4kt

import com.jetbrains.jsonrpc4kt.IntEnum
import com.jetbrains.jsonrpc4kt.json.serializers.EnumAsIntSerializer
import kotlinx.serialization.KSerializer
import kotlinx.serialization.Serializable

@Serializable(with = ScalaPlatform.Companion::class)
enum class ScalaPlatform(override val value: Int) : IntEnum {
  JVM(1),
  JS(2),
  NATIVE(3);

  companion object : KSerializer<ScalaPlatform> by EnumAsIntSerializer(ScalaPlatform::class)
}
