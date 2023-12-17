package org.jetbrains.bsp.generators.bsp4rs.def

import org.jetbrains.bsp.generators.bsp4json.ContentsType
import org.jetbrains.bsp.generators.bsp4json.NotRequired
import org.jetbrains.bsp.generators.bsp4rs.RustRenderer
import org.jetbrains.bsp.generators.bsp4rs.renderType
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
    val renderedJson = jsonRenderer.renderAliasJson(def, ContentsType.TestOnlyPrimitive, NotRequired.Include)

    return rustCode {
        -"#[test]"
        block("fn ${name.camelToSnakeCase()}()") {
            -"let test_data = $renderedTestValue;"
            newline()
            -renderSerializationTest("test_data", renderedJson, true)
            newline()
            -renderDeserializationTest("test_data", renderedJson)
        }
    }
}
