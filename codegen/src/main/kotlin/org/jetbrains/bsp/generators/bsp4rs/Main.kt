package org.jetbrains.bsp.generators.bsp4rs

import org.jetbrains.bsp.generators.FilesGenerator
import org.jetbrains.bsp.generators.Loader
import org.jetbrains.bsp.generators.ir.SmithyToIr
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
        val definitions = namespaces.map { Module(it.removePrefix("bsp."), ir.definitions(it)) }
        val version = Loader.protocolVersion
        val renderer = RustRenderer("org.jetbrains.bsp.bsp4rs", definitions, version)

        val codegenFiles = renderer.render()

        val clippy =
            "cargo clippy --fix --lib -p $name --allow-no-vcs --manifest-path \$BUILD_WORKSPACE_DIRECTORY/$name/Cargo.toml"
        val fmt = "cargo fmt --all --manifest-path \$BUILD_WORKSPACE_DIRECTORY/$name/Cargo.toml"

        FilesGenerator(name, output, generatorScript, codegenFiles, listOf(clippy, fmt)).run()
    }
}