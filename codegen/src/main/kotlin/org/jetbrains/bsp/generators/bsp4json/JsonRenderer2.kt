package org.jetbrains.bsp.generators.bsp4json

import org.jetbrains.bsp.generators.ir.*
import kotlinx.serialization.json.*

class JsonRenderer2(val definitions: List<Def>) {
    val shapes = definitions.associateBy { it.shapeId }

    // render def Json

    fun renderAliasJson(def: Def.Alias): JsonElement {
        return renderTypeJson(def.aliasedType)
    }

    fun renderStructureJson(def: Def.Structure, allFields: Boolean): JsonElement {
        val filteredFields = def.fields.filter { allFields || it.required }

        val dataKindsFlattened = filteredFields.foldRight(emptyList<Field>()) { field, acc ->
            if (field.type is Type.Ref && shapes[field.type.shapeId]!! is Def.DataKinds) {
                acc + Field("dataKind", Type.String, true, listOf()) + Field("data", Type.Json, true, listOf())
            } else {
                acc + field
            }
        }

        val fields = dataKindsFlattened.associate { field ->
            val jsonValue = if (allFields) renderTypeJson(field.type) else renderTypeDefaultJson(field.type)
            renderStructFieldNameJson(field) to jsonValue
        }

        return JsonObject(fields)
    }

    private fun renderStructFieldNameJson(field: Field): String {
        val renamed: Hint.JsonRename? = field.hints.find { it is Hint.JsonRename } as Hint.JsonRename?
        return renamed?.name ?: field.name
    }

    // render Def default json

    private fun renderDefDefaultJson(def: Def): JsonElement = when (def) {
        is Def.Alias -> renderAliasDefaultJson(def)
        is Def.Structure -> renderStructureDefaultJson(def)
        is Def.ClosedEnum<*> -> renderClosedEnumDefaultJson(def)
        is Def.OpenEnum<*> -> renderOpenEnumDefaultJson(def)
        is Def.UntaggedUnion -> renderUntaggedUnionDefaultJson(def)
        else -> JsonNull
    }

    private fun renderAliasDefaultJson(def: Def.Alias): JsonElement {
        return renderTypeDefaultJson(def.aliasedType)
    }

    private fun renderStructureDefaultJson(def: Def.Structure): JsonElement =
        renderStructureJson(def, false)

    private fun renderClosedEnumDefaultJson(def: Def.ClosedEnum<*>): JsonElement =
        renderEnumValueJson(def.values.first())

    fun renderEnumValueJson(ev: EnumValue<*>): JsonElement = when (val value = ev.value) {
        is Int -> JsonPrimitive(value)
        is String -> JsonPrimitive(value)
        else -> JsonNull
    }

    private fun renderOpenEnumDefaultJson(def: Def.OpenEnum<*>): JsonElement = when (def.enumType) {
        EnumType.IntEnum -> renderTypeDefaultJson(Type.Int)
        EnumType.StringEnum -> renderTypeDefaultJson(Type.String)
    }

    private fun renderUntaggedUnionDefaultJson(def: Def.UntaggedUnion): JsonElement = def.members.first().let {
        renderTypeDefaultJson(it)
    }

    // render Type json value

    // value in json of minimal not default type
    fun renderTypeJson(type: Type): JsonElement = when (type) {
        is Type.Unit -> JsonNull
        is Type.Bool -> renderTypeTestConstValue(Type.Bool)
        is Type.Int -> renderTypeTestConstValue(Type.Int)
        is Type.Long -> renderTypeTestConstValue(Type.Long)
        is Type.String -> renderTypeTestConstValue(Type.String)
        is Type.Json -> JsonObject(mapOf(renderTestMapKey() to renderTypeTestConstValue(Type.String)))
        is Type.List -> JsonArray(listOf(renderTypeJson(type.member)))
        is Type.Map -> JsonObject(mapOf(renderTestMapKey() to renderTypeJson(type.value)))
        is Type.Set -> JsonArray(listOf(renderTypeJson(type.member)))
        is Type.Ref -> renderDefDefaultJson(shapes[type.shapeId]!!)
        else -> JsonNull
    }

    // value in json of default type
    fun renderTypeDefaultJson(type: Type): JsonElement = when (type) {
        is Type.Unit -> JsonNull
        is Type.Bool -> JsonPrimitive(false)
        is Type.Int -> JsonPrimitive(0)
        is Type.Long -> JsonPrimitive(0)
        is Type.String -> JsonPrimitive("")
        is Type.Json -> JsonNull
        is Type.List -> JsonArray(emptyList())
        is Type.Map -> JsonObject(emptyMap())
        is Type.Set -> JsonArray(emptyList())
        is Type.Ref -> renderDefDefaultJson(shapes[type.shapeId]!!)
        else -> JsonNull
    }

    private fun renderTypeTestConstValue(type: Type): JsonElement = when (type) {
        is Type.Bool -> JsonPrimitive(true)
        is Type.Int -> JsonPrimitive(1)
        is Type.Long -> JsonPrimitive(2)
        is Type.String -> JsonPrimitive("test_string")
        else -> JsonNull
    }

    private fun renderTestMapKey(): String = "test_key"
}
