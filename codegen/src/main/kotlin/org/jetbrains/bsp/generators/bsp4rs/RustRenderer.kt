package org.jetbrains.bsp.generators.bsp4rs

import org.jetbrains.bsp.generators.CodegenFile
import org.jetbrains.bsp.generators.bsp4rs.def.renderAlias
import org.jetbrains.bsp.generators.bsp4rs.def.renderClosedEnum
import org.jetbrains.bsp.generators.bsp4rs.def.renderClosedEnumTest
import org.jetbrains.bsp.generators.bsp4rs.def.renderDataKinds
import org.jetbrains.bsp.generators.bsp4rs.def.renderOpenEnum
import org.jetbrains.bsp.generators.bsp4rs.def.renderOpenEnumTest
import org.jetbrains.bsp.generators.bsp4rs.def.renderService
import org.jetbrains.bsp.generators.bsp4rs.def.renderStructure
import org.jetbrains.bsp.generators.bsp4rs.def.renderUntaggedUnion
import org.jetbrains.bsp.generators.dsl.CodeBlock
import org.jetbrains.bsp.generators.dsl.rustCode
import org.jetbrains.bsp.generators.ir.Def
import org.jetbrains.bsp.generators.ir.EnumValue
import org.jetbrains.bsp.generators.ir.Hint
import org.jetbrains.bsp.generators.utils.camelToSnakeCase
import org.jetbrains.bsp.generators.utils.printEnumValue
import java.nio.file.Path
import kotlin.io.path.Path

class RustRenderer(basepkg: String, private val modules: List<Module>, val version: String) {
    private val baseRelPath = Path(basepkg.replace(".", "/"))
    val deriveRenderer = DeriveRenderer(modules.flatMap { it.definitions }.associateBy { it.shapeId })
    val serializationRenderer = SerializationRenderer()

    private val renames: Map<String, String> = mapOf(Pair("type", "r#type"), Pair("r#version", "version"))

    fun render(): List<CodegenFile> {
        val defFiles = modules.flatMap { renderModule(it) }
        val libFile = generateLibFile(modules.map { it.moduleName })
        return defFiles + libFile
    }

    private fun renderModule(module: Module): List<CodegenFile> {
        val filesWithNames = module.definitions.mapNotNull {
            renderDefCodeBlock(it).run {
                val name = makeName(it.name).camelToSnakeCase()
                Pair(generateFile(this, module.path, "$name.rs"), name)
            }
        }

        val modFile = generateModFile(module.path, filesWithNames.map { (_, defName) -> defName })
        val files = filesWithNames.map { (file, _) -> file }

        return files + modFile
    }

    private fun renderDefCodeBlock(def: Def): CodeBlock =
        rustCode {
            include(renderImports(true))
            newline()
            include(renderDef(def))
            newline()
            -"#[cfg(test)]"
            block("mod tests") {
                include(renderTestsImports())
                newline()
                renderDefTest(def)?.let {
                    -"#[test]"
                    include(it)
                }
            }
            newline()
        }

    private fun renderDef(def: Def): CodeBlock = when (def) {
        is Def.Structure -> renderStructure(def)
        is Def.OpenEnum<*> -> renderOpenEnum(def)
        is Def.ClosedEnum<*> -> renderClosedEnum(def)
        is Def.Service -> renderService(def)
        is Def.Alias -> renderAlias(def)
        is Def.DataKinds -> renderDataKinds(def)
        is Def.UntaggedUnion -> renderUntaggedUnion(def)
    }

    private fun renderDefTest(def: Def): CodeBlock? = when (def) {
        is Def.ClosedEnum<*> -> renderClosedEnumTest(def)
        is Def.OpenEnum<*> -> renderOpenEnumTest(def)
        else -> null
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
        return CodegenFile(createPath(namespacePath, fileName), content.toString())
    }

    fun renderImports(canImportCrate: Boolean): CodeBlock {
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

    private fun renderTestsImports(): CodeBlock {
        return rustCode {
            -"use insta::assert_json_snapshot;"
            -"use crate::tests::test_deserialization;"
            -"use super::*;"
        }
    }

    private fun createPath(namespacePath: Path, fileName: String): Path {
        return baseRelPath.resolve(namespacePath).resolve(fileName)
    }

    fun makeName(name: String): String =
        renames[name] ?: name

    fun renderVariantsEnum(name: String, values: List<Pair<String, String>>): CodeBlock =
        rustCode {
            block("pub enum $name") {
                lines(values.map { "${it.first}(${it.second})" }, ",", ",")
            }
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
            el.string.split("\n").map { it.prependIndent("/// ") }
        }
    }

    private fun renderDeprecated(hints: List<Hint.Deprecated>): List<String> {
        return hints.map { """#[deprecated(note = "${it.message}")]""" }
    }

    fun renderEnumTest(name: String, values: List<EnumValue<*>>, fn: (String) -> String): CodeBlock {
        return rustCode {
            block("fn ${name.camelToSnakeCase()}()") {
                values.forEach { value ->
                    val enumValueName = fn(makeName(value.name))
                    -"""assert_json_snapshot!($name::$enumValueName, @r#"${printEnumValue(value.value)}"#);"""
                }
            }
        }
    }
}

data class Module(val moduleName: String, val definitions: List<Def>) {
    val path = Path(moduleName)
}
