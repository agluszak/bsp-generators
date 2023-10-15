package org.jetbrains.bsp.generators.bsp4rs.def

import org.jetbrains.bsp.generators.bsp4rs.RustRenderer
import org.jetbrains.bsp.generators.bsp4rs.renderIrShape
import org.jetbrains.bsp.generators.bsp4rs.renderIrShapeTest
import org.jetbrains.bsp.generators.bsp4rs.renderTypeDefaultJson
import org.jetbrains.bsp.generators.bsp4rs.renderTypeJson
import org.jetbrains.bsp.generators.dsl.CodeBlock
import org.jetbrains.bsp.generators.dsl.rustCode
import org.jetbrains.bsp.generators.ir.Def
import org.jetbrains.bsp.generators.ir.Field
import org.jetbrains.bsp.generators.ir.Hint
import org.jetbrains.bsp.generators.ir.Type
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
    val fieldName = renderStructFieldName(field)
    val fieldType = renderIrShape(field.type, field.required)

    return "pub $fieldName: $fieldType"
}

private fun RustRenderer.renderStructFieldName(field: Field): String = makeName(field.name).camelToSnakeCase()

fun RustRenderer.renderStructureTest(def: Def.Structure): CodeBlock {
    val renderedTestValue = renderStructureTestValue(def)
    val renderedJson = renderStructureJson(def, true)

    return rustCode {
        -"#[test]"
        block("fn ${def.name.camelToSnakeCase()}()") {
            -renderSerializationTest(renderedTestValue, renderedJson)
        }
    }
}

fun RustRenderer.renderStructureTestValue(def: Def.Structure): String {
    val renderedFields = def.fields.map { field ->
        renderStructFieldName(field) + ": " + renderIrShapeTest(field.type, field.required)
    }

    return "${def.name} {" + renderedFields.joinToString(", ") + "}"
}

fun RustRenderer.renderStructureJson(def: Def.Structure, allFields: Boolean): String {
    val filteredFields = def.fields.filter { allFields || it.required }

    val renderedFields = filteredFields.map { field ->
        if (field.type is Type.Ref && shapes[field.type.shapeId]!! is Def.DataKinds) {
            renderDataKindsDefaultJson()
        } else {
            val jsonValue = if (allFields) renderTypeJson(field.type) else renderTypeDefaultJson(field.type)
            """"${renderStructFieldNameJson(field)}"""" + ": " + jsonValue
        }
    }

    return "{" + renderedFields.joinToString(", ") + "}"
}

fun RustRenderer.renderStructureDefaultJson(def: Def.Structure): String =
    renderStructureJson(def, false)

private fun renderStructFieldNameJson(field: Field): String {
    val renamed: Hint.JsonRename? = field.hints.find { it is Hint.JsonRename } as Hint.JsonRename?
    return renamed?.name ?: field.name
}
