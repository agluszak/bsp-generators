package org.jetbrains.bsp.generators.bsp4rs

import org.jetbrains.bsp.generators.CodegenFile
import org.jetbrains.bsp.generators.bsp4rs.def.*
import org.jetbrains.bsp.generators.dsl.CodeBlock
import org.jetbrains.bsp.generators.dsl.rustCode
import org.jetbrains.bsp.generators.ir.Def
import org.jetbrains.bsp.generators.ir.Hint
import org.jetbrains.bsp.generators.ir.Type
import org.jetbrains.bsp.generators.utils.camelToSnakeCase
import software.amazon.smithy.model.shapes.ShapeId
import java.nio.file.Path
import kotlin.io.path.Path

class RustRenderer(basepkg: String, private val modules: List<Module>, val version: String) {
    private val baseRelPath = Path(basepkg.replace(".", "/"))
    val deriveRenderer = DeriveRenderer(modules.flatMap { it.definitions }.associateBy { it.shapeId })
    val serializationRenderer = SerializationRenderer()

    private val renames: Map<String, String> = mapOf(Pair("type", "r#type"), Pair("r#version", "version"))
    private val bannedAliases: List<String> = listOf("Integer", "Long")

    fun render(): List<CodegenFile> {
        val defFiles = modules.flatMap { renderModule(it) }
        val libFile = generateLibFile(modules.map { it.moduleName })
        return defFiles + libFile
    }

    private fun renderModule(module: Module): List<CodegenFile> {
        val filesWithNames = module.definitions.mapNotNull {
            renderDef(it)?.run {
                val name = makeName(it.name).camelToSnakeCase()
                Pair(generateFile(this, module.path, "$name.rs"), name)
            }
        }

        val modFile = generateModFile(module.path, filesWithNames.map { (_, defName) -> defName })
        val files = filesWithNames.map { (file, _) -> file }

        return files + modFile
    }

    private fun renderDef(def: Def): CodeBlock? = when (def) {
        is Def.Structure -> renderStructure(def)
        is Def.OpenEnum<*> -> renderOpenEnum(def)
        is Def.ClosedEnum<*> -> renderClosedEnum(def)
        is Def.Service -> renderService(def)
        is Def.Alias -> renderAlias(def)
        is Def.DataKinds -> renderDataKinds(def)
    }

    private fun generateModFile(modulePath: Path, filesNames: List<String>): CodegenFile {
        val code = rustCode {
            lines(filesNames.map { "mod $it" }, ";", ";")
            newline()
            lines(filesNames.map { "pub use $it::*" }, ";", ";")
        }

        return generateFile(code, modulePath, "mod.rs")
    }

    fun generateFile(content: CodeBlock, namespacePath: Path, fileName: String): CodegenFile {
        val code = rustCode {
            include(renderImports(fileName != "lib.rs"))
            newline()
            include(content)
            newline()
        }

        return CodegenFile(createPath(namespacePath, fileName), code.toString())
    }

    private fun renderImports(canImportCrate: Boolean): CodeBlock {
        return rustCode {
            -"use serde::{Deserialize, Serialize};"
            -"use serde::de::DeserializeOwned;"
            -"use serde_repr::{Deserialize_repr, Serialize_repr};"
            -"use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};"
            -"use std::collections::{BTreeSet, BTreeMap};"
            if (canImportCrate)
                -"use crate::*;"
        }
    }

    private fun createPath(namespacePath: Path, fileName: String): Path {
        return baseRelPath.resolve(namespacePath).resolve(fileName)
    }

    fun makeName(name: String): String =
        renames[name] ?: name

    fun isAliasRenderable(shapeId: ShapeId, type: Type): Boolean {
        if (!shapeId.namespace.startsWith("bsp")) return false
        if (shapeId.name in bannedAliases) return false
        if (type is Type.List) return false
        if (type is Type.Set) return false

        return true
    }

    fun renderPreDef(def: Def, hints: Boolean = true, untagged: Boolean = false): CodeBlock =
        rustCode {
            if (hints)
                include(renderHints(def.hints))
            include(deriveRenderer.renderForDef(def))
            include(serializationRenderer.renderForDef(def, untagged))
        }

    fun renderHints(hints: List<Hint>): CodeBlock =
        rustCode {
            lines(renderDocumentation(hints.filterIsInstance<Hint.Documentation>()))
            lines(renderDeprecated(hints.filterIsInstance<Hint.Deprecated>()))
        }

    private fun renderDocumentation(hints: List<Hint.Documentation>): List<String> {
        return hints.flatMap { el ->
            val docLines = el.string.split("\n").toMutableList()
            docLines[0] = docLines.first().let { "/** $it" }
            docLines[docLines.lastIndex] = docLines.last().let { "$it */" }
            docLines
        }
    }

    private fun renderDeprecated(hints: List<Hint.Deprecated>): List<String> {
        return hints.map { """#[deprecated(note = "${it.message}")]""" }
    }
}

data class Module(val moduleName: String, val definitions: List<Def>) {
    val path = Path(moduleName)
}
