package org.jetbrains.bsp.bsp4kt

import kotlinx.serialization.Serializable
import kotlinx.serialization.SerialName
import kotlinx.serialization.json.JsonElement

@Serializable
data class ScalaTestClassesItem(
  val target: BuildTargetIdentifier,
  val framework: String? = null,
  val classes: List<String>
)
