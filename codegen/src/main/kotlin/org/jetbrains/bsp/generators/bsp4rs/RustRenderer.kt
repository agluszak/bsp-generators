package org.jetbrains.bsp.generators.bsp4rs

import org.jetbrains.bsp.generators.CodegenFile
import org.jetbrains.bsp.generators.dsl.CodeBlock
import org.jetbrains.bsp.generators.dsl.rustCode
import org.jetbrains.bsp.generators.ir.*
import org.jetbrains.bsp.generators.utils.camelToSnakeCase
import software.amazon.smithy.model.shapes.ShapeId
import java.nio.file.Path
import kotlin.io.path.Path

class RustRenderer(basepkg: String, private val modules: List<Module>, val version: String) {
    private val baseRelPath = Path(basepkg.replace(".", "/"))
    val deriveRenderer = DeriveRenderer(modules.flatMap { it.definitions }.associateBy { it.shapeId })
    val serializationRenderer = SerializationRenderer()

    fun makeName(name: String): String {
        val renames: Map<String, String> = mapOf(Pair("type", "r#type"), Pair("r#version", "version"))

        return renames[name] ?: name
    }

    fun render(): List<CodegenFile> {
        val defFiles = modules.flatMap { renderModule(it) }
        val libFile = generateLibFile(modules.map { it.moduleName })
        return listOf(libFile) + defFiles
    }

    private fun renderModule(module: Module): List<CodegenFile> {
        val files = module.definitions.mapNotNull {
            renderDef(it)?.run {
                val name = makeName(it.name).camelToSnakeCase()
                Pair(generateFile(this, Path(module.moduleName), "$name.rs"), name)
            }
        }
        val modFile = generateModFile(module.moduleName, files.map { it.second })
        return files.unzip().first.toMutableList() + modFile
    }

    fun isAliasRenderable(shapeId: ShapeId, type: Type): Boolean {
        val bannedAliases: List<String> = listOf("Integer", "Long")

        if (!shapeId.namespace.startsWith("bsp")) return false
        if (shapeId.name in bannedAliases) return false
        if (type is Type.List) return false
        if (type is Type.Set) return false

        return true
    }

    private fun createPath(namespacePath: Path, fileName: String): Path {
        return baseRelPath.resolve(namespacePath).resolve(fileName)
    }

    private fun generateModFile(moduleName: String, filesNames: List<String>): CodegenFile {
        val code = rustCode {
            lines(filesNames.map { "mod $it" }, ";", ";")
            newline()
            lines(filesNames.map { "pub use $it::*" }, ";", ";")
        }

        return generateFile(code, Path(moduleName), "mod.rs")
    }

    private fun renderDef(def: Def): CodeBlock? {
        return when (def) {
            is Def.Structure -> renderStructure(def)
            is Def.OpenEnum<*> -> renderOpenEnum(def)
            is Def.ClosedEnum<*> -> renderClosedEnum(def)
            is Def.Service -> renderService(def)
            is Def.Alias -> renderAlias(def)
            is Def.DataKinds -> renderDataKinds(def)
        }
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

    fun renderHints(hints: List<Hint>): List<String> {
        return renderDocumentation(hints.filterIsInstance<Hint.Documentation>()) +
                renderDeprecated(hints.filterIsInstance<Hint.Deprecated>())
    }

    private fun renderImports(canImportCrate: Boolean): CodeBlock {
        return rustCode {
            -"use serde::{Deserialize, Serialize};"
            -"use serde::de::DeserializeOwned;"
            -"use serde_repr::{Deserialize_repr, Serialize_repr};"
            -"use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};"
            -"use std::collections::{BTreeSet, BTreeMap};"
            -(if (canImportCrate) "use crate::*;" else null)
        }
    }

    private fun renderOpenEnum(def: Def.OpenEnum<*>): CodeBlock {
        val name = def.name

        fun renderValue(ev: EnumValue<*>): String = when (def.enumType) {
            EnumType.IntEnum -> "${ev.value}"
            EnumType.StringEnum -> """"${ev.value}""""
        }

        fun renderEnumValue(ev: EnumValue<*>): String =
            "pub const ${makeName(ev.name.uppercase())}: $name = $name::new(${renderValue(ev)})"

        val values = def.values.map { value ->
            rustCode { lines(renderHints(value.hints) + renderEnumValue(value), end = ";") }
        }
        val type = when (def.enumType) {
            EnumType.IntEnum -> "i32"
            EnumType.StringEnum -> "std::borrow::Cow<'static, str>"
        }
        val newFun = when (def.enumType) {
            EnumType.IntEnum -> "pub const fn new(tag: i32) -> Self { $name(tag) }"
            EnumType.StringEnum -> "pub const fn new(tag: &'static str) -> Self { $name(std::borrow::Cow::Borrowed(tag)) }"
        }

        return rustCode {
            lines(renderHints(def.hints))
            -deriveRenderer.renderForDef(def)
            lines(serializationRenderer.renderForDef(def))
            -"pub struct $name(pub $type);"
            block("impl $name") {
                for (value in values) {
                    include(value)
                }
                newline()
                -newFun
            }
        }
    }
}
