package org.jetbrains.bsp.generators.bsp4rs.def

import org.jetbrains.bsp.generators.bsp4rs.RustRenderer
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

private fun RustRenderer.renderEnumValue(ev: EnumValue<*>): CodeBlock {
    val enumValueName = makeName(ev.name.snakeToUpperCamelCase())

    val enumVal = when (ev.value) {
        is Int -> "$enumValueName = ${ev.value}"
        is String -> enumValueName
        else -> ""
    }

    return rustCode {
        lines(renderHints(ev.hints))
        -"$enumVal,"
    }
}
