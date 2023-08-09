package com.jetbrains.bsp.generators

import java.io.File
import java.io.FileWriter
import java.nio.file.Path

class FilesGenerator(
    val name: String,
    val output: Path,
    private val generatorScript: File,
    private val codegenFiles: List<CodegenFile>
) {
    fun run() {
        if (codegenFiles.isEmpty()) {
            System.err.println("No files to generate")
            return
        }

        val updateScript = FileWriter(generatorScript)
        updateScript.use { writer ->
            writer.appendLine("#!/bin/bash")
            writer.appendLine("set -e")
            val targetPath = "\$BUILD_WORKSPACE_DIRECTORY/$name/src"
            writer.appendLine("rm -rf $targetPath")
            writer.appendLine("mkdir -p $targetPath")
            codegenFiles.forEach {
                val fullPath = output.resolve(it.path)
                fullPath.parent.toFile().mkdirs()
                fullPath.toFile().writeText(it.contents)
                writer.appendLine("mkdir -p $targetPath/${it.path.parent}")
                writer.appendLine("cp \$BUILD_WORKSPACE_DIRECTORY/${fullPath} $targetPath/${it.path}")
                writer.appendLine("chmod u+rw $targetPath/${it.path}")
            }
        }

    }
}
