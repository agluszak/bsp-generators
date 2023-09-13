package org.jetbrains.bsp.generators.bsp4rs.def

import org.jetbrains.bsp.generators.bsp4rs.RustRenderer
import org.jetbrains.bsp.generators.dsl.CodeBlock
import org.jetbrains.bsp.generators.dsl.rustCode
import org.jetbrains.bsp.generators.ir.Def
import org.jetbrains.bsp.generators.ir.EnumType
import org.jetbrains.bsp.generators.ir.EnumValue

fun RustRenderer.renderOpenEnum(def: Def.OpenEnum<*>): CodeBlock {
    val name = def.name

    return rustCode {
        include(renderHints(def.hints))
        include(deriveRenderer.renderForDef(def))
        include(serializationRenderer.renderForDef(def))
        -"pub struct $name(pub ${renderType(def.enumType)});"
        newline()
        block("impl $name") {
            def.values.forEach { value ->
                include(renderEnumValue(value, name))
            }
            newline()
            include(renderConstructor(def.enumType))
        }
    }
}

private fun renderType(type: EnumType<*>): String = when (type) {
    EnumType.IntEnum -> "i32"
    EnumType.StringEnum -> "std::borrow::Cow<'static, str>"
}

private fun RustRenderer.renderEnumValue(ev: EnumValue<*>, enumName: String): CodeBlock {
    val enumValueName = makeName(ev.name).uppercase()

    val enumValue = when (ev.value) {
        is Int -> "${ev.value}"
        is String -> """"${ev.value}""""
        else -> ""
    }

    return rustCode {
        include(renderHints(ev.hints))
        -"pub const $enumValueName: $enumName = $enumName::new($enumValue);"
    }
}

private fun renderConstructor(type: EnumType<*>): CodeBlock =
    when (type) {
        EnumType.IntEnum -> rustCode {
            block("pub const fn new(tag: i32) -> Self") {
                -"Self(tag)"
            }
        }

        EnumType.StringEnum -> rustCode {
            block("pub const fn new(tag: &'static str) -> Self") {
                -"Self(std::borrow::Cow::Borrowed(tag))"
            }
        }
    }
