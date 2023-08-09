package com.jetbrains.bsp.generators.bsp4rs

import com.jetbrains.bsp.generators.CodegenFile
import com.jetbrains.bsp.generators.dsl.CodeBlock
import com.jetbrains.bsp.generators.dsl.rustCode
import com.jetbrains.bsp.generators.ir.*
import java.nio.file.Path
import kotlin.io.path.Path

class RustRenderer(basepkg: String, private val definitions: List<Def>, val version: String) {
    private val baseRelPath = Path(basepkg.replace(".", "/"))

    fun render(): List<CodegenFile> {
        val defFiles = definitions.mapNotNull { renderDef(it) }
        val libFile = CodegenFile(rustFileName("lib"), renderRpcTraits().toString())
        return listOf(libFile) + defFiles
    }

    private fun rustFileName(name: String): Path {
        return baseRelPath.resolve("${name.camelToSnakeCase()}.rs")
    }

    private fun renderRpcTraits(): CodeBlock {
        val paramsStr = "type Params: DeserializeOwned + Serialize"
        val resultStr = "type Result: DeserializeOwned + Serialize"
        val methodStr = "const METHOD: &'static str"

        return rustCode {
            include(renderImports())
            block("pub trait Request") {
                lines(listOf(paramsStr, resultStr, methodStr), ";", ";")
            }
            newline()
            block("pub trait Notification") {
                lines(listOf(paramsStr, methodStr), ";", ";")
            }
            newline()
        }
    }

    private fun renderDef(def: Def): CodegenFile? {
        return when (def) {
            is Def.Structure -> generateFile(renderStructure(def), def.name)
            is Def.OpenEnum<*> -> generateFile(renderOpenEnum(def), def.name)
            is Def.ClosedEnum<*> -> generateFile(renderClosedEnum(def), def.name)
            is Def.Service -> generateServiceFile(def)
            else -> null
        }
    }

    private fun generateFile(content: CodeBlock, name: String): CodegenFile {
        val code = rustCode {
            include(renderImports())
            include(content)
        }
        return CodegenFile(rustFileName(name), code.toString())
    }

    private fun renderDocumentation(hints: List<Hint.Documentation>): List<String> {
        return hints.flatMap { el ->
            val docLines = el.string.split("\n").toMutableList()
            docLines[0] = docLines.first().let { "/** $it" }
            docLines[docLines.lastIndex] = docLines.last().let { "$it */" }
            docLines
        }
    }

    //    TODO
    private fun renderDeprecated(hints: List<Hint.Deprecated>): List<String> {
        return emptyList()
    }

    //    TODO
    private fun renderRename(hints: List<Hint.JsonRename>): List<String> {
        return emptyList()
    }

    fun renderHints(hints: List<Hint>): List<String> {
        return renderDocumentation(hints.filterIsInstance<Hint.Documentation>()) +
                renderDeprecated(hints.filterIsInstance<Hint.Deprecated>()) +
                renderRename(hints.filterIsInstance<Hint.JsonRename>())
    }

    fun renderFieldSerialization(field: Field): String? {
        if (field.required) return null

        return when (field.type) {
            is Type.List -> """#[serde(default, skip_serializing_if = "Vec::is_empty")]"""
            is Type.Map -> """#[serde(default, skip_serializing_if = "HashMap::is_empty")]"""
            is Type.Set -> """#[serde(default, skip_serializing_if = "BTreeSet::is_empty")]"""
            else -> """#[serde(skip_serializing_if = "Option::is_none")]"""
        }
    }

    private fun renderTypeName(type: Type): String = when (type) {
        Type.Bool -> "bool"
        Type.Int -> "i32"
        Type.Json -> "serde_json::Value"
        is Type.List -> "Vec<${renderTypeName(type.member)}>"
        Type.Long -> "i64"
        is Type.Map -> "HashMap<${renderTypeName(type.key)}, ${renderTypeName(type.value)}>"
        is Type.Ref -> type.shapeId.name
        is Type.Set -> "BTreeSet<${renderTypeName(type.member)}>"
        Type.String -> "String"
        Type.Unit -> "()"
    }

    fun renderType(type: Type, isRequired: Boolean): String {
        if (isRequired) return renderTypeName(type)

        return when (type) {
            is Type.List, is Type.Map, is Type.Set -> renderTypeName(type)
            else -> "Option<${renderTypeName(type)}>"
        }
    }

    fun renderStructFieldRaw(field: Field): String {
        return "pub ${field.name.camelToSnakeCase()}: ${renderType(field.type, field.required)}"
    }

    private fun renderStructField(field: Field): CodeBlock {
        return rustCode {
            lines(renderHints(field.hints))
            -renderFieldSerialization(field)
            -"${renderStructFieldRaw(field)},"
        }
    }

    private fun renderImports(): CodeBlock {
        return rustCode {
            -"use cargo_metadata::Edition;"
            -"use serde::{Deserialize, Serialize};"
            -"use serde::de::DeserializeOwned;"
            -"use serde_repr::{Deserialize_repr, Serialize_repr};"
            -"use std::collections::{BTreeSet, HashMap};"
            newline()
        }
    }

    private fun renderStructure(def: Def.Structure): CodeBlock {
        return rustCode {
            lines(renderHints(def.hints))
            -"#[derive(Debug, PartialEq, Serialize, Deserialize, Default, Clone)]"
            -"""#[serde(rename_all = "camelCase")]"""
            block("pub struct ${def.name}") {
                def.fields.forEach { field ->
                    include(renderStructField(field))
                }
            }
            newline()
        }
    }

    private fun renderJsonRpcMethodType(type: JsonRpcMethodType): String {
        return when (type) {
            JsonRpcMethodType.Notification -> "Notification"
            JsonRpcMethodType.Request -> "Request"
        }
    }

    fun renderOperation(op: Operation): CodeBlock {
        val name = op.name
        val output = when (op.jsonRpcMethodType) {
            JsonRpcMethodType.Notification -> null
            JsonRpcMethodType.Request -> "type Result = ${renderType(op.outputType, true)}"
        }
        val operationProperties = listOfNotNull(
            "type Params = ${renderType(op.inputType, true)}",
            output,
            """const METHOD: &'static str = "${op.jsonRpcMethod}""""
        )

        return rustCode {
            -"#[derive(Debug)]"
            -"pub enum $name {}"
            newline()
            block("impl ${renderJsonRpcMethodType(op.jsonRpcMethodType)} for $name") {
                lines(operationProperties, join = ";", end = ";")
            }
            newline()
        }
    }

    private fun generateServiceFile(def: Def.Service): CodegenFile {
        val code = rustCode {
            include(renderImports())
            def.operations.forEach { operation ->
                include(renderOperation(operation))
            }
        }

        return CodegenFile(rustFileName(def.name), code.toString())
    }

    private fun renderEnumValue(enumType: EnumType<*>): (EnumValue<*>) -> String {
        return when (enumType) {
            EnumType.IntEnum -> { ev: EnumValue<*> -> "${ev.name.snakeToUpperCamelCase()} = ${ev.value}" }
            EnumType.StringEnum -> { ev: EnumValue<*> -> ev.name.snakeToUpperCamelCase() }
        }
    }

    private fun renderEnumSerialization(enumType: EnumType<*>): CodeBlock {
        return when (enumType) {
            EnumType.IntEnum -> rustCode {
                -"#[derive(Debug, PartialEq, Serialize_repr, Deserialize_repr, Clone)]"
                -"#[repr(u8)]"
            }

            EnumType.StringEnum -> rustCode {
                -"#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]"
                -"""#[serde(rename_all = "kebab-case")]"""
            }
        }
    }

    fun renderClosedEnum(def: Def.ClosedEnum<*>): CodeBlock {
        val values = def.values.map { value ->
            rustCode {
                lines(renderHints(def.hints) + renderEnumValue(def.enumType)(value), end = ",")
            }
        }
        return rustCode {
            lines(renderHints(def.hints))
            include(renderEnumSerialization(def.enumType))
            block("pub enum ${def.name}") {
                for (value in values) {
                    include(value)
                }
            }
        }
    }

    // TODO - this is a copy of renderClosedEnum, we need to figure out how it should be changed
    fun renderOpenEnum(def: Def.OpenEnum<*>): CodeBlock {
        val values = def.values.map { value ->
            rustCode {
                lines(renderHints(def.hints) + renderEnumValue(def.enumType)(value), end = ",")
            }
        }
        return rustCode {
            lines(renderHints(def.hints))
            include(renderEnumSerialization(def.enumType))
            block("pub enum ${def.name}") {
                for (value in values) {
                    include(value)
                }
            }
        }
    }
}

fun String.camelToSnakeCase(): String {
    val pattern = "(?<=.)[A-Z]".toRegex()
    return this.replace(pattern, "_$0").lowercase()
}

fun String.snakeToUpperCamelCase(): String {
    val pattern = "_(.)".toRegex()
    return this.lowercase().replace(pattern) { it.groupValues[1].uppercase() }
        .replaceFirstChar { it.uppercaseChar() }
}
