package org.jetbrains.bsp.generators.bsp4rs.def

import org.jetbrains.bsp.generators.bsp4rs.RustRenderer
import org.jetbrains.bsp.generators.bsp4rs.renderType
import org.jetbrains.bsp.generators.dsl.CodeBlock
import org.jetbrains.bsp.generators.dsl.rustCode
import org.jetbrains.bsp.generators.ir.Def
import org.jetbrains.bsp.generators.ir.Type

fun RustRenderer.renderAlias(def: Def.Alias): CodeBlock? {
    if (!isAliasRenderable(def.shapeId, def.aliasedType)) return null

    val name = def.name
    val type = this.renderType(def.aliasedType)

    return rustCode {
        lines(renderHints(def.hints))
        -deriveRenderer.renderForDef(def)
        lines(serializationRenderer.renderForDef(def))
        -"""pub struct $name(pub $type);"""
        newline()
        include(renderDerefForAlias(name, type))
        newline()
        include(renderFromForAlias(type, name, ""))
        if (def.aliasedType is Type.String) {
            newline()
            include(renderFromForAlias("&str", name, ".to_string()"))
        }
    }
}

private fun renderDerefForAlias(name: String, type: String): CodeBlock =
    rustCode {
        block("""impl std::ops::Deref for $name""") {
            -"""type Target = $type;"""
            newline()
            block("fn deref(&self) -> &Self::Target") {
                -"&self.0"
            }
        }
    }

private fun renderFromForAlias(from: String, name: String, fn: String): CodeBlock =
    rustCode {
        block("""impl From<$from> for $name""") {
            block("fn from(input: $from) -> Self") {
                -"Self(input$fn)"
            }
        }
    }
