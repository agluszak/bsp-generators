package org.jetbrains.bsp.generators.bsp4rs

import com.google.gson.GsonBuilder
import com.google.gson.JsonParser
import org.jetbrains.bsp.generators.CodegenFile
import org.jetbrains.bsp.generators.bsp4json.JsonRenderer
import org.jetbrains.bsp.generators.bsp4rs.def.renderDef
import org.jetbrains.bsp.generators.bsp4rs.def.renderDefTest
import org.jetbrains.bsp.generators.dsl.CodeBlock
import org.jetbrains.bsp.generators.dsl.rustCode
import org.jetbrains.bsp.generators.ir.Def
import org.jetbrains.bsp.generators.ir.EnumValue
import org.jetbrains.bsp.generators.ir.Field
import org.jetbrains.bsp.generators.ir.Hint
import org.jetbrains.bsp.generators.ir.Type
import org.jetbrains.bsp.generators.utils.camelToSnakeCase
import software.amazon.smithy.model.shapes.ShapeId
import java.nio.file.Path
import kotlin.io.path.Path

class RustRenderer(basepkg: String, private val modules: List<Module>, val version: String) {
    private val baseRelPath = Path(basepkg.replace(".", "/"))
    val otherDataDef = Def.Structure(
        ShapeId.fromParts("bsp", "OtherData"),
        listOf(
            Field("dataKind", Type.String, true, listOf()),
            Field("data", Type.Json, true, listOf())
        ),
        listOf()
    )
    val shapes = modules.flatMap { it.definitions }.plus(otherDataDef).associateBy { it.shapeId }

    val deriveRenderer = DeriveRenderer(shapes)
    val serializationRenderer = SerializationRenderer()
    val jsonRenderer = JsonRenderer(modules.flatMap { it.definitions })

    private val renames: Map<String, String> = mapOf(Pair("type", "r#type"), Pair("r#version", "version"))

    fun render(): List<CodegenFile> {
        val defFiles = modules.flatMap { renderModule(it) }
        val libFile = generateLibFile(modules.map { it.moduleName })
        return defFiles + libFile
    }

    private fun renderModule(module: Module): List<CodegenFile> {
        val filesWithNames = module.definitions.map {
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
                include(renderTestsImports(true))
                newline()
                include(renderDefTest(def))
            }
            newline()
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

    fun renderTestsImports(canImportCrate: Boolean): CodeBlock {
        return rustCode {
            -"use super::*;"
            if (canImportCrate)
                -"use crate::tests::*;"
            -"use insta::assert_compact_json_snapshot;"
            -"use insta::assert_json_snapshot;"
            -"use serde::Deserialize;"
        }
    }

    private fun createPath(namespacePath: Path, fileName: String): Path {
        return baseRelPath.resolve(namespacePath).resolve(fileName)
    }

    fun makeName(name: String): String =
        renames[name] ?: name

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

    fun renderVariantsEnum(name: String, values: List<Pair<String, String>>): CodeBlock =
        rustCode {
            block("pub enum $name") {
                lines(values.map { "${it.first}(${it.second})" }, ",", ",")
            }
        }

    fun renderEnumTest(name: String, values: List<EnumValue<*>>, fn: (String) -> String): CodeBlock {
        fun renderEnumValueTest(value: EnumValue<*>): CodeBlock {
            val enumValueName = fn(makeName(value.name))
            val renderedTestValue = "$name::$enumValueName"
            val renderedJson = jsonRenderer.renderEnumValueJson(value)

            return rustCode {
                -renderSerializationTest(renderedTestValue, renderedJson, true)
                -renderDeserializationTest(renderedTestValue, renderedJson)
            }
        }

        return rustCode {
            -"#[test]"
            block("fn ${name.camelToSnakeCase()}()") {
                values.forEach { value ->
                    include(renderEnumValueTest(value))
                    newline()
                }
            }
        }
    }

    fun renderSerializationTest(testedValue: String, expectedJson: String, isCompact: Boolean): String {
        if (isCompact) {
            return """assert_compact_json_snapshot!(
            |   $testedValue, 
            |   @r#"$expectedJson"#
            |);""".trimMargin()
        }

        val jsonElement = JsonParser.parseString(expectedJson)
        val gson = GsonBuilder().setPrettyPrinting().serializeNulls().create()
        val prettyJson = gson.toJson(jsonElement)

        return """assert_json_snapshot!($testedValue,
            |@r#"
            |$prettyJson
            |"#);""".trimMargin()
    }

    fun renderDeserializationTest(testedValue: String, expectedJson: String): String {
        return """test_deserialization(
            |   r#"$expectedJson"#,
            |   &$testedValue
            |);""".trimMargin()
    }
}

data class Module(val moduleName: String, val definitions: List<Def>) {
    val path = Path(moduleName)
}
