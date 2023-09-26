package org.jetbrains.bsp.generators.bsp4kt

import org.jetbrains.bsp.generators.CodegenFile
import org.jetbrains.bsp.generators.dsl.CodeBlock
import org.jetbrains.bsp.generators.dsl.code
import org.jetbrains.bsp.generators.ir.Def
import org.jetbrains.bsp.generators.ir.EnumType
import org.jetbrains.bsp.generators.ir.EnumValue
import org.jetbrains.bsp.generators.ir.Field
import org.jetbrains.bsp.generators.ir.Hint
import org.jetbrains.bsp.generators.ir.JsonRpcMethodType
import org.jetbrains.bsp.generators.ir.Operation
import org.jetbrains.bsp.generators.ir.Type
import org.jetbrains.bsp.generators.utils.kebabToScreamingSnakeCase
import org.jetbrains.bsp.generators.utils.snakeToUpperCamelCase
import kotlin.io.path.Path

class KotlinRenderer(val basepkg: String, val definitions: List<Def>, val version: String) {
    val baseRelPath = Path(basepkg.replace(".", "/"))

    fun render(): List<CodegenFile> {
        val files = definitions.mapNotNull { renderDef(it) }
        val versionFile = renderVersion()
        return listOf(versionFile) + files
    }

    fun renderVersion(): CodegenFile {
        val code = code {
            -"package $basepkg"
            newline()
            block("object Bsp4Kt") {
                -"""const val ProtocolVersion: String = "$version""""
            }
        }

        return CodegenFile(baseRelPath.resolve("Bsp4Kt.kt"), code.toString())
    }

    fun renderDef(def: Def): CodegenFile? {
        return when (def) {
            is Def.Alias -> null
            is Def.ClosedEnum<*> -> renderClosedEnum(def)
            is Def.OpenEnum<*> -> renderOpenEnum(def)
            is Def.Service -> renderService(def)
            is Def.Structure -> renderStructure(def)
            is Def.DataKinds -> renderData(def)
            is Def.UntaggedUnion -> TODO()
        }
    }

    fun enumValueType(enumType: EnumType<*>): String {
        return when (enumType) {
            is EnumType.IntEnum -> "Int"
            EnumType.StringEnum -> "String"
        }
    }

    fun renderEnumValueDef(enumType: EnumType<*>): (EnumValue<*>) -> String {
        return when (enumType) {
            EnumType.IntEnum -> { ev: EnumValue<*> -> "${ev.name.snakeToUpperCamelCase()}(${ev.value})" }
            EnumType.StringEnum -> { ev: EnumValue<*> -> """${ev.name.snakeToUpperCamelCase()}("${ev.value}")""" }
        }
    }

    fun renderClosedEnum(def: Def.ClosedEnum<*>): CodegenFile {
        val name = def.name
        val values = def.values.map { value ->
            renderEnumValueDef(def.enumType)(value)
        }
        val code = code {
            -"package $basepkg"
            newline()
            -"import org.jetbrains.jsonrpc4kt.IntEnum"
            -"import org.jetbrains.jsonrpc4kt.json.serializers.EnumAsIntSerializer"
            -"import kotlinx.serialization.KSerializer"
            -"import kotlinx.serialization.Serializable"
            newline()
            -"@Serializable(with = $name.Companion::class)"
            block("enum class $name(override val value: ${enumValueType(def.enumType)}) : IntEnum") {
                lines(values, join = ",", end = ";")
                newline()
                -"companion object : KSerializer<$name> by EnumAsIntSerializer($name::class)"
            }
        }

        return CodegenFile(baseRelPath.resolve("$name.kt"), code.toString())
    }

    fun renderStaticValue(enumType: EnumType<*>): (EnumValue<*>) -> String {
        return when (enumType) {
            EnumType.IntEnum -> { ev: EnumValue<*> -> "${ev.name.snakeToUpperCamelCase()} = ${ev.value}" }
            EnumType.StringEnum -> { ev: EnumValue<*> -> """${ev.name.snakeToUpperCamelCase()} = "${ev.value}"""" }
        }
    }

    fun renderOpenEnum(def: Def.OpenEnum<*>): CodegenFile {
        val name = def.name
        val code = code {
            -"package $basepkg"
            newline()
            block("object $name") {
                def.values.forEach { value ->
                    -"const val ${renderStaticValue(def.enumType)(value)}"
                }
            }
        }

        return CodegenFile(baseRelPath.resolve("$name.kt"), code.toString())
    }

    private fun renderData(def: Def.DataKinds): CodegenFile {
        val values = def.kinds.map { polyData ->
            val snakeCased = polyData.kind.kebabToScreamingSnakeCase()
            EnumValue(snakeCased, polyData.kind, polyData.hints)
        }
        val dataKindDef = Def.OpenEnum(def.kindsEnumId, EnumType.StringEnum, values, def.hints)

        return renderOpenEnum(dataKindDef)
    }

    fun renderFieldRaw(field: Field): String {
        return "val ${field.name}: ${renderType(field.type)}${if (field.required) "" else "? = null"}"
    }

    fun renderKotlinField(field: Field): CodeBlock {
        val rename = field.hints.filterIsInstance<Hint.JsonRename>().firstOrNull()?.let {
            """@SerialName("${it.name}")"""
        }
        return code {
            -rename
            -renderFieldRaw(field)
        }
    }

    fun renderType(type: Type): String = when (type) {
        Type.TBool -> "Boolean"
        Type.TInt -> "Int"
        Type.TJson -> "JsonElement"
        is Type.TList -> "List<${renderType(type.member)}>"
        Type.TLong -> "Long"
        is Type.TMap -> "Map<${renderType(type.key)}, ${renderType(type.value)}>"
        is Type.TRef -> type.shapeId.name
        is Type.TSet -> "Set<${renderType(type.member)}>"
        Type.TString -> "String"
        Type.TUnit -> "Unit"
        is Type.TUntaggedUnion -> TODO()
    }

    fun renderOperation(op: Operation): CodeBlock {
        val output = when (op.jsonRpcMethodType) {
            JsonRpcMethodType.Notification -> "Unit"
            JsonRpcMethodType.Request -> renderType(op.outputType)
        }
        val maybeSuspend = when (op.jsonRpcMethodType) {
            JsonRpcMethodType.Notification -> ""
            JsonRpcMethodType.Request -> "suspend "
        }
        val input = when (op.inputType) {
            Type.TUnit -> ""
            else -> "params: ${renderType(op.inputType)}"
        }
        val rpcMethod = op.jsonRpcMethod
        val rpcAnnotation = when (op.jsonRpcMethodType) {
            JsonRpcMethodType.Notification -> """@JsonNotification("$rpcMethod")"""
            JsonRpcMethodType.Request -> """@JsonRequest("$rpcMethod")"""
        }
        val maybeDeprecated = op.hints.filterIsInstance<Hint.Deprecated>().firstOrNull()?.let {
            """@Deprecated("${it.message}")"""
        }
        val name = op.name.replaceFirstChar { it.lowercase() }
        return code {
            -maybeDeprecated
            -rpcAnnotation
            -"${maybeSuspend}fun $name($input): $output"
            newline()
        }
    }

    fun renderService(def: Def.Service): CodegenFile {
        val name = def.name
        val code = code {
            -"package $basepkg"
            newline()
            -"import org.jetbrains.jsonrpc4kt.services.JsonNotification"
            -"import org.jetbrains.jsonrpc4kt.services.JsonRequest"
            -"import kotlinx.serialization.SerialName"
            newline()
            block("interface $name") {
                def.operations.forEach { operation ->
                    include(renderOperation(operation))
                }
            }
        }

        val fileName = "$name.kt"
        return CodegenFile(baseRelPath.resolve(fileName), code.toString())
    }

    fun renderStructure(def: Def.Structure): CodegenFile {
        val code = code {
            -"package $basepkg"
            newline()
            -"import kotlinx.serialization.Serializable"
            -"import kotlinx.serialization.SerialName"
            -"import kotlinx.serialization.json.JsonElement"
            newline()
            -"@Serializable"
            if (def.fields.isEmpty()) {
                -"object ${def.name}"
            } else {
                paren("data class ${def.name}") {
                    lines(def.fields.map { renderKotlinField(it).toString() }, join = ",")
                }
            }
        }

        val fileName = "${def.name}.kt"
        return CodegenFile(baseRelPath.resolve(fileName), code.toString())
    }
}
