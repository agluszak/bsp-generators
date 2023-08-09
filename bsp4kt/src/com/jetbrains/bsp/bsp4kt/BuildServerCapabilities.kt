package com.jetbrains.bsp.bsp4kt

import kotlinx.serialization.Serializable
import kotlinx.serialization.SerialName
import kotlinx.serialization.json.JsonElement

@Serializable
data class BuildServerCapabilities(
  val compileProvider: CompileProvider? = null,
  val testProvider: TestProvider? = null,
  val runProvider: RunProvider? = null,
  val debugProvider: DebugProvider? = null,
  val inverseSourcesProvider: Boolean? = null,
  val dependencySourcesProvider: Boolean? = null,
  val dependencyModulesProvider: Boolean? = null,
  val resourcesProvider: Boolean? = null,
  val outputPathsProvider: Boolean? = null,
  val buildTargetChangedProvider: Boolean? = null,
  val jvmRunEnvironmentProvider: Boolean? = null,
  val jvmTestEnvironmentProvider: Boolean? = null,
  val canReload: Boolean? = null
)
