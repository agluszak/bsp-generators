package com.jetbrains.bsp.generators

import java.io.File
import java.io.FileWriter
import java.nio.file.Files
import java.nio.file.Path
import java.nio.file.attribute.BasicFileAttributes
import java.nio.file.attribute.FileAttribute
import java.nio.file.attribute.PosixFileAttributes

class FilesGenerator(val output: Path, private val generatorScript: File, private val codegenFiles: List<CodegenFile>) {
    fun run() {
        if (codegenFiles.isEmpty()) {
            System.err.println("No files to generate")
            return
        }

        val updateScript = FileWriter(generatorScript)
        updateScript.use { writer ->
            codegenFiles.forEach {
                val fullPath = output.resolve(it.path)
                fullPath.parent.toFile().mkdirs()
                fullPath.toFile().writeText(it.contents)
                writer.append("cp ${fullPath.toAbsolutePath()} \$BUILD_WORKSPACE_DIRECTORY/$output")
            }
        }
    }
}
