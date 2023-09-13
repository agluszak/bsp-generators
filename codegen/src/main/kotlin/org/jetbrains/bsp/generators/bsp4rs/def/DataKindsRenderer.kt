package org.jetbrains.bsp.generators.bsp4rs.def

import org.jetbrains.bsp.generators.bsp4rs.RustRenderer
import org.jetbrains.bsp.generators.bsp4rs.renderIrShapeType
import org.jetbrains.bsp.generators.dsl.CodeBlock
import org.jetbrains.bsp.generators.dsl.rustCode
import org.jetbrains.bsp.generators.ir.Def
import org.jetbrains.bsp.generators.ir.PolymorphicDataKind
import org.jetbrains.bsp.generators.utils.camelToSnakeCase
import org.jetbrains.bsp.generators.utils.kebabToUpperCamelCase

fun RustRenderer.renderDataKinds(def: Def.DataKinds): CodeBlock {
    val namedName = "Named${def.name}"
    val dataKinds = makeDataKindsList(def.kinds)
    val wrapperEnum = listOf(Pair("Named", namedName), Pair("Other", "OtherData"))
    val derives = deriveRenderer.renderForDef(def)

    return rustCode {
        include(derives)
        include(serializationRenderer.renderForDef(def))
        include(renderDataKindsEnum(namedName, dataKinds))
        newline()
        include(renderHints(def.hints))
        -"#[allow(clippy::large_enum_variant)]"
        include(derives)
        include(serializationRenderer.renderForDef(def, true))
        include(renderDataKindsEnum(def.name, wrapperEnum))
        newline()
        include(renderDataKindsImpl(def.name, namedName, dataKinds))
    }
}

private fun RustRenderer.makeDataKindsList(irKinds: List<PolymorphicDataKind>): List<Pair<String, String>> =
    irKinds.map { kind ->
        val name = makeName(kind.kind).kebabToUpperCamelCase()
        val dataType = renderIrShapeType(kind.shape)

        Pair(name, dataType)
    }

private fun renderDataKindsEnum(name: String, values: List<Pair<String, String>>): CodeBlock =
    rustCode {
        block("pub enum $name") {
            lines(values.map { "${it.first}(${it.second})" }, ",", ",")
        }
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
