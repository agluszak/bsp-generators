package org.jetbrains.bsp.generators.bsp4rs.def

import org.jetbrains.bsp.generators.bsp4rs.RustRenderer
import org.jetbrains.bsp.generators.bsp4rs.renderType
import org.jetbrains.bsp.generators.bsp4rs.renderTypeDefaultJson
import org.jetbrains.bsp.generators.bsp4rs.renderTypeJson
import org.jetbrains.bsp.generators.bsp4rs.renderTypeTest
import org.jetbrains.bsp.generators.dsl.CodeBlock
import org.jetbrains.bsp.generators.dsl.rustCode
import org.jetbrains.bsp.generators.ir.Def
import org.jetbrains.bsp.generators.ir.Type
import org.jetbrains.bsp.generators.utils.camelToSnakeCase

fun RustRenderer.renderAlias(def: Def.Alias): CodeBlock {
    val name = def.name
    val type = this.renderType(def.aliasedType)

    return rustCode {
        include(renderPreDef(def))
        -"""pub struct $name(pub $type);"""
        newline()
        include(renderConstructor(name, type))
        newline()
        include(renderDeref(name, type))
        if (def.aliasedType is Type.String) {
            newline()
            include(renderFrom("&str", name, ".to_string()"))
        }
    }
}

private fun renderConstructor(name: String, from: String): CodeBlock =
    rustCode {
        block("""impl $name""") {
            block("pub fn new(input: $from) -> Self") {
                -"Self(input)"
            }
        }
    }

private fun renderDeref(name: String, type: String): CodeBlock =
    rustCode {
        block("""impl std::ops::Deref for $name""") {
            -"""type Target = $type;"""
            newline()
            block("fn deref(&self) -> &Self::Target") {
                -"&self.0"
            }
        }
    }

private fun renderFrom(from: String, name: String, fn: String): CodeBlock =
    rustCode {
        block("""impl From<$from> for $name""") {
            block("fn from(input: $from) -> Self") {
                -"Self(input$fn)"
            }
        }
    }

fun RustRenderer.renderAliasTest(def: Def.Alias): CodeBlock {
    val name = def.name
    val renderedTestValue = "$name(${renderTypeTest(def.aliasedType)})"
    val renderedJson = renderAliasJson(def)

    return rustCode {
        -"#[test]"
        block("fn ${name.camelToSnakeCase()}()") {
            -renderSerializationTest(renderedTestValue, renderedJson)
        }
    }
}

fun RustRenderer.renderAliasJson(def: Def.Alias): String {
    return renderTypeJson(def.aliasedType)
}

fun RustRenderer.renderAliasDefaultJson(def: Def.Alias): String {
    return renderTypeDefaultJson(def.aliasedType)
}