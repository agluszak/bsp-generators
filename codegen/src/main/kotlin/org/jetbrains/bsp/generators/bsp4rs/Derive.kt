package org.jetbrains.bsp.generators.bsp4rs

import org.jetbrains.bsp.generators.ir.*
import software.amazon.smithy.model.shapes.ShapeId

class DeriveRenderer(val defs: Map<ShapeId, Def>) {
    private var derivesSet: Set<DeriveOption> = emptySet()

    private fun defToSerializeList(def: Def) = when (def) {
        is Def.Service -> emptySet()
        is Def.ClosedEnum<*> -> when (def.enumType) {
            is EnumType.IntEnum -> setOf(DeriveOption.ENUM_INT_SERIALIZE)
            is EnumType.StringEnum -> setOf(DeriveOption.ENUM_STR_SERIALIZE)
        }

        else -> setOf(DeriveOption.STANDARD_SERIALIZE)
    }

    private fun typeToBlackList(type: Type): Set<DeriveOption> {
        return when (val innerType = type.type) {
            is InnerType.Json -> setOf(DeriveOption.HASH, DeriveOption.ORD)
            is InnerType.Ref -> defToBlackList(defs[type.shapeId]!!)
            is InnerType.List -> typeToBlackList(innerType.member)
            is InnerType.Set -> typeToBlackList(innerType.member)
            is InnerType.Map -> typeToBlackList(innerType.key).plus(typeToBlackList(innerType.value))
            else -> emptySet()
        }
    }

    private fun fieldToBlackList(field: Field): Set<DeriveOption> {
        return typeToBlackList(field.type).minus(
            if (field.required) emptySet()
            else setOf(DeriveOption.DEFAULT)
        )
    }

    private fun defToBlackList(def: Def): Set<DeriveOption> {
        return when (def) {
            is Def.Structure -> def.fields.flatMap { fieldToBlackList(it) }.toSet()
            is Def.OpenEnum<*> -> emptySet()
            is Def.ClosedEnum<*> -> if (def.values.isEmpty()) setOf(DeriveOption.DEFAULT) else emptySet()
            is Def.Service -> emptySet()
            is Def.Alias -> emptySet()
            is Def.DataKinds -> setOf(DeriveOption.DEFAULT, DeriveOption.HASH, DeriveOption.ORD)
        }
    }

    fun renderForDef(def: Def): String {
        derivesSet = setOf(
            DeriveOption.CLONE,
            DeriveOption.DEBUG,
            DeriveOption.DEFAULT,
            DeriveOption.EQ,
            DeriveOption.HASH,
            DeriveOption.ORD,
        )
        derivesSet = derivesSet.plus(defToSerializeList(def))
        derivesSet = derivesSet.minus(defToBlackList(def).toSet())

        return this.print()
    }

    fun renderForOp(): String {
        derivesSet = setOf(DeriveOption.DEBUG)

        return this.print()
    }

    private fun print(): String {
        return "#[derive(${derivesSet.joinToString(", ") { it.print }})]"
    }

    enum class DeriveOption(val print: String) {
        CLONE("Clone"),
        DEBUG("Debug"),
        DEFAULT("Default"),
        EQ("Eq, PartialEq"),
        HASH("Hash"),
        ORD("Ord, PartialOrd"),
        STANDARD_SERIALIZE("Serialize, Deserialize"),
        ENUM_INT_SERIALIZE("Serialize_repr, Deserialize_repr"),
        ENUM_STR_SERIALIZE("Deserialize_enum_str, Serialize_enum_str"), ;
    }
}