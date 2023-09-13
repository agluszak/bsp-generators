package org.jetbrains.bsp.generators.bsp4rs

import org.jetbrains.bsp.generators.ir.IrShape
import org.jetbrains.bsp.generators.ir.Type

fun RustRenderer.renderIrShape(shape: IrShape, isRequired: Boolean): String {
    if (isRequired) return renderIrShapeType(shape)

    return when (shape.type) {
        is Type.List, is Type.Map, is Type.Set -> renderIrShapeType(shape)
        else -> "Option<${renderIrShapeType(shape)}>"
    }
}

fun RustRenderer.renderIrShapeType(irShape: IrShape): String {
    if (irShape.type is Type.Ref)
        return makeName(irShape.shapeId.name)

    if (irShape.shapeId.namespace.startsWith("bsp") && isAliasRenderable(irShape.shapeId, irShape.type))
        return makeName(irShape.shapeId.name)

    return renderType(irShape.type)
}

fun RustRenderer.renderType(type: Type): String = when (type) {
    is Type.Bool -> "bool"
    is Type.Int -> "i32"
    is Type.Json -> "serde_json::Value"
    is Type.List -> "Vec<${renderIrShapeType(type.member)}>"
    is Type.Long -> "i64"
    is Type.Map -> "BTreeMap<${renderIrShapeType(type.key)}, ${renderIrShapeType(type.value)}>"
    is Type.Set -> "BTreeSet<${renderIrShapeType(type.member)}>"
    is Type.String -> "String"
    is Type.Unit -> "()"
    else -> ""
}

