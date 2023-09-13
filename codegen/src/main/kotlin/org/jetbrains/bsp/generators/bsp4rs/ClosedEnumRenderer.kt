package org.jetbrains.bsp.generators.bsp4rs

import org.jetbrains.bsp.generators.dsl.CodeBlock
import org.jetbrains.bsp.generators.dsl.rustCode
import org.jetbrains.bsp.generators.ir.Def
import org.jetbrains.bsp.generators.ir.EnumValue
import org.jetbrains.bsp.generators.utils.snakeToUpperCamelCase

fun RustRenderer.renderClosedEnum(def: Def.ClosedEnum<*>): CodeBlock =
    rustCode {
        lines(renderHints(def.hints))
        -deriveRenderer.renderForDef(def)
        lines(serializationRenderer.renderForDef(def))
        block("pub enum ${def.name}") {
            if (def.values.isNotEmpty())
                -"#[default]"

            def.values.forEach { value ->
                include(renderEnumValue(value))
            }
        }
    }

private fun RustRenderer.renderEnumValue(value: EnumValue<*>): CodeBlock {
    val enumValueName = makeName(value.name.snakeToUpperCamelCase())

    val enumVal = when (value.value) {
        is Int -> "$enumValueName = ${value.value}"
        is String -> enumValueName
        else -> ""
    }

    return rustCode {
        lines(renderHints(value.hints))
        -"$enumVal,"
    }
}
