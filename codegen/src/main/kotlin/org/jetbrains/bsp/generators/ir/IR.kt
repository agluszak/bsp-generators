package org.jetbrains.bsp.generators.ir

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

    data class DataKinds(
        override val shapeId: ShapeId,
        val kindsEnumId: ShapeId,
        val kinds: List<PolymorphicDataKind>,
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

sealed interface InnerType {
    data class Set(val member: Type) : InnerType
    data class List(val member: Type) : InnerType
    data class Map(val key: Type, val value: Type) : InnerType
    object Ref : InnerType

    // Should be data objects
    object Unit : InnerType
    object Bool : InnerType
    object String : InnerType
    object Int : InnerType
    object Long : InnerType
    object Json : InnerType
}

data class Type(val shapeId: ShapeId, val type: InnerType) {
    companion object {
        val Unit: Type = Type(ShapeId.from("smithy.api#Unit"), InnerType.Unit)
        val Bool: Type= Type(ShapeId.from("smithy.api#Boolean"), InnerType.Bool)
        val String: Type = Type(ShapeId.from("smithy.api#String"), InnerType.String)
        val Int: Type = Type(ShapeId.from("smithy.api#Integer"), InnerType.Int)
        val Long: Type = Type(ShapeId.from("smithy.api#Long"), InnerType.Long)
        val Json: Type = Type(ShapeId.from("smithy.api#Document"), InnerType.Json)
    }
}

data class Field(
    val name: String,
    val type: Type,
    val required: Boolean,
    val hints: List<Hint>
)

data class PolymorphicDataKind(
    val kindStr: String,
    val shapeId: ShapeId,
    val name: String,
    val type: Type,
    val hints: List<Hint>
)

sealed interface Hint {
    data class Documentation(val string: String) : Hint
    data class Deprecated(val message: String) : Hint

    data class JsonRename(val name: String) : Hint
}

