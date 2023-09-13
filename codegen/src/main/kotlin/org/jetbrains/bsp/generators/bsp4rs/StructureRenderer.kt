package org.jetbrains.bsp.generators.bsp4rs

import org.jetbrains.bsp.generators.dsl.CodeBlock
import org.jetbrains.bsp.generators.dsl.rustCode
import org.jetbrains.bsp.generators.ir.Def
import org.jetbrains.bsp.generators.ir.Field
import org.jetbrains.bsp.generators.ir.IrShape
import org.jetbrains.bsp.generators.utils.camelToSnakeCase

fun RustRenderer.renderStructure(def: Def.Structure): CodeBlock {
    return rustCode {
        lines(renderHints(def.hints))
        -deriveRenderer.renderForDef(def)
        lines(serializationRenderer.renderForDef(def))
        block("pub struct ${def.name}") {
            def.fields.forEach { field ->
                include(renderStructField(field))
            }
        }
    }
}

private fun RustRenderer.renderStructField(field: Field): CodeBlock {
    if (field.name == "dataKind" && field.type != IrShape.String) {
        return rustCode { }
    }

    return rustCode {
        lines(renderHints(field.hints))
        lines(serializationRenderer.renderForField(field))
        -"${renderStructFieldRaw(field)},"
    }
}

private fun RustRenderer.renderStructFieldRaw(field: Field): String {
    val fieldName = makeName(field.name).camelToSnakeCase()
    val fieldType = renderIrShape(field.type, field.required)

    return "pub $fieldName: $fieldType"
}
