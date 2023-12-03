package org.jetbrains.bsp.generators.bsp4json

import org.jetbrains.bsp.generators.ir.*
import kotlinx.serialization.json.*

const val TEST_BOOL = true
const val TEST_INT = 1
const val TEST_LONG = 2L
const val TEST_STRING = "test_string"
const val TEST_MAP_KEY = "test_key"

const val DEFAULT_BOOL = false
const val DEFAULT_INT = 0
const val DEFAULT_LONG = 0L
const val DEFAULT_STRING = ""
val DEFAULT_MAP = emptyMap<String, JsonElement>()
val DEFAULT_LIST = emptyList<JsonElement>()

enum class ContentsType {
    Default,
    TestValue
}

enum class NotRequiredState {
    Include,
    Exclude
}

class JsonRenderer2(val definitions: List<Def>) {
    val shapes = definitions.associateBy { it.shapeId }

    fun renderDefJsonString(def: Def, contents: ContentsType, notRequired: NotRequiredState): String =
        renderDefJson(def, contents, notRequired).toString()

    fun renderDefJson(def: Def, contents: ContentsType, notRequired: NotRequiredState): JsonElement =
        when (def) {
            is Def.Alias -> renderAliasJson(def, contents, notRequired)
            is Def.Structure -> renderStructureJson(def, contents, notRequired)
            is Def.ClosedEnum<*> -> renderClosedEnumJson(def, contents)
            is Def.OpenEnum<*> -> renderOpenEnumJson(def, contents)
            is Def.UntaggedUnion -> renderUntaggedUnionJson(def, contents, notRequired)
            else -> JsonNull
        }

    private fun renderStructureJson(def: Def.Structure, contents: ContentsType, notRequired: NotRequiredState): JsonElement {
        val allFields = notRequired == NotRequiredState.Include
        val filteredFields = def.fields.filter { allFields || it.required }

        val dataKindsFlattened = filteredFields.fold(emptyList<Field>()) { acc, field->
            if (field.type is Type.Ref && shapes[field.type.shapeId]!! is Def.DataKinds) {
                acc + Field("dataKind", Type.String, true, listOf()) + Field("data", Type.Json, true, listOf())
            } else {
                acc + field
            }
        }

        val fields = dataKindsFlattened.associate { field ->
            renderStructFieldNameJson(field) to renderTypeJson(field.type, contents, notRequired)
        }

        return JsonObject(fields)
    }

    private fun renderStructFieldNameJson(field: Field): String {
        val renamed: Hint.JsonRename? = field.hints.find { it is Hint.JsonRename } as Hint.JsonRename?
        return renamed?.name ?: field.name
    }

    private fun renderAliasJson(def: Def.Alias, contents: ContentsType, notRequired: NotRequiredState): JsonElement {
        return renderTypeJson(def.aliasedType, contents, notRequired)
    }

    private fun renderClosedEnumJson(def: Def.ClosedEnum<*>, contents: ContentsType): JsonElement =
        if (contents == ContentsType.Default) {
            renderEnumValueJson(def.values.first())
        } else {
            // TODO Kasia: do something smarter
            renderEnumValueJson(def.values.first())
        }

    private fun renderEnumValueJson(ev: EnumValue<*>): JsonElement = when (val value = ev.value) {
        is Int -> JsonPrimitive(value)
        is String -> JsonPrimitive(value)
        else -> JsonNull
    }

    private fun renderOpenEnumJson(def: Def.OpenEnum<*>, contents: ContentsType): JsonElement {
        val type = when (def.enumType) {
            EnumType.IntEnum -> Type.Int
            EnumType.StringEnum -> Type.String
        }
        return renderTypeJson(type, contents, NotRequiredState.Exclude)
    }

    private fun renderUntaggedUnionJson(
        def: Def.UntaggedUnion,
        contents: ContentsType,
        notRequired: NotRequiredState
    ): JsonElement = if (contents == ContentsType.Default) {
        def.members.first().let { renderTypeJson(it, contents, notRequired) }
    } else {
        // TODO Kasia: do something smarter
        def.members.first().let { renderTypeJson(it, contents, notRequired) }
    }

    private fun renderTypeJson(type: Type, contents: ContentsType, notRequired: NotRequiredState): JsonElement =
        when (contents) {
            ContentsType.Default -> when (type) {
                is Type.Unit -> JsonNull
                is Type.Bool -> JsonPrimitive(DEFAULT_BOOL)
                is Type.Int -> JsonPrimitive(DEFAULT_INT)
                is Type.Long -> JsonPrimitive(DEFAULT_LONG)
                is Type.String -> JsonPrimitive(DEFAULT_STRING)
                is Type.Json -> JsonObject(DEFAULT_MAP)
                is Type.List -> JsonArray(DEFAULT_LIST)
                is Type.Map -> JsonObject(DEFAULT_MAP)
                is Type.Set -> JsonArray(DEFAULT_LIST)
                is Type.Ref -> renderDefJson(shapes[type.shapeId]!!, contents, notRequired)
                else -> JsonNull
            }

            ContentsType.TestValue -> when (type) {
                is Type.Unit -> JsonNull
                is Type.Bool -> JsonPrimitive(TEST_BOOL)
                is Type.Int -> JsonPrimitive(TEST_INT)
                is Type.Long -> JsonPrimitive(TEST_LONG)
                is Type.String -> JsonPrimitive(TEST_STRING)
                is Type.Json -> JsonObject(mapOf(TEST_MAP_KEY to JsonPrimitive(TEST_LONG)))
                is Type.List -> JsonArray(listOf(renderTypeJson(type.member, contents, notRequired)))
                is Type.Map -> JsonObject(mapOf(TEST_MAP_KEY to renderTypeJson(type.value, contents, notRequired)))
                is Type.Set -> JsonArray(listOf(renderTypeJson(type.member, contents, notRequired)))
                is Type.Ref -> renderDefJson(shapes[type.shapeId]!!, contents, notRequired)
                else -> JsonNull
            }
        }
} 
