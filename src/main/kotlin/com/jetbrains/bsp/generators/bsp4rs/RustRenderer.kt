package com.jetbrains.bsp.generators.bsp4rs

import com.jetbrains.bsp.generators.CodegenFile
import com.jetbrains.bsp.generators.dsl.CodeBlock
import com.jetbrains.bsp.generators.dsl.code
import com.jetbrains.bsp.generators.ir.Def
import com.jetbrains.bsp.generators.ir.Field
import com.jetbrains.bsp.generators.ir.Hint
import com.jetbrains.bsp.generators.ir.Type
import kotlin.io.path.Path

class RustRenderer(basepkg: String, private val definitions: List<Def>, val version: String) {
    private val baseRelPath = Path(basepkg.replace(".", "/"))

    fun render(): List<CodegenFile> {
        return definitions.mapNotNull { renderDef(it) }
    }

    private fun renderDef(def: Def): CodegenFile? {
        return when (def) {
            is Def.Structure -> generateStructureFile(def)
            else -> null
        }
    }

    private fun generateStructureFile(def: Def.Structure): CodegenFile {
        val name = def.name
        val file = baseRelPath.resolve("${name}.rs")
        val code = code {
            code(renderImports())
            code(renderStructure(def))
        }
        return CodegenFile(file, code.toString())
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
}

fun String.camelToSnakeCase(): String {
    val pattern = "(?<=.)[A-Z]".toRegex()
    return this.replace(pattern, "_$0").lowercase()
}
