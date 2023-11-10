package org.jetbrains.bsp.generators.bsp4json

import org.jetbrains.bsp.generators.ir.*
import software.amazon.smithy.model.shapes.ShapeId

class JsonRenderer(val definitions: List<Def>) {
    private val otherDataDef = Def.Structure(
        ShapeId.fromParts("bsp", "OtherData"),
        listOf(
            Field("dataKind", Type.String, true, listOf()),
            Field("data", Type.Json, true, listOf())
        ),
        listOf()
    )
    val shapes = definitions.plus(otherDataDef).associateBy { it.shapeId }

    // render def Json

    fun renderAliasJson(def: Def.Alias): String {
        return renderTypeJson(def.aliasedType)
    }

    fun renderStructureJson(def: Def.Structure, allFields: Boolean): String {
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

    private fun renderStructFieldNameJson(field: Field): String {
        val renamed: Hint.JsonRename? = field.hints.find { it is Hint.JsonRename } as Hint.JsonRename?
        return renamed?.name ?: field.name
    }

    private fun renderDataKindsDefaultJson(): String =
        renderDefDefaultJson(otherDataDef).drop(1).dropLast(1)

    // render Def default json

    private fun renderDefDefaultJson(def: Def): String = when (def) {
        is Def.Alias -> renderAliasDefaultJson(def)
        is Def.Structure -> renderStructureDefaultJson(def)
        is Def.ClosedEnum<*> -> renderClosedEnumDefaultJson(def)
        is Def.OpenEnum<*> -> renderOpenEnumDefaultJson(def)
        is Def.UntaggedUnion -> renderUntaggedUnionDefaultJson(def)
        else -> ""
    }

    private fun renderAliasDefaultJson(def: Def.Alias): String {
        return renderTypeDefaultJson(def.aliasedType)
    }

    private fun renderStructureDefaultJson(def: Def.Structure): String =
        renderStructureJson(def, false)

    private fun renderClosedEnumDefaultJson(def: Def.ClosedEnum<*>): String =
        renderEnumValueJson(def.values.first())

    fun renderEnumValueJson(ev: EnumValue<*>): String = when (val value = ev.value) {
        is Int -> "$value"
        is String -> """"$value""""
        else -> ""
    }

    private fun renderOpenEnumDefaultJson(def: Def.OpenEnum<*>): String = when (def.enumType) {
        EnumType.IntEnum -> renderTypeDefaultJson(Type.Int)
        EnumType.StringEnum -> renderTypeDefaultJson(Type.String)
    }

    private fun renderUntaggedUnionDefaultJson(def: Def.UntaggedUnion): String = def.members.first().let {
        renderTypeDefaultJson(it)
    }

    // render Type json value

    // value in json of minimal not default type
    fun renderTypeJson(type: Type): String = when (type) {
        is Type.Unit -> "null"
        is Type.Bool -> renderTypeTestConstValue(Type.Bool)
        is Type.Int -> renderTypeTestConstValue(Type.Int)
        is Type.Long -> renderTypeTestConstValue(Type.Long)
        is Type.String -> renderTypeTestConstValue(Type.String)
        is Type.Json -> "{${renderTypeJson(Type.String)}: ${renderTypeJson(Type.String)}}"
        is Type.List -> "[${renderTypeJson(type.member)}]"
        is Type.Map -> "{${renderTypeJson(type.key)}: ${renderTypeJson(type.value)}}"
        is Type.Set -> "[${renderTypeJson(type.member)}]"
        is Type.Ref -> renderDefDefaultJson(shapes[type.shapeId]!!)
        else -> ""
    }

    // value in json of default type
    fun renderTypeDefaultJson(type: Type): String = when (type) {
        is Type.Unit -> "null"
        is Type.Bool -> "false"
        is Type.Int -> "0"
        is Type.Long -> "0"
        is Type.String -> """"""""
        is Type.Json -> "null"
        is Type.List -> "[]"
        is Type.Map -> "{}"
        is Type.Set -> "[]"
        is Type.Ref -> renderDefDefaultJson(shapes[type.shapeId]!!)
        else -> ""
    }

    private fun renderTypeTestConstValue(type: Type): String = when (type) {
        is Type.Bool -> "true"
        is Type.Int -> "1"
        is Type.Long -> "2"
        is Type.String -> """"test_string""""
        else -> ""
    }
}
