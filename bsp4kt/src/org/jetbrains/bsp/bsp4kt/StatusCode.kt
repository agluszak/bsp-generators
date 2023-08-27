package org.jetbrains.bsp.bsp4kt

import com.jetbrains.jsonrpc4kt.IntEnum
import com.jetbrains.jsonrpc4kt.json.serializers.EnumAsIntSerializer
import kotlinx.serialization.KSerializer
import kotlinx.serialization.Serializable

@Serializable(with = StatusCode.Companion::class)
enum class StatusCode(override val value: Int) : IntEnum {
  Ok(1),
  Error(2),
  Cancelled(3);

  companion object : KSerializer<StatusCode> by EnumAsIntSerializer(StatusCode::class)
}
