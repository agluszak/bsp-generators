package com.jetbrains.bsp.generators.bsp4rs

import com.jetbrains.bsp.generators.CodegenFile
import com.jetbrains.bsp.generators.dsl.CodeBlock
import com.jetbrains.bsp.generators.dsl.code
import com.jetbrains.bsp.generators.ir.Def
import com.jetbrains.bsp.generators.ir.Field
import com.jetbrains.bsp.generators.ir.Type
import kotlin.io.path.Path

class RustRenderer(val basepkg: String, val definitions: List<Def>, val version: String) {
    val baseRelPath = Path(basepkg.replace(".", "/"))

    fun render(): List<CodegenFile> = TODO()

    fun renderFieldSerialization(field: Field): String? {
        if (field.required) return null

        return when (field.type) {
            is Type.List -> """#[serde(default, skip_serializing_if = "Vec::is_empty")]"""
            is Type.Map -> """#[serde(default, skip_serializing_if = "HashMap::is_empty")]"""
            is Type.Set -> """#[serde(default, skip_serializing_if = "BTreeSet::is_empty")]"""
            else -> """#[serde(skip_serializing_if = "Option::is_none")]"""
        }
    }

    fun renderFieldRaw(field: Field): String {
        var result = "pub ${field.name}: "
        result += if (field.required) renderType(field.type) else renderOptionalType(field.type)
        result += ","

        return result
    }

    fun renderRustField(field: Field): CodeBlock {
        val serde = renderFieldSerialization(field)

        return code {
            -serde
            -renderFieldRaw(field)
        }
    }

    fun renderOptionalType(type: Type): String {
        return when (type) {
            is Type.List, is Type.Map, is Type.Set -> renderType(type)
            else -> "Option<${renderType(type)}>"
        }
    }

    fun renderType(type: Type): String = when (type) {
        Type.Bool -> "bool"
        Type.Int -> "i32"
        Type.Json -> "serde_json::Value"
        is Type.List -> "Vec<${renderType(type.member)}>"
        Type.Long -> "i64"
        is Type.Map -> "HashMap<${renderType(type.key)}, ${renderType(type.value)}>"
        is Type.Ref -> type.shapeId.name
        is Type.Set -> "BTreeSet<${renderType(type.member)}>"
        Type.String -> "String"
        Type.Unit -> "()"
    }
}
