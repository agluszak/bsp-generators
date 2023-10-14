package org.jetbrains.bsp.generators.bsp4rs

import org.jetbrains.bsp.generators.ir.Type

fun renderEnumValueJson(value: Any?): String = when (value) {
    is Int -> "$value"
    is String -> """"$value""""
    else -> ""
}

// normal type

fun RustRenderer.renderIrShape(type: Type, isRequired: Boolean): String =
    if (isRequired) renderType(type)
    else "Option<${renderType(type)}>"

fun RustRenderer.renderType(type: Type): String = when (type) {
    is Type.Unit -> "()"
    is Type.Bool -> "bool"
    is Type.Int -> "i32"
    is Type.Long -> "i64"
    is Type.String -> "String"
    is Type.Json -> "serde_json::Value"
    is Type.List -> "Vec<${renderType(type.member)}>"
    is Type.Map -> "BTreeMap<${renderType(type.key)}, ${renderType(type.value)}>"
    is Type.Set -> "BTreeSet<${renderType(type.member)}>"
    is Type.Ref -> makeName(type.shapeId.name)
    is Type.UntaggedUnion -> ""
}

// const values

fun renderTypeTestConstName(type: Type): String = when (type) {
    is Type.Bool -> "TEST_BOOL"
    is Type.Int -> "TEST_INT"
    is Type.Long -> "TEST_LONG"
    is Type.String -> "TEST_STRING"
    else -> ""
}

// value in test

fun RustRenderer.renderTypeTest(type: Type): String = when (type) {
    is Type.Unit -> "()"
    is Type.Bool -> renderTypeTestConstName(Type.Bool)
    is Type.Int -> renderTypeTestConstName(Type.Int)
    is Type.Long -> renderTypeTestConstName(Type.Long)
    is Type.String -> renderTypeTestConstName(Type.String) + ".to_string()"
    is Type.Json -> "serde_json::json!({${renderTypeTestConstName(Type.String)}: ${renderTypeTestConstName(Type.String)}})"
    is Type.List -> "vec![${renderTypeTest(type.member)}]"
    is Type.Map -> "BTreeMap::from([(${renderTypeTest(type.key)}, ${renderTypeTest(type.value)})])"
    is Type.Set -> "BTreeSet::from([${renderTypeTest(type.member)}])"
    is Type.Ref -> makeName(type.shapeId.name) + "::default()"
    else -> ""
}

// value in json of minimal not default type

fun RustRenderer.renderTypeJson(type: Type): String = when (type) {
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

fun RustRenderer.renderTypeDefaultJson(type: Type): String = when (type) {
    is Type.Unit -> "null"
    is Type.Bool -> "false"
    is Type.Int -> "0"
    is Type.Long -> "0"
    is Type.String -> """"""""
    is Type.Json -> "{}"
    is Type.List -> "[]"
    is Type.Map -> "{}"
    is Type.Set -> "[]"
    is Type.Ref -> renderDefDefaultJson(shapes[type.shapeId]!!)
    else -> ""
}
