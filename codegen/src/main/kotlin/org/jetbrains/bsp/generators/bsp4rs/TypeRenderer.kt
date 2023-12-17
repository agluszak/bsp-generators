package org.jetbrains.bsp.generators.bsp4rs

import org.jetbrains.bsp.generators.bsp4rs.def.renderDefDefault
import org.jetbrains.bsp.generators.ir.Type

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

fun RustRenderer.renderIrShapeTest(type: Type, isRequired: Boolean): String =
    if (isRequired) renderTypeTest(type)
    else "Some(${renderTypeTest(type)})"

fun RustRenderer.renderTypeTest(type: Type): String = when (type) {
    is Type.Unit -> "()"
    is Type.Bool -> renderTypeTestConstName(Type.Bool)
    is Type.Int -> renderTypeTestConstName(Type.Int)
    is Type.Long -> renderTypeTestConstName(Type.Long)
    is Type.String -> renderTypeTestConstName(Type.String) + ".to_string()"
    is Type.Json -> "serde_json::json!({})"
    is Type.List -> "vec![${renderTypeDefault(type.member)}]"
    is Type.Map -> "BTreeMap::from([(${renderTypeDefault(type.key)}, ${renderTypeDefault(type.value)})])"
    is Type.Set -> "BTreeSet::from([${renderTypeDefault(type.member)}])"
    is Type.Ref -> renderDefDefault(shapes[type.shapeId]!!)
    else -> ""
}

fun RustRenderer.renderTypeDefault(type: Type): String {
    val typeName = when (type) {
        is Type.List -> "Vec::<${renderType(type.member)}>"
        is Type.Map -> "BTreeMap::<${renderType(type.key)}, ${renderType(type.value)}>"
        is Type.Set -> "BTreeSet::<${renderType(type.member)}>"
        else -> renderType(type)
    }

    return "$typeName::default()"
}
