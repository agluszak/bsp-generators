package org.jetbrains.bsp.generators.bsp4rs.def

import org.jetbrains.bsp.generators.bsp4rs.RustRenderer
import org.jetbrains.bsp.generators.bsp4rs.renderType
import org.jetbrains.bsp.generators.dsl.CodeBlock
import org.jetbrains.bsp.generators.dsl.rustCode
import org.jetbrains.bsp.generators.ir.Def
import org.jetbrains.bsp.generators.ir.Type

fun RustRenderer.renderUntaggedUnion(def: Def.UntaggedUnion): CodeBlock? {
    return rustCode {
        include(renderPreDef(def, untagged = true))
        include(renderVariantsEnum(def.name, makeUnionOptions(def.members)))
    }
}

private fun RustRenderer.makeUnionOptions(types: List<Type>): List<Pair<String, String>> =
    types.map { type ->
        val renderedType = renderType(type)
        val name = renderedType.replaceFirstChar { it.uppercase() }

        Pair(name, renderedType)
    }

