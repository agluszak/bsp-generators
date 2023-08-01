package com.jetbrains.bsp.generators


import java.nio.file.Files
import java.nio.file.Paths

object ProtocolVersionLoader {
  fun version(): String {
    try {
      val path = Paths.get("spec/src/main/resources/META-INF/smithy/bsp/version")
      return Files.readString(path).trim()
    } catch (e: Throwable) {
      throw RuntimeException(
          "Failed to load the protocol version, make sure that the working directory is set correctly",
          e
        )
    }
  }
}
