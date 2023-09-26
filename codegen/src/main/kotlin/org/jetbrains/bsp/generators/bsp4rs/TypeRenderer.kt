package org.jetbrains.bsp.generators.bsp4rs

import org.jetbrains.bsp.generators.ir.Type

fun RustRenderer.renderIrShape(type: Type, isRequired: Boolean): String {
    if (isRequired) return renderType(type)

    return when (type) {
        is Type.TList, is Type.TMap, is Type.TSet -> renderType(type)
        else -> "Option<${renderType(type)}>"
    }
}

fun RustRenderer.renderType(type: Type): String = when (type) {
    is Type.TBool -> "bool"
    is Type.TInt -> "i32"
    is Type.TJson -> "serde_json::Value"
    is Type.TList -> "Vec<${renderType(type.member)}>"
    is Type.TLong -> "i64"
    is Type.TMap -> "BTreeMap<${renderType(type.key)}, ${renderType(type.value)}>"
    is Type.TSet -> "BTreeSet<${renderType(type.member)}>"
    is Type.TString -> "String"
    is Type.TUnit -> "()"
    is Type.TRef -> makeName(type.shapeId.name)
    is Type.TUntaggedUnion -> TODO()
}
