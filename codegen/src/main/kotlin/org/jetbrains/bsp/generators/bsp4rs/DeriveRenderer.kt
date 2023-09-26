package org.jetbrains.bsp.generators.bsp4rs

import org.jetbrains.bsp.generators.dsl.CodeBlock
import org.jetbrains.bsp.generators.dsl.rustCode
import org.jetbrains.bsp.generators.ir.Def
import org.jetbrains.bsp.generators.ir.EnumType
import org.jetbrains.bsp.generators.ir.Field
import org.jetbrains.bsp.generators.ir.Type
import software.amazon.smithy.model.shapes.ShapeId

class DeriveRenderer(private val defs: Map<ShapeId, Def>) {
    private var derivesSet: Set<DeriveOption> = emptySet()

    fun renderForOperation(): CodeBlock {
        derivesSet = setOf(DeriveOption.DEBUG)

        return this.render()
    }

    fun renderForDef(def: Def): CodeBlock {
        derivesSet = setOf(
            DeriveOption.CLONE,
            DeriveOption.DEBUG,
            DeriveOption.DEFAULT,
            DeriveOption.EQ,
            DeriveOption.HASH,
            DeriveOption.ORD,
        )
        derivesSet = derivesSet.plus(defToSerializeList(def))
        derivesSet = derivesSet.minus(defToDenyList(def).toSet())

        return this.render()
    }

    private fun defToSerializeList(def: Def): Set<DeriveOption> = when (def) {
        is Def.Service -> emptySet()
        is Def.ClosedEnum<*> -> when (def.enumType) {
            is EnumType.IntEnum -> setOf(DeriveOption.ENUM_INT_SERIALIZE)
            is EnumType.StringEnum -> setOf(DeriveOption.ENUM_STR_SERIALIZE)
        }

        else -> setOf(DeriveOption.STANDARD_SERIALIZE)
    }

    private fun defToDenyList(def: Def): Set<DeriveOption> {
        return when (def) {
            is Def.Structure -> def.fields.flatMap { fieldToDenyList(it) }.toSet()
            is Def.OpenEnum<*> -> emptySet()
            is Def.ClosedEnum<*> -> if (def.values.isEmpty()) setOf(DeriveOption.DEFAULT) else emptySet()
            is Def.Service -> emptySet()
            is Def.Alias -> emptySet()
            is Def.DataKinds -> setOf(DeriveOption.DEFAULT, DeriveOption.HASH, DeriveOption.ORD)
        }
    }

    private fun fieldToDenyList(field: Field): Set<DeriveOption> {
        return typeToDenyList(field.type).minus(
            if (field.required) emptySet()
            else setOf(DeriveOption.DEFAULT)
        )
    }

    private fun typeToDenyList(type: Type): Set<DeriveOption> {
        return when (type) {
            is Type.TJson -> setOf(DeriveOption.HASH, DeriveOption.ORD)
            is Type.TRef -> defToDenyList(defs[type.shapeId]!!)
            is Type.TList -> typeToDenyList(type.member)
            is Type.TSet -> typeToDenyList(type.member)
            is Type.TMap -> typeToDenyList(type.key).plus(typeToDenyList(type.value))
            else -> emptySet()
        }
    }

    private fun render(): CodeBlock = rustCode {
        -"#[derive(${derivesSet.joinToString(", ") { it.print }})]"
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
