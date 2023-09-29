package org.jetbrains.bsp.generators.bsp4rs

import org.jetbrains.bsp.generators.ir.Type

fun RustRenderer.renderIrShape(type: Type, isRequired: Boolean): String {
    if (isRequired) return renderType(type)

    return when (type) {
        is Type.List, is Type.Map, is Type.Set -> renderType(type)
        else -> "Option<${renderType(type)}>"
    }
}

fun RustRenderer.renderType(type: Type): String = when (type) {
    is Type.Bool -> "bool"
    is Type.Int -> "i32"
    is Type.Json -> "serde_json::Value"
    is Type.List -> "Vec<${renderType(type.member)}>"
    is Type.Long -> "i64"
    is Type.Map -> "BTreeMap<${renderType(type.key)}, ${renderType(type.value)}>"
    is Type.Set -> "BTreeSet<${renderType(type.member)}>"
    is Type.String -> "String"
    is Type.Unit -> "()"
    is Type.Ref -> makeName(type.shapeId.name)
    is Type.UntaggedUnion -> ""
}
