package com.jetbrains.bsp.generators

import java.nio.file.Path

class FilesGenerator(val output: Path, val codegenFiles: List<CodegenFile>) {
    fun run() {
        if (codegenFiles.isEmpty()) {
            System.err.println("No files to generate")
            return
        }
        codegenFiles.forEach {
            val fullPath = output.resolve(it.path)
            fullPath.parent.toFile().mkdirs()
            fullPath.toFile().writeText(it.contents)
            System.err.println("Generated file ${fullPath.toAbsolutePath()}")
        }
    }
}
