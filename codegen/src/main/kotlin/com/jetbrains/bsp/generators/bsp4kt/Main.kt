package com.jetbrains.bsp.generators.bsp4kt

import com.jetbrains.bsp.generators.FilesGenerator
import com.jetbrains.bsp.generators.Loader
import com.jetbrains.bsp.generators.ir.SmithyToIr
import java.io.File
import kotlin.io.path.Path

object Main {
    @JvmStatic
    fun main(args: Array<String>) {
        if (args.size != 3) {
            println("Usage: bsp4kt <name> <output directory> <generator script path>")
            return
        }

        val name = args[0]
        val output = Path(args[1])
        val generatorScript = File(args[2])
        val model = Loader.model
        val namespaces = Loader.namespaces
        val ir = SmithyToIr(model)
        val definitions = namespaces.flatMap { ir.definitions(it) }
        val version = Loader.protocolVersion
        val renderer = KotlinRenderer("com.jetbrains.bsp.bsp4kt", definitions, version)

        val codegenFiles = renderer.render()

        FilesGenerator(name, output, generatorScript, codegenFiles).run()
    }
}
