package org.jetbrains.bsp.generators.bsp4rs.def

import org.jetbrains.bsp.generators.bsp4json.ContentsType
import org.jetbrains.bsp.generators.bsp4json.NotRequired
import org.jetbrains.bsp.generators.bsp4rs.RustRenderer
import org.jetbrains.bsp.generators.bsp4rs.renderIrShape
import org.jetbrains.bsp.generators.bsp4rs.renderIrShapeTest
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
    val fieldName = renderStructFieldName(field)
    val fieldType = renderIrShape(field.type, field.required)

    return "pub $fieldName: $fieldType"
}

private fun RustRenderer.renderStructFieldName(field: Field): String = makeName(field.name).camelToSnakeCase()

fun RustRenderer.renderStructureTest(def: Def.Structure): CodeBlock {
    val renderedTestValue = renderStructureTestValue(def)
    val renderedJson = jsonRenderer.renderStructureJson(def, ContentsType.TestOnlyPrimitive, NotRequired.Include)

    return rustCode {
        -"#[test]"
        block("fn ${def.name.camelToSnakeCase()}()") {
            -"let test_data = $renderedTestValue;"
            newline()
            -renderSerializationTest("test_data", renderedJson, false)
            newline()
            -renderDeserializationTest("test_data", renderedJson)
        }
    }
}

fun RustRenderer.renderStructureTestValue(def: Def.Structure): String {
    val renderedFields = def.fields.map { field ->
        renderStructFieldName(field) + ": " + renderIrShapeTest(field.type, field.required)
    }

    return "${def.name} {" + renderedFields.joinToString(", ") + "}"
}
