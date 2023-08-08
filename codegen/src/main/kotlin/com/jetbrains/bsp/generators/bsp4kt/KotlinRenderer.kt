package com.jetbrains.bsp.generators.bsp4kt

import com.jetbrains.bsp.generators.CodegenFile
import com.jetbrains.bsp.generators.dsl.CodeBlock
import com.jetbrains.bsp.generators.dsl.Line
import com.jetbrains.bsp.generators.dsl.code
import com.jetbrains.bsp.generators.ir.Def
import com.jetbrains.bsp.generators.ir.EnumType
import com.jetbrains.bsp.generators.ir.EnumValue
import com.jetbrains.bsp.generators.ir.Field
import com.jetbrains.bsp.generators.ir.Hint
import com.jetbrains.bsp.generators.ir.JsonRpcMethodType
import com.jetbrains.bsp.generators.ir.Operation
import com.jetbrains.bsp.generators.ir.Type
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
        }
    }

    fun enumValueType(enumType: EnumType<*>): String {
        return when (enumType) {
            is EnumType.IntEnum -> "Int"
            EnumType.StringEnum -> "String"
        }
    }

    fun String.toUpperCamelCase(): String {
        return this.lowercase().split("_").joinToString("") { word ->
            word.replaceFirstChar {
                it.uppercase()
            }
        }
    }

    fun renderEnumValueDef(enumType: EnumType<*>): (EnumValue<*>) -> String {
        return when (enumType) {
            EnumType.IntEnum -> { ev: EnumValue<*> -> "${ev.name.toUpperCamelCase()}(${ev.value})" }
            EnumType.StringEnum -> { ev: EnumValue<*> -> """${ev.name.toUpperCamelCase()}("${ev.value}")""" }
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
            -"import com.jetbrains.jsonrpc4kt.IntEnum"
            -"import com.jetbrains.jsonrpc4kt.json.serializers.EnumAsIntSerializer"
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
            EnumType.IntEnum -> { ev: EnumValue<*> -> "${ev.name.toUpperCamelCase()} = ${ev.value}" }
            EnumType.StringEnum -> { ev: EnumValue<*> -> """${ev.name.toUpperCamelCase()} = "${ev.value}"""" }
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

    fun renderFieldRaw(field: Field): String {
        return "val ${field.name}: ${renderType(field.type)}${if (field.required) "" else "?"}"
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
        Type.Bool -> "Boolean"
        Type.Int -> "Int"
        Type.Json -> "JsonElement"
        is Type.List -> "List<${renderType(type.member)}>"
        Type.Long -> "Long"
        is Type.Map -> "Map<${renderType(type.key)}, ${renderType(type.value)}>"
        is Type.Ref -> type.shapeId.name
        is Type.Set -> "Set<${renderType(type.member)}>"
        Type.String -> "String"
        Type.Unit -> "Unit"
    }

    fun renderOperation(op: Operation): CodeBlock {
        val output = when (op.jsonRpcMethodType) {
            JsonRpcMethodType.Notification -> "Unit"
            JsonRpcMethodType.Request -> "CompletableFuture<${renderType(op.outputType)}>"
        }
        val input = when (op.inputType) {
            Type.Unit -> ""
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
            -"fun $name($input): $output"
            newline()
        }
    }

    fun renderService(def: Def.Service): CodegenFile {
        val name = def.name
        val code = code {
            -"package $basepkg"
            newline()
            -"import com.jetbrains.jsonrpc4kt.services.JsonNotification"
            -"import com.jetbrains.jsonrpc4kt.services.JsonRequest"
            -"import kotlinx.serialization.SerialName"
            -"import java.util.concurrent.CompletableFuture"
            newline()
            block("interface $name") {
                def.operations.forEach { operation ->
                    include(renderOperation(operation))
                }
            }
            newline()
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
