package com.jetbrains.bsp.generators.bsp4rs

import com.jetbrains.bsp.generators.CodegenFile
import com.jetbrains.bsp.generators.dsl.CodeBlock
import com.jetbrains.bsp.generators.dsl.code
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
        return code {
            code(renderImports())
            block("pub trait Request") {
                -"type Params: DeserializeOwned + Serialize;"
                -"type Result: DeserializeOwned + Serialize;"
                -"const METHOD: &'static str;"
            }
            newline()
            block("pub trait Notification") {
                -"type Params: DeserializeOwned + Serialize;"
                -"const METHOD: &'static str;"
            }
            newline()
        }
    }

    private fun renderDef(def: Def): CodegenFile? {
        return when (def) {
            is Def.Structure -> generateStructureFile(def)
            is Def.Service -> generateServiceFile(def)
            else -> null
        }
    }

    private fun generateStructureFile(def: Def.Structure): CodegenFile {
        val code = code {
            code(renderImports())
            code(renderStructure(def))
        }
        return CodegenFile(rustFileName(def.name), code.toString())
    }

    fun renderDocumentation(hints: List<Hint>): CodeBlock {
        val doc = hints.filterIsInstance<Hint.Documentation>()

        return code {
            for (el in doc) {
                val docLines = el.string.split("\n").toMutableList()
                docLines[0] = docLines.first().let { "/** $it" }
                docLines[docLines.lastIndex] = docLines.last().let { "$it */" }

                for (line in docLines) {
                    -line
                }
            }
        }
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

    fun renderFieldRaw(field: Field): String {
        return "pub ${field.name.camelToSnakeCase()}: ${renderType(field.type, field.required)},"
    }

    private fun renderRustField(field: Field): CodeBlock {
        return code {
            code(renderDocumentation(field.hints))
            -renderFieldSerialization(field)
            -renderFieldRaw(field)
        }
    }

    private fun renderImports(): CodeBlock {
        return code {
            -"use cargo_metadata::Edition;"
            -"use serde::{Deserialize, Serialize};"
            -"use serde::de::DeserializeOwned;"
            -"use serde_repr::{Deserialize_repr, Serialize_repr};"
            -"use std::collections::{BTreeSet, HashMap};"
            newline()
        }
    }

    private fun renderStructure(def: Def.Structure): CodeBlock {
        return code {
            code(renderDocumentation(def.hints))
            -"#[derive(Debug, PartialEq, Serialize, Deserialize, Default, Clone)]"
            -"""#[serde(rename_all = "camelCase")]"""
            block("pub struct ${def.name}") {
                def.fields.forEach { field ->
                    code(renderRustField(field))
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
            JsonRpcMethodType.Request -> "type Result = ${renderType(op.outputType, true)};"
        }

        return code {
            -"#[derive(Debug)]"
            -"pub enum $name {}"
            newline()
            block("impl ${renderJsonRpcMethodType(op.jsonRpcMethodType)} for $name") {
                -"type Params = ${renderType(op.inputType, true)};"
                -output
                -"""const METHOD: &'static str = "${op.jsonRpcMethod}";"""
            }
            newline()
        }
    }

    private fun generateServiceFile(def: Def.Service): CodegenFile {
        val code = code {
            code(renderImports())
            def.operations.forEach { operation ->
                code(renderOperation(operation))
            }
        }

        return CodegenFile(rustFileName(def.name), code.toString())
    }
}

fun String.camelToSnakeCase(): String {
    val pattern = "(?<=.)[A-Z]".toRegex()
    return this.replace(pattern, "_$0").lowercase()
}
