package com.jetbrains.bsp.bsp4kt

import com.jetbrains.jsonrpc4kt.IntEnum
import com.jetbrains.jsonrpc4kt.json.serializers.EnumAsIntSerializer
import kotlinx.serialization.KSerializer
import kotlinx.serialization.Serializable

@Serializable(with = TestStatus.Companion::class)
enum class TestStatus(override val value: Int) : IntEnum {
  PASSED(1),
  FAILED(2),
  IGNORED(3),
  CANCELLED(4),
  SKIPPED(5);

  companion object : KSerializer<TestStatus> by EnumAsIntSerializer(TestStatus::class)
}
