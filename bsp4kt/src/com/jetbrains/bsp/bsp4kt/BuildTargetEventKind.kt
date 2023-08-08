package com.jetbrains.bsp.bsp4kt

import com.jetbrains.jsonrpc4kt.IntEnum
import com.jetbrains.jsonrpc4kt.json.serializers.EnumAsIntSerializer
import kotlinx.serialization.KSerializer
import kotlinx.serialization.Serializable

@Serializable(with = BuildTargetEventKind.Companion::class)
enum class BuildTargetEventKind(override val value: Int) : IntEnum {
  CREATED(1),
  CHANGED(2),
  DELETED(3);

  companion object : KSerializer<BuildTargetEventKind> by EnumAsIntSerializer(BuildTargetEventKind::class)
}
