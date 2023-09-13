package org.jetbrains.bsp.generators.bsp4rs

import org.jetbrains.bsp.generators.ir.Def
import org.jetbrains.bsp.generators.ir.EnumType
import org.jetbrains.bsp.generators.ir.Field
import org.jetbrains.bsp.generators.ir.IrShape
import org.jetbrains.bsp.generators.ir.Type
import software.amazon.smithy.model.shapes.ShapeId

class DeriveRenderer(private val defs: Map<ShapeId, Def>) {
    private var derivesSet: Set<DeriveOption> = emptySet()

    fun renderForOp(): String {
        derivesSet = setOf(DeriveOption.DEBUG)

        return this.print()
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

    private fun defToSerializeList(def: Def) = when (def) {
        is Def.Service -> emptySet()
        is Def.ClosedEnum<*> -> when (def.enumType) {
            is EnumType.IntEnum -> setOf(DeriveOption.ENUM_INT_SERIALIZE)
            is EnumType.StringEnum -> setOf(DeriveOption.ENUM_STR_SERIALIZE)
        }

        else -> setOf(DeriveOption.STANDARD_SERIALIZE)
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

    private fun fieldToBlackList(field: Field): Set<DeriveOption> {
        return typeToBlackList(field.type).minus(
            if (field.required) emptySet()
            else setOf(DeriveOption.DEFAULT)
        )
    }

    private fun typeToBlackList(shape: IrShape): Set<DeriveOption> {
        return when (val type = shape.type) {
            is Type.Json -> setOf(DeriveOption.HASH, DeriveOption.ORD)
            is Type.Ref -> defToBlackList(defs[shape.shapeId]!!)
            is Type.List -> typeToBlackList(type.member)
            is Type.Set -> typeToBlackList(type.member)
            is Type.Map -> typeToBlackList(type.key).plus(typeToBlackList(type.value))
            else -> emptySet()
        }
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
