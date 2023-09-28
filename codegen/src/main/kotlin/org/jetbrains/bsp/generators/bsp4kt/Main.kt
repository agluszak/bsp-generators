package org.jetbrains.bsp.generators.bsp4kt

import org.jetbrains.bsp.generators.FilesGenerator
import org.jetbrains.bsp.generators.Loader
import org.jetbrains.bsp.generators.ir.AbstractionLevel
import org.jetbrains.bsp.generators.ir.IrConfig
import org.jetbrains.bsp.generators.ir.SmithyToIr
import org.jetbrains.bsp.generators.ir.TypeAliasing
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
        val irConfig = IrConfig(
            strings = TypeAliasing.Pure,
            maps = TypeAliasing.Pure,
            dataWithKind = AbstractionLevel.AsType,
            openEnums = AbstractionLevel.AsType,
            untaggedUnions = AbstractionLevel.AsDef,
        )
        val ir = SmithyToIr(model, irConfig)
        val definitions = namespaces.flatMap { ir.definitions(it) }
        val version = Loader.protocolVersion
        val renderer = KotlinRenderer("org.jetbrains.bsp.bsp4kt", definitions, version)

        val codegenFiles = renderer.render()

        val generator = FilesGenerator(name, output, generatorScript, codegenFiles)
        generator.generateFiles()
        generator.writeScript()
    }
}
