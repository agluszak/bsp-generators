package com.jetbrains.bsp.bsp4kt

import com.jetbrains.jsonrpc4kt.IntEnum
import com.jetbrains.jsonrpc4kt.json.serializers.EnumAsIntSerializer
import kotlinx.serialization.KSerializer
import kotlinx.serialization.Serializable

@Serializable(with = StatusCode.Companion::class)
enum class StatusCode(override val value: Int) : IntEnum {
  OK(1),
  ERROR(2),
  CANCELLED(3);

  companion object : KSerializer<StatusCode> by EnumAsIntSerializer(StatusCode::class)
}
