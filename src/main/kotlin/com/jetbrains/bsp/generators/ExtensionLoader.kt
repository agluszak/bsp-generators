package com.jetbrains.bsp.generators

import java.nio.file.Files
import java.nio.file.Paths
import java.util.stream.Collectors

object ExtensionLoader {
  val extensions: List<String> = run {
      try {
          val extensionsPath = Paths.get("spec/src/main/resources/META-INF/smithy/bsp/extensions")
          return@run Files
              .list(extensionsPath)
              .collect(Collectors.toList())
              .map { it.fileName.toString().split("\\.").first() }
      } catch (e: Throwable) {
        throw RuntimeException(
          "Failed to load extensions, make sure that the working directory is set correctly",
          e
        )
    }
  }

  val namespaces: List<String> = run {
    listOf("bsp") + extensions.map { "bsp.$it" }
  }
}