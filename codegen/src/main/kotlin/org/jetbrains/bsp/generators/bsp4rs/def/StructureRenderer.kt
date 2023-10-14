package org.jetbrains.bsp.generators.bsp4rs.def

import org.jetbrains.bsp.generators.bsp4rs.RustRenderer
import org.jetbrains.bsp.generators.bsp4rs.renderIrShape
import org.jetbrains.bsp.generators.bsp4rs.renderTypeDefaultJson
import org.jetbrains.bsp.generators.dsl.CodeBlock
import org.jetbrains.bsp.generators.dsl.rustCode
import org.jetbrains.bsp.generators.ir.Def
import org.jetbrains.bsp.generators.ir.Field
import org.jetbrains.bsp.generators.utils.camelToSnakeCase

fun RustRenderer.renderStructure(def: Def.Structure): CodeBlock {
    return rustCode {
        include(renderPreDef(def))
        block("pub struct ${def.name}") {
            def.fields.forEach { field ->
                include(renderStructField(field))
            }
        }
    }
}

private fun RustRenderer.renderStructField(field: Field): CodeBlock =
    rustCode {
        include(renderHints(field.hints))
        include(serializationRenderer.renderForField(field))
        -"${renderStructFieldRaw(field)},"
    }

private fun RustRenderer.renderStructFieldRaw(field: Field): String {
    val fieldName = makeName(field.name).camelToSnakeCase()
    val fieldType = renderIrShape(field.type, field.required)

    return "pub $fieldName: $fieldType"
}

fun RustRenderer.renderStructureDefaultJson(def: Def.Structure): String {
    val filteredFields = def.fields.filter { it.required }
    val renderedFields = filteredFields.map { field ->
        """"${field.name}"""" + ": " + renderTypeDefaultJson(field.type)
    }

    return "{" + renderedFields.joinToString(", ") + "}"
}
