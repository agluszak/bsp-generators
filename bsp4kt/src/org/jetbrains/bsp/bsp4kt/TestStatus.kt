package org.jetbrains.bsp.bsp4kt

import org.jetbrains.jsonrpc4kt.IntEnum
import org.jetbrains.jsonrpc4kt.json.serializers.EnumAsIntSerializer
import kotlinx.serialization.KSerializer
import kotlinx.serialization.Serializable

@Serializable(with = TestStatus.Companion::class)
enum class TestStatus(override val value: Int) : IntEnum {
  Passed(1),
  Failed(2),
  Ignored(3),
  Cancelled(4),
  Skipped(5);

  companion object : KSerializer<TestStatus> by EnumAsIntSerializer(TestStatus::class)
}
