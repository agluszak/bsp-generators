package com.jetbrains.bsp.bsp4kt

import kotlinx.serialization.Serializable
import kotlinx.serialization.SerialName
import kotlinx.serialization.json.JsonElement

@Serializable
data class BuildServerCapabilities(
val compileProvider: CompileProvider?,
val testProvider: TestProvider?,
val runProvider: RunProvider?,
val debugProvider: DebugProvider?,
val inverseSourcesProvider: Boolean?,
val dependencySourcesProvider: Boolean?,
val dependencyModulesProvider: Boolean?,
val resourcesProvider: Boolean?,
val outputPathsProvider: Boolean?,
val buildTargetChangedProvider: Boolean?,
val jvmRunEnvironmentProvider: Boolean?,
val jvmTestEnvironmentProvider: Boolean?,
val canReload: Boolean?)

