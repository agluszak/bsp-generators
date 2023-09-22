package bsp.codegen.bsp4j

import scala.jdk.CollectionConverters._
import bsp.codegen.ir.SmithyToIR
import org.jetbrains.bsp.generators.{FilesGenerator, Loader}

import java.util
import java.io.File
import java.nio.file.Path

object Main {
  def main(args: Array[String]): Unit = {
    if (args.length != 3) {
      println("Usage: bsp4s <name> <output directory> <generator script path>")
      return
    }

    val model = Loader.INSTANCE.getModel
    val namespaces = Loader.INSTANCE.getNamespaces.asScala.toList
    val ir = new SmithyToIR(model)
    val definitions = namespaces.flatMap(ir.definitions)
    val version = Loader.INSTANCE.getProtocolVersion
    val renderer = new JavaRenderer("ch.epfl.scala.bsp4j", definitions, version)

    val codegenFiles = renderer.render().asJava

    val name = args(0)
    val output = Path.of(args(1))
    val generatorScript = new File(args(2))

    val additionalCommands = List().asJava

    new FilesGenerator(name, output, generatorScript, codegenFiles, additionalCommands).run()

  }
}

