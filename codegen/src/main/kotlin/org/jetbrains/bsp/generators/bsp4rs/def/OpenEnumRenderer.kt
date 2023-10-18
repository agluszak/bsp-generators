package org.jetbrains.bsp.generators.bsp4rs.def

import org.jetbrains.bsp.generators.bsp4rs.RustRenderer
import org.jetbrains.bsp.generators.bsp4rs.renderEnumValueJson
import org.jetbrains.bsp.generators.bsp4rs.renderTypeDefaultJson
import org.jetbrains.bsp.generators.dsl.CodeBlock
import org.jetbrains.bsp.generators.dsl.rustCode
import org.jetbrains.bsp.generators.ir.Def
import org.jetbrains.bsp.generators.ir.EnumType
import org.jetbrains.bsp.generators.ir.EnumValue
import org.jetbrains.bsp.generators.ir.Type

fun RustRenderer.renderOpenEnum(def: Def.OpenEnum<*>): CodeBlock {
    val name = def.name

    return rustCode {
        include(renderPreDef(def))
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

    return rustCode {
        include(renderHints(ev.hints))
        -"pub const $enumValueName: $enumName = $enumName::new(${renderEnumValueJson(ev)});"
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

fun RustRenderer.renderOpenEnumTest(def: Def.OpenEnum<*>): CodeBlock =
    renderEnumTest(def.name, def.values) { it.uppercase() }

fun RustRenderer.renderOpenEnumDefaultJson(def: Def.OpenEnum<*>): String = when (def.enumType) {
    EnumType.IntEnum -> renderTypeDefaultJson(Type.Int)
    EnumType.StringEnum -> renderTypeDefaultJson(Type.String)
}
