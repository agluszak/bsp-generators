package org.jetbrains.bsp.generators.bsp4rs.def

import org.jetbrains.bsp.generators.bsp4rs.RustRenderer
import org.jetbrains.bsp.generators.dsl.CodeBlock
import org.jetbrains.bsp.generators.dsl.rustCode
import org.jetbrains.bsp.generators.ir.Def
import org.jetbrains.bsp.generators.ir.EnumValue
import org.jetbrains.bsp.generators.utils.snakeToUpperCamelCase

fun RustRenderer.renderClosedEnum(def: Def.ClosedEnum<*>): CodeBlock =
    rustCode {
        include(renderPreDef(def))
        block("pub enum ${def.name}") {
            def.values.first().let { firstValue ->
                include(renderEnumValue(firstValue, true))
            }
            def.values.drop(1).forEach { value ->
                include(renderEnumValue(value, false))
            }
        }
    }

private fun RustRenderer.renderEnumValue(ev: EnumValue<*>, is_first: Boolean): CodeBlock {
    val enumValueName = makeName(ev.name).snakeToUpperCamelCase()

    val enumVal = when (ev.value) {
        is Int -> "$enumValueName = ${ev.value}"
        is String -> enumValueName
        else -> ""
    }

    return rustCode {
        include(renderHints(ev.hints))
        if (is_first) -"#[default]"
        -"$enumVal,"
    }
}

fun RustRenderer.renderClosedEnumTest(def: Def.ClosedEnum<*>): CodeBlock =
    renderEnumTest(def.name, def.values) { it.snakeToUpperCamelCase() }
