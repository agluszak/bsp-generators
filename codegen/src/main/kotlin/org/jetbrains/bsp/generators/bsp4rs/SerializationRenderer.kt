package org.jetbrains.bsp.generators.bsp4rs

import org.jetbrains.bsp.generators.dsl.CodeBlock
import org.jetbrains.bsp.generators.dsl.rustCode
import org.jetbrains.bsp.generators.ir.Def
import org.jetbrains.bsp.generators.ir.EnumType
import org.jetbrains.bsp.generators.ir.Field
import org.jetbrains.bsp.generators.ir.Hint
import org.jetbrains.bsp.generators.ir.Type

class SerializationRenderer {
    private var serdeSet: Set<SerdeOption> = emptySet()
    private var reprSet: Set<ReprOption> = emptySet()

    fun renderForDef(def: Def, untagged: Boolean = false): CodeBlock {
        prepareSerdeSet(def.hints)
        serdeSet = serdeSet.plus(defToSerdeList(def, untagged))

        reprSet = setOf()
        reprSet = reprSet.plus(defToReprList(def))

        return rustCode {
            -printSerde()
            -printRepr()
        }
    }

    fun renderForField(field: Field): CodeBlock {
        prepareSerdeSet(field.hints)
        serdeSet = serdeSet.plus(fieldToSerdeList(field))

        return rustCode {
            -printSerde()
        }
    }

    private fun prepareSerdeSet(hints: List<Hint>) {
        val rename = hints.find { it is Hint.JsonRename }

        serdeSet = if (rename is Hint.JsonRename) setOf(SerdeOption.Rename(""""${rename.name}"""")) else emptySet()
    }

    private fun defToSerdeList(def: Def, untagged: Boolean): Set<SerdeOption> = when (def) {
        is Def.Structure -> setOf(SerdeOption.RenameAllCamelCase)
        is Def.OpenEnum<*> -> setOf(SerdeOption.Transparent)
        is Def.ClosedEnum<*> -> when (def.enumType) {
            is EnumType.StringEnum -> setOf(SerdeOption.RenameAllKebabCase)
            else -> emptySet()
        }

        is Def.Alias -> setOf(SerdeOption.Transparent)
        is Def.DataKinds ->
            if (untagged) setOf(SerdeOption.Untagged)
            else setOf(SerdeOption.RenameAllKebabCase, SerdeOption.TagDataKind, SerdeOption.ContentData)

        else -> emptySet()
    }

    private fun defToReprList(def: Def): Set<ReprOption> = when (def) {
        is Def.ClosedEnum<*> -> when (def.enumType) {
            is EnumType.IntEnum -> setOf(ReprOption.U8)
            else -> emptySet()
        }

        else -> emptySet()
    }

    private fun fieldToSerdeList(field: Field): Set<SerdeOption> {
        var serdeOpt = emptySet<SerdeOption>()

        if (field.type.type is Type.Json && field.name == "data"
            && field.type.shapeId.namespace.startsWith("bsp")
        ) {
            serdeOpt = serdeOpt.plus(SerdeOption.Flatten)
        }

        if (!field.required) {
            // TODO: remove default once every not required is Option
            serdeOpt = serdeOpt.plus(SerdeOption.Default)
            serdeOpt = serdeOpt.plus(SerdeOption.SkipOption)
        }

        return serdeOpt
    }

    private fun printSerde(): String? =
        if (serdeSet.isEmpty()) null
        else "#[serde(${serdeSet.joinToString(", ") { it.print() }})]"

    private fun printRepr(): String? =
        if (reprSet.isEmpty()) null
        else "#[repr(${reprSet.joinToString(", ") { it.print }})]"

    enum class ReprOption(val print: String) {
        U8("u8"),
    }

    sealed class SerdeOption(val key: String, val value: String) {
        object Default : SerdeOption("default", "")
        object Transparent : SerdeOption("transparent", "")
        object Untagged : SerdeOption("untagged", "")
        object Flatten : SerdeOption("flatten", "")
        object SkipOption : SerdeOption("skip_serializing_if", """"Option::is_none"""")
        class Rename(name: String) : SerdeOption("rename", name)
        class RenameAll(case: String) : SerdeOption("rename_all", case)
        class Tag(tag: String) : SerdeOption("tag", tag)
        class Content(content: String) : SerdeOption("content", content)

        companion object {
            val RenameAllCamelCase = RenameAll(""""camelCase"""")
            val RenameAllKebabCase = RenameAll(""""kebab-case"""")
            val TagDataKind = Tag(""""dataKind"""")
            val ContentData = Content(""""data"""")
        }

        fun print(): String {
            return if (this.value.isEmpty()) key else "$key = $value"
        }
    }
}
