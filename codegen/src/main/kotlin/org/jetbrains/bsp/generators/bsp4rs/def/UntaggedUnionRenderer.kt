package org.jetbrains.bsp.generators.bsp4rs.def

import org.jetbrains.bsp.generators.bsp4rs.RustRenderer
import org.jetbrains.bsp.generators.bsp4rs.renderType
import org.jetbrains.bsp.generators.bsp4rs.renderTypeJson
import org.jetbrains.bsp.generators.bsp4rs.renderTypeTest
import org.jetbrains.bsp.generators.dsl.CodeBlock
import org.jetbrains.bsp.generators.dsl.rustCode
import org.jetbrains.bsp.generators.ir.Def
import org.jetbrains.bsp.generators.ir.Type
import org.jetbrains.bsp.generators.utils.camelCaseUpperCamelCase
import org.jetbrains.bsp.generators.utils.camelToSnakeCase

fun RustRenderer.renderUntaggedUnion(def: Def.UntaggedUnion): CodeBlock {
    return rustCode {
        include(renderPreDef(def, untagged = true))
        include(renderVariantsEnum(def.name, makeUnionOptions(def.members)))
    }
}

private fun RustRenderer.makeUnionOptions(types: List<Type>): List<Pair<String, String>> =
    types.map { type ->
        val renderedType = renderType(type)
        val name = renderedType.camelCaseUpperCamelCase()

        Pair(name, renderedType)
    }

fun RustRenderer.renderUntaggedUnionTest(def: Def.UntaggedUnion): CodeBlock {
    val name = def.name
    fun renderMemberTest(type: Type): String {
        val enumValueName = renderType(type).camelCaseUpperCamelCase()
        val renderedTestValue = "$name::$enumValueName(${renderTypeTest(type)})"
        val renderedJson = renderTypeJson(type)

        return """assert_compact_json_snapshot!($renderedTestValue, @r#"$renderedJson"#);"""
    }

    return rustCode {
        -"#[test]"
        block("fn ${name.camelToSnakeCase()}()") {
            def.members.forEach { type ->
                -renderMemberTest(type)
            }
        }
    }
}
