package com.jetbrains.bsp.generators.bsp4kt

import com.jetbrains.bsp.generators.FilesGenerator
import com.jetbrains.bsp.generators.Loader
import com.jetbrains.bsp.generators.ir.SmithyToIr
import java.io.File
import kotlin.io.path.Path

object Main {
    @JvmStatic
    fun main(args: Array<String>) {
        if (args.size != 2) {
            println("Usage: bsp4kt <output directory> <generator script path>")
            return
        }

        val output = Path(args[0])
        val generatorScript = File(args[1])
        val model = Loader.model
        val namespaces = Loader.namespaces
        val ir = SmithyToIr(model)
        val definitions = namespaces.flatMap { ir.definitions(it) }
        val version = Loader.protocolVersion
        val renderer = KotlinRenderer("com.jetbrains.bsp.bsp4kt", definitions, version)

        val codegenFiles = renderer.render()

        FilesGenerator(output, generatorScript, codegenFiles).run()
    }
}