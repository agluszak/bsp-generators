package org.jetbrains.bsp.generators.bsp4rs.def

import org.jetbrains.bsp.generators.bsp4rs.RustRenderer
import org.jetbrains.bsp.generators.bsp4rs.renderType
import org.jetbrains.bsp.generators.bsp4rs.renderTypeDefault
import org.jetbrains.bsp.generators.dsl.CodeBlock
import org.jetbrains.bsp.generators.dsl.rustCode
import org.jetbrains.bsp.generators.ir.Def
import org.jetbrains.bsp.generators.ir.PolymorphicDataKind
import org.jetbrains.bsp.generators.ir.Type
import org.jetbrains.bsp.generators.utils.camelToSnakeCase
import org.jetbrains.bsp.generators.utils.kebabToSnakeCase
import org.jetbrains.bsp.generators.utils.kebabToUpperCamelCase

fun RustRenderer.renderDataKinds(def: Def.DataKinds): CodeBlock {
    val namedName = "Named${def.name}"
    val dataKinds = makeDataKindsList(def.kinds)
    val wrapperEnum = listOf(Pair("Named", namedName), Pair("Other", "OtherData"))

    return rustCode {
        -"#[allow(clippy::large_enum_variant)]"
        newline()
        include(renderPreDef(def, hints = false))
        include(renderVariantsEnum(namedName, dataKinds))
        newline()
        include(renderPreDef(def, untagged = true))
        include(renderVariantsEnum(def.name, wrapperEnum))
        newline()
        include(renderDataKindsImpl(def.name, namedName, dataKinds))
    }
}

private fun RustRenderer.makeDataKindsList(irKinds: List<PolymorphicDataKind>): List<Pair<String, String>> =
    irKinds.map { kind ->
        val name = makeName(kind.kind).kebabToUpperCamelCase()
        val dataType = renderType(kind.shape)

        Pair(name, dataType)
    }

private fun renderDataKindsImpl(
    enumName: String,
    namedName: String,
    values: List<Pair<String, String>>
): CodeBlock =
    rustCode {
        block("impl $enumName") {
            values.forEach { (name, dataType) ->
                block("pub fn ${name.camelToSnakeCase()}(data: $dataType) -> Self") {
                    -"Self::Named($namedName::$name(data))"
                }
            }
        }
    }

fun RustRenderer.renderDataKindsTest(def: Def.DataKinds): CodeBlock {
    val enumName = def.name
    return rustCode {
        -"#[test]"
        block("fn ${enumName.camelToSnakeCase()}()") {
            def.kinds.forEach { data ->
                -renderDataKindTest(enumName, data)
                newline()
            }
            -renderOtherDataKindTest(enumName)
        }
    }
}

private fun RustRenderer.renderDataKindTest(enumName: String, data: PolymorphicDataKind): String {
    val dataName = makeName(data.kind).kebabToSnakeCase()
    val renderedTestValue = "$enumName::$dataName(${renderTypeDefault(data.shape)})"
    val renderedJson = """{"dataKind": "${data.kind}", "data": ${jsonRenderer.renderTypeDefaultJson(data.shape)}}"""

    return renderSerializationTest(renderedTestValue, renderedJson, false)
}

private fun RustRenderer.renderOtherDataKindTest(enumName: String): String {
    val otherType = Type.Ref(otherDataDef.shapeId)
    val renderedTestValue = "$enumName::Other(${renderTypeDefault(otherType)})"
    val renderedJson = jsonRenderer.renderTypeDefaultJson(otherType)

    return renderSerializationTest(renderedTestValue, renderedJson, true)
}

fun RustRenderer.renderDataKindsDefault(def: Def.DataKinds): String =
    "${def.name}::Other(${renderDefDefault(otherDataDef)})"
