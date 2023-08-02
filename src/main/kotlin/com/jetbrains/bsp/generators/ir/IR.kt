package com.jetbrains.bsp.generators.ir

import software.amazon.smithy.model.shapes.ShapeId

sealed interface Def {
    val name get() = shapeId.name
    val shapeId: ShapeId
    val hints: List<Hint>

    data class Alias(override val shapeId: ShapeId, val aliasedType: Type, override val hints: List<Hint>) : Def
    data class Structure(
        override val shapeId: ShapeId,
        val fields: List<Field>,
        override val hints: List<Hint>,
    ) : Def

    data class OpenEnum<A>(
        override val shapeId: ShapeId,
        val enumType: EnumType<A>,
        val values: List<EnumValue<A>>,
        override val hints: List<Hint>
    ) : Def

    data class ClosedEnum<A>(
        override val shapeId: ShapeId,
        val enumType: EnumType<A>,
        val values: List<EnumValue<A>>,
        override val hints: List<Hint>
    ) : Def

    data class Service(
        override val shapeId: ShapeId,
        val operations: List<Operation>,
        override val hints: List<Hint>
    ) : Def
}

sealed interface JsonRpcMethodType {
    object Request : JsonRpcMethodType
    object Notification : JsonRpcMethodType
}

data class Operation(
    val shapeId: ShapeId,
    val inputType: Type,
    val outputType: Type,
    val jsonRpcMethodType: JsonRpcMethodType,
    val jsonRpcMethod: String,
    val hints: List<Hint>
) {
    val name: String
        get() = shapeId.name
}

sealed interface EnumType<A> {
    object IntEnum : EnumType<Int>
    object StringEnum : EnumType<String>
}

data class EnumValue<A>(val name: String, val value: A, val hints: List<Hint>)

sealed interface Type {
    data class Set(val member: Type) : Type
    data class List(val member: Type) : Type
    data class Map(val key: Type, val value: Type) : Type
    data class Ref(val shapeId: ShapeId) : Type

    // Should be data objects
    object Unit : Type
    object Bool : Type
    object String : Type
    object Int : Type
    object Long : Type
    object Json : Type
}


data class Field(
    val name: String,
    val type: Type,
    val required: Boolean,
    val hints: List<Hint>
)

data class PolymorphicDataKind(val kind: String, val shapeId: ShapeId)

sealed interface Hint {
    data class Documentation(val string: String) : Hint
    data class Deprecated(val message: String) : Hint

    data class JsonRename(val name: String) : Hint
}

