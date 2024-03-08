package org.jetbrains.bsp.generators.bsp4ts

import org.jetbrains.bsp.generators.CodegenFile
import org.jetbrains.bsp.generators.Loader
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
import org.jetbrains.bsp.generators.utils.camelCaseUpperCamelCase
import org.jetbrains.bsp.generators.utils.kebabToScreamingSnakeCase
import org.jetbrains.bsp.generators.utils.snakeToUpperCamelCase
import kotlin.io.path.Path

class TypeScriptRenderer(val basepkg: String, val definitions: List<Def>, val version: String) {
    val baseRelPath = Path(basepkg.replace(".", "/"))

    fun render(): List<CodegenFile> {
        val renderedDefs = definitions.mapNotNull { renderDef(it) }
        var allCodeBlocks = listOf(renderImports(), renderVersion()) + renderedDefs
        val combinedCode = allCodeBlocks.joinToString("\n") { it.toString() }
        val combinedFile = CodegenFile(baseRelPath.resolve("src/bsp.ts"), combinedCode)
        return listOf(combinedFile, renderPackageJson(), renderTSConfig())
    }

    fun renderPackageJson(): CodegenFile {
        val contents = """
        {
            "name": "bsp4ts",
            "version": "$version",
            "description": "Types for Build Server Protocol",
            "main": "./lib/bsp.js",
            "typings": "./lib/bsp.d.ts",
            "scripts": {
              "build": "tsc"
            },
            "files": ["lib/**/*"],
            "license": "Apache-2.0",
            "devDependencies": {
              "typescript": "^5.3.3"
            },
            "dependencies": {
              "vscode-jsonrpc": "^8.2.0"
            }
          }
        """

        return CodegenFile(baseRelPath.resolve("package.json"), contents.trimIndent())
    }

    fun renderTSConfig(): CodegenFile {
        val contents = """
        {
            "compilerOptions": {
              "outDir": "./lib",
              "module": "CommonJS",
              "declaration": true,
              "declarationMap": true,
              "esModuleInterop": true,
              "target": "ES6",
              "strict": true
            }
        }
        """

        return CodegenFile(baseRelPath.resolve("tsconfig.json"), contents.toString())
    }

    fun renderImports(): CodeBlock {
        val code = code {
            -"import { RequestType, RequestType0, RequestHandler, NotificationType, NotificationHandler } from 'vscode-jsonrpc'"
            -"import { MessageConnection } from 'vscode-jsonrpc/node'"
        }

        return code
    }

    fun renderVersion(): CodeBlock {
        val code = code {
            block("export namespace Bsp4Ts") {
                -"""export const ProtocolVersion: string = "$version""""
            }
        }

        return code
    }

    fun renderDef(def: Def): CodeBlock? {
        return when (def) {
            is Def.Alias -> null
            is Def.ClosedEnum<*> -> renderClosedEnum(def)
            is Def.OpenEnum<*> -> renderOpenEnum(def)
            is Def.Service -> renderService(def)
            is Def.Structure -> renderStructure(def)
            is Def.DataKinds -> renderData(def)
            is Def.UntaggedUnion -> renderUntaggedUnion(def)
        }
    }

    fun enumValueType(enumType: EnumType<*>): String {
        return when (enumType) {
            is EnumType.IntEnum -> "number"
            EnumType.StringEnum -> "string"
        }
    }

    fun renderEnumValueDef(enumType: EnumType<*>): (EnumValue<*>) -> String {
        return when (enumType) {
            EnumType.IntEnum -> { ev: EnumValue<*> -> "${ev.name.snakeToUpperCamelCase()} = ${ev.value}" }
            EnumType.StringEnum -> { ev: EnumValue<*> -> """${ev.name.snakeToUpperCamelCase()} = "${ev.value}"""" }
        }
    }

    fun renderClosedEnum(def: Def.ClosedEnum<*>): CodeBlock {
        val name = def.name
        val values = def.values.map { value ->
            renderEnumValueDef(def.enumType)(value)
        }
        val code = code {
            block("export enum $name") {
                lines(values, join = ",")
            }
        }

        return code

    }

    fun renderStaticValue(enumType: EnumType<*>): (EnumValue<*>) -> String {
        return when (enumType) {
            EnumType.IntEnum -> { ev: EnumValue<*> -> "${ev.name.snakeToUpperCamelCase()} = ${ev.value}" }
            EnumType.StringEnum -> { ev: EnumValue<*> -> """${ev.name.snakeToUpperCamelCase()} = "${ev.value}"""" }
        }
    }

    fun renderOpenEnum(def: Def.OpenEnum<*>): CodeBlock {
        val name = def.name
        val code = code {
            block("export namespace $name") {
                if (def.values.isNotEmpty()) {
                    def.values.forEach { value ->
                        -"export const ${renderStaticValue(def.enumType)(value)}"
                    }
                } else {
                    removeNewline()
                }
            }
            newline()
            -"export type $name = ${enumValueType(def.enumType)}"
        }

        return code
    }

    private fun renderData(def: Def.DataKinds): CodeBlock {
        val values = def.kinds.map { polyData ->
            val snakeCased = polyData.kind.kebabToScreamingSnakeCase()
            EnumValue(snakeCased, polyData.kind, polyData.hints)
        }
        val dataKindDef = Def.OpenEnum(def.kindsEnumId, EnumType.StringEnum, values, def.hints)

        return renderOpenEnum(dataKindDef)
    }

    private fun renderUntaggedUnion(def: Def.UntaggedUnion): CodeBlock {
        require(def.members.size == 2 && def.members.containsAll(listOf(Type.String, Type.Int))) {
            "Only unions with String and Int are supported"
        }

        fun makeTypeName(renderedType: String): String = "${renderedType.camelCaseUpperCamelCase()}"
        val name = def.name
        val code = code {
            - """export type ${makeTypeName(name)} = string | number"""
        }

        return code
    }

    fun renderFieldRaw(field: Field): String {
        return "${field.name}${if (field.required) "" else "?"}: ${renderType(field.type)}"
    }

    fun renderType(type: Type): String = when (type) {
        Type.Bool -> "boolean"
        Type.Int -> "number"
        Type.Json -> "any"
        is Type.List -> "${renderType(type.member)}[]"
        Type.Long -> "number"
        is Type.Map -> "{ [key: ${renderType(type.key)}]: ${renderType(type.value)} }"
        is Type.Ref -> type.shapeId.name
        is Type.Set -> "Set<${renderType(type.member)}>"
        Type.String -> "string"
        Type.Unit -> "void"
        is Type.UntaggedUnion -> type.members.joinToString(" | ") { renderType(it) }
    }

    fun renderOperation(op: Operation): CodeBlock {
        val output = when (op.jsonRpcMethodType) {
            JsonRpcMethodType.Notification -> "void"
            JsonRpcMethodType.Request -> renderType(op.outputType)
        }

        val input = when (op.inputType) {
            Type.Unit -> "void"
            else -> "${renderType(op.inputType)}"
        }
        val rpcMethod = op.jsonRpcMethod
        val methodType = """export const method: '$rpcMethod' = '$rpcMethod'"""
        val maybeDeprecated = op.hints.filterIsInstance<Hint.Deprecated>().firstOrNull()?.let {"""
            /**
             * @deprecated ${it.message}
             */""".trimIndent()
        }

        val typeLine = when (op.jsonRpcMethodType) {
            JsonRpcMethodType.Request -> when (op.inputType) {
                Type.Unit -> """export const type = new RequestType0<${output}, void>(method)"""
                else -> """export const type = new RequestType<${input}, ${output}, void>(method)"""
            }
            JsonRpcMethodType.Notification -> """export const type = new NotificationType<${input}>(method)"""
        }

        val handlerLine = when (op.jsonRpcMethodType) {
            JsonRpcMethodType.Request -> """export type HandlerSignature = RequestHandler<${input}, ${output}, void>"""
            JsonRpcMethodType.Notification -> """export type HandlerSignature = NotificationHandler<${input}>"""
        }

        val name = op.name
        return code {
            -maybeDeprecated
            block("export namespace $name") {
                -methodType
                -typeLine
                -handlerLine
            }
            newline()
        }
    }

    fun renderService(def: Def.Service): CodeBlock {
        val name = def.name
        val code = code {
            // Render method, type, and HandlerSignature for each operation.
            def.operations.forEach { operation ->
                include(renderOperation(operation))
            }

            // Interface with all required handlers.
            block("export interface $name") {
                code {
                    def.operations.forEach { operation ->
                        line("""${operation.name.replaceFirstChar { it.lowercase() }}: ${operation.name}.HandlerSignature""")
                    }
                }
            }
            newline()

            // Registration function to assign handler implementations to a connection.
            val funcName = "register${name}Handlers"
            block("export function $funcName(connection: MessageConnection, handlers: $name)") {
                def.operations.forEach { operation ->
                    when (operation.jsonRpcMethodType) {
                        JsonRpcMethodType.Request -> line("connection.onRequest(${operation.name}.type, handlers.${operation.name.replaceFirstChar { it.lowercase() }})")
                        JsonRpcMethodType.Notification -> line("connection.onNotification(${operation.name}.type, handlers.${operation.name.replaceFirstChar { it.lowercase() }})")
                    }
                }
            }
        }

        return code
    }

    fun renderStructure(def: Def.Structure): CodeBlock {
        val code = code {
            block("export interface ${def.name}") {
                lines(def.fields.map { renderFieldRaw(it).toString() }, join = "")
            }
        }

        return code
    }
}
