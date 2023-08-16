package com.jetbrains.bsp.generators.bsp4rs

import com.jetbrains.bsp.generators.CodegenFile
import com.jetbrains.bsp.generators.dsl.CodeBlock
import com.jetbrains.bsp.generators.dsl.rustCode
import com.jetbrains.bsp.generators.ir.*
import java.nio.file.Path
import kotlin.io.path.Path

class RustRenderer(basepkg: String, private val modules: List<Module>, val version: String) {
    private val baseRelPath = Path(basepkg.replace(".", "/"))
    val renames: Map<String, String> = mapOf(Pair("type", "r#type"), Pair("r#version", "version"))

    fun makeName(name: String): String {
        return renames[name] ?: name
    }

    fun render(): List<CodegenFile> {
        val defFiles = modules.flatMap { renderModule(it) }
        val libFile = generateLibFile(modules.map { it.moduleName })
        return listOf(libFile) + defFiles
    }

    fun renderModule(module: Module): List<CodegenFile> {
        val files = module.definitions.mapNotNull {
            renderDef(it)?.run {
                val name = makeName(it.name).camelToSnakeCase()
                Pair(generateFile(this, Path(module.moduleName), "$name.rs"), name)
            }
        }
        val modFile = generateModFile(module.moduleName, files.map { it.second })
        return files.unzip().first.toMutableList() + modFile
    }

    private fun createPath(namespacePath: Path, fileName: String): Path {
        return baseRelPath.resolve(namespacePath).resolve(fileName)
    }

    private fun generateLibFile(modulesNames: List<String>): CodegenFile {
        val code = rustCode {
            lines(modulesNames.map { "pub mod $it" }, ";", ";")
            newline()
            lines(modulesNames.map { "use $it::*" }, ";", ";")
            newline()
            include(renderRpcTraits())
        }

        return generateFile(code, Path(""), "lib.rs")
    }

    private fun generateModFile(moduleName: String, filesNames: List<String>): CodegenFile {
        val code = rustCode {
            lines(filesNames.map { "mod $it" }, ";", ";")
            newline()
            lines(filesNames.map { "pub use $it::*" }, ";", ";")
        }

        return generateFile(code, Path(moduleName), "mod.rs")
    }

    private fun renderRpcTraits(): CodeBlock {
        val paramsStr = "type Params: DeserializeOwned + Serialize"
        val resultStr = "type Result: DeserializeOwned + Serialize"
        val methodStr = "const METHOD: &'static str"

        return rustCode {
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

    private fun renderDef(def: Def): CodeBlock? {
        return when (def) {
            is Def.Structure -> renderStructure(def)
            is Def.OpenEnum<*> -> renderOpenEnum(def)
            is Def.ClosedEnum<*> -> renderClosedEnum(def)
            is Def.Service -> renderServiceFile(def)
            else -> null
        }
    }

    private fun generateFile(content: CodeBlock, namespacePath: Path, fileName: String): CodegenFile {
        val code = rustCode {
            include(renderImports(fileName != "lib.rs"))
            include(content)
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

    //    TODO
//    private fun renderDeprecated(hints: List<Hint.Deprecated>): List<String> {
//        return emptyList()
//    }

    //    TODO
//    private fun renderRename(hints: List<Hint.JsonRename>): List<String> {
//        return emptyList()
//    }

    fun renderHints(hints: List<Hint>): List<String> {
        return renderDocumentation(hints.filterIsInstance<Hint.Documentation>())
//                + renderDeprecated(hints.filterIsInstance<Hint.Deprecated>()) + renderRename(hints.filterIsInstance<Hint.JsonRename>())
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
        is Type.Ref -> makeName(type.shapeId.name)
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
        return "pub ${makeName(field.name).camelToSnakeCase()}: ${renderType(field.type, field.required)}"
    }

    private fun renderStructField(field: Field): CodeBlock {
        return rustCode {
            lines(renderHints(field.hints))
            -renderFieldSerialization(field)
            -"${renderStructFieldRaw(field)},"
        }
    }

    private fun renderImports(canImportCrate: Boolean): CodeBlock {
        return rustCode {
            -"use cargo_metadata::Edition;"
            -"use serde::{Deserialize, Serialize};"
            -"use serde::de::DeserializeOwned;"
            -"use serde_repr::{Deserialize_repr, Serialize_repr};"
            -"use std::collections::{BTreeSet, HashMap};"
            -(if (canImportCrate) "use crate::*;" else null)
            newline()
        }
    }

    private fun renderBasicDerives(isDefault: Boolean): String {
        return """Debug, PartialEq, Clone""" + if (isDefault) ", Default" else ""
    }

    private fun renderSerializeDerives(isDefault: Boolean): String {
        return if (isDefault) """Serialize, Deserialize""" else "Serialize_repr, Deserialize_repr"
    }

    private fun renderStructure(def: Def.Structure): CodeBlock {
        return rustCode {
            lines(renderHints(def.hints))
            -"#[derive(${renderBasicDerives(true)}, ${renderSerializeDerives(true)})]"
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
        val name = makeName(op.name)
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

    private fun renderServiceFile(def: Def.Service): CodeBlock {
        return rustCode {
            def.operations.forEach { operation ->
                include(renderOperation(operation))
            }
        }
    }

    private fun renderEnumValue(enumType: EnumType<*>): (EnumValue<*>) -> String {
        val genName: (String) -> String = { makeName(it.snakeToUpperCamelCase()) }
        return when (enumType) {
            EnumType.IntEnum -> { ev: EnumValue<*> -> "${genName(ev.name)} = ${ev.value}" }
            EnumType.StringEnum -> { ev: EnumValue<*> -> genName(ev.name) }
        }
    }

    private fun renderEnumSerialization(isDefault: Boolean, enumType: EnumType<*>): CodeBlock {
        return when (enumType) {
            EnumType.IntEnum -> rustCode {
                -"#[derive(${renderBasicDerives(isDefault)}, ${renderSerializeDerives(false)})]"
                -"#[repr(u8)]"
            }

            EnumType.StringEnum -> rustCode {
                -"#[derive(${renderBasicDerives(isDefault)}, ${renderSerializeDerives(true)})]"
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
        val isDefault = values.isNotEmpty()
        return rustCode {
            lines(renderHints(def.hints))
            include(renderEnumSerialization(isDefault, def.enumType))
            block("pub enum ${def.name}") {
                if (isDefault) {
                    -"#[default]"
                }
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
        val isDefault = values.isNotEmpty()
        return rustCode {
            lines(renderHints(def.hints))
            include(renderEnumSerialization(isDefault, def.enumType))
            block("pub enum ${def.name}") {
                if (isDefault) {
                    -"#[default]"
                }
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
