package org.jetbrains.bsp.bsp4kt

import com.jetbrains.jsonrpc4kt.IntEnum
import com.jetbrains.jsonrpc4kt.json.serializers.EnumAsIntSerializer
import kotlinx.serialization.KSerializer
import kotlinx.serialization.Serializable

@Serializable(with = ScalaPlatform.Companion::class)
enum class ScalaPlatform(override val value: Int) : IntEnum {
  Jvm(1),
  Js(2),
  Native(3);

  companion object : KSerializer<ScalaPlatform> by EnumAsIntSerializer(ScalaPlatform::class)
}
