package org.jetbrains.bsp.generators.bsp4rs

import org.jetbrains.bsp.generators.CodegenFile
import org.jetbrains.bsp.generators.dsl.CodeBlock
import org.jetbrains.bsp.generators.dsl.rustCode
import org.jetbrains.bsp.generators.ir.*
import org.jetbrains.bsp.generators.utils.camelToSnakeCase
import org.jetbrains.bsp.generators.utils.kebabToUpperCamelCase
import org.jetbrains.bsp.generators.utils.snakeToUpperCamelCase
import software.amazon.smithy.model.shapes.ShapeId
import java.nio.file.Path
import kotlin.io.path.Path

class RustRenderer(basepkg: String, private val modules: List<Module>, val version: String) {
    private val baseRelPath = Path(basepkg.replace(".", "/"))
    private val renames: Map<String, String> = mapOf(Pair("type", "r#type"), Pair("r#version", "version"))
    private val bannedAliases: List<String> = listOf("Integer", "Long")
    private val deriveRenderer = DeriveRenderer(modules.flatMap { it.definitions }.associateBy { it.shapeId })
    private val serializationRenderer = SerializationRenderer()

    private fun makeName(name: String): String {
        return renames[name] ?: name
    }

    fun render(): List<CodegenFile> {
        val defFiles = modules.flatMap { renderModule(it) }
        val libFile = generateLibFile(modules.map { it.moduleName })
        return listOf(libFile) + defFiles
    }

    private fun renderModule(module: Module): List<CodegenFile> {
        val files = module.definitions.mapNotNull {
            renderDef(it)?.run {
                val name = makeName(it.name).camelToSnakeCase()
                Pair(generateFile(this, Path(module.moduleName), "$name.rs"), name)
            }
        }
        val modFile = generateModFile(module.moduleName, files.map { it.second })
        return files.unzip().first.toMutableList() + modFile
    }

    private fun renderData(def: Def.DataKinds): CodeBlock {
        val dataKinds = def.kinds.map { kind ->
            val name = makeName(kind.kind.kebabToUpperCamelCase())
            val dataType = renderType(kind.shape, true)
            Pair(name, dataType)
        }

        val namedName = "Named${def.name}"

        return rustCode {
            -deriveRenderer.renderForDef(def)
            lines(serializationRenderer.renderForDef(def))
            block("pub enum $namedName") {
                lines(dataKinds.map { "${it.first}(${it.second})" }, ",", ",")
            }
            newline()
            lines(renderHints(def.hints))
            -"#[allow(clippy::large_enum_variant)]"
            -deriveRenderer.renderForDef(def)
            lines(serializationRenderer.renderForDef(def.copy(shapeId = ShapeId.fromParts("wrap", def.name))))
            block("pub enum ${def.name}") {
                -"Named($namedName),"
                -"Other(OtherData),"
            }
            newline()
            block("impl ${def.name}") {
                dataKinds.forEach { dataKind ->
                    block("pub fn ${dataKind.first.camelToSnakeCase()}(data: ${dataKind.second}) -> Self") {
                        -"${def.name}::Named($namedName::${dataKind.first}(data))"
                    }
                }
            }
        }
    }

    private fun isAliasRenderable(name: String, type: Type): Boolean {
        if (name in bannedAliases) return false
        if (type is Type.List) return false
        if (type is Type.Set) return false

        return true
    }

    private fun renderAlias(def: Def.Alias): CodeBlock? {
        if (!isAliasRenderable(def.name, def.aliasedType)) return null
        val name = def.name
        val type = renderBuiltInType(def.aliasedType)

        val derefBlock = rustCode {
            block("""impl std::ops::Deref for $name""") {
                -"""type Target = $type;"""
                newline()
                block("fn deref(&self) -> &Self::Target") {
                    -"&self.0"
                }
            }
        }

        fun from(fromType: String, mod: String): CodeBlock {
            return rustCode {
                block("""impl From<$fromType> for $name""") {
                    block("fn from(input: $fromType) -> Self") {
                        -"$name(input$mod)"
                    }
                }
            }
        }

        return rustCode {
            lines(renderHints(def.hints))
            -deriveRenderer.renderForDef(def)
            lines(serializationRenderer.renderForDef(def))
            -"""pub struct $name(pub $type);"""
            newline()
            include(derefBlock)
            newline()
            include(from(type, ""))
            newline()
            if (def.aliasedType is Type.String) include(from("&str", ".to_string()"))
        }
    }

    private fun createPath(namespacePath: Path, fileName: String): Path {
        return baseRelPath.resolve(namespacePath).resolve(fileName)
    }

    private fun generateLibFile(modulesNames: List<String>): CodegenFile {
        val code = rustCode {
            lines(modulesNames.map { "pub mod $it" }, ";", ";")
            newline()
            lines(modulesNames.map { "use $it::*" }, ";", ";")
            newline()
            include(renderRpcTraits())
            newline()
            include(renderOtherDataStruct())
        }

        return generateFile(code, Path(""), "lib.rs")
    }

    private fun generateModFile(moduleName: String, filesNames: List<String>): CodegenFile {
        val code = rustCode {
            lines(filesNames.map { "mod $it" }, ";", ";")
            newline()
            lines(filesNames.map { "pub use $it::*" }, ";", ";")
        }

        return generateFile(code, Path(moduleName), "mod.rs")
    }

    private fun renderRpcTraits(): CodeBlock {
        val paramsStr = "type Params: DeserializeOwned + Serialize"
        val resultStr = "type Result: DeserializeOwned + Serialize"
        val methodStr = "const METHOD: &'static str"

        return rustCode {
            block("pub trait Request") {
                lines(listOf(paramsStr, resultStr, methodStr), ";", ";")
            }
            newline()
            block("pub trait Notification") {
                lines(listOf(paramsStr, methodStr), ";", ";")
            }
        }
    }

    private fun renderOtherDataStruct(): CodeBlock {
        val def = Def.Structure(
            ShapeId.fromParts("bsp", "OtherData"),
            listOf(
                Field("dataKind", IrShape.String, true, listOf()),
                Field("data", IrShape.Json, true, listOf())
            ),
            listOf()
        )

        return renderStructure(def)
    }

    private fun renderDef(def: Def): CodeBlock? {
        return when (def) {
            is Def.Structure -> renderStructure(def)
            is Def.OpenEnum<*> -> renderOpenEnum(def)
            is Def.ClosedEnum<*> -> renderClosedEnum(def)
            is Def.Service -> renderService(def)
            is Def.Alias -> renderAlias(def)
            is Def.DataKinds -> renderData(def)
        }
    }

    private fun generateFile(content: CodeBlock, namespacePath: Path, fileName: String): CodegenFile {
        val code = rustCode {
            include(renderImports(fileName != "lib.rs"))
            include(content)
        }
        return CodegenFile(createPath(namespacePath, fileName), code.toString())
    }

    private fun renderDocumentation(hints: List<Hint.Documentation>): List<String> {
        return hints.flatMap { el ->
            val docLines = el.string.split("\n").toMutableList()
            docLines[0] = docLines.first().let { "/** $it" }
            docLines[docLines.lastIndex] = docLines.last().let { "$it */" }
            docLines
        }
    }

    private fun renderDeprecated(hints: List<Hint.Deprecated>): List<String> {
        return hints.map { """#[deprecated(note = "${it.message}")]""" }
    }

    fun renderHints(hints: List<Hint>): List<String> {
        return renderDocumentation(hints.filterIsInstance<Hint.Documentation>()) +
                renderDeprecated(hints.filterIsInstance<Hint.Deprecated>())
    }

    private fun renderBuiltInType(type: Type): String = when (type) {
        Type.Bool -> "bool"
        Type.Int -> "i32"
        Type.Json -> "serde_json::Value"
        is Type.List -> "Vec<${renderTypeName(type.member)}>"
        Type.Long -> "i64"
        is Type.Map -> "BTreeMap<${renderTypeName(type.key)}, ${renderTypeName(type.value)}>"
        is Type.Set -> "BTreeSet<${renderTypeName(type.member)}>"
        Type.String -> "String"
        Type.Unit -> "()"
        else -> ""
    }

    private fun renderTypeName(irShape: IrShape): String =
        if (irShape.type is Type.Ref) makeName(irShape.shapeId.name)
        else if (irShape.shapeId.namespace.startsWith("bsp") && isAliasRenderable(irShape.shapeId.name, irShape.type))
            makeName(irShape.shapeId.name)
        else renderBuiltInType(irShape.type)

    fun renderType(shape: IrShape, isRequired: Boolean): String {
        if (isRequired) return renderTypeName(shape)

        return when (shape.type) {
            is Type.List, is Type.Map, is Type.Set -> renderTypeName(shape)
            else -> "Option<${renderTypeName(shape)}>"
        }
    }

    fun renderStructFieldRaw(field: Field): String {
        return "pub ${makeName(field.name).camelToSnakeCase()}: ${renderType(field.type, field.required)}"
    }

    private fun renderStructField(field: Field): CodeBlock {
        if (field.name == "dataKind" && field.type != IrShape.String) {
            return rustCode { }
        }

        return rustCode {
            lines(renderHints(field.hints))
            lines(serializationRenderer.renderForField(field))
            -"${renderStructFieldRaw(field)},"
        }
    }

    private fun renderImports(canImportCrate: Boolean): CodeBlock {
        return rustCode {
            -"use serde::{Deserialize, Serialize};"
            -"use serde::de::DeserializeOwned;"
            -"use serde_repr::{Deserialize_repr, Serialize_repr};"
            -"use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};"
            -"use std::collections::{BTreeSet, BTreeMap};"
            -(if (canImportCrate) "use crate::*;" else null)
            newline()
        }
    }

    private fun renderStructure(def: Def.Structure): CodeBlock {
        return rustCode {
            lines(renderHints(def.hints))
            -deriveRenderer.renderForDef(def)
            lines(serializationRenderer.renderForDef(def))
            block("pub struct ${def.name}") {
                def.fields.forEach { field ->
                    include(renderStructField(field))
                }
            }
            newline()
        }
    }

    private fun renderJsonRpcMethodType(type: JsonRpcMethodType): String {
        return when (type) {
            JsonRpcMethodType.Notification -> "Notification"
            JsonRpcMethodType.Request -> "Request"
        }
    }

    fun renderOperation(op: Operation): CodeBlock {
        val name = makeName(op.name)
        val output = when (op.jsonRpcMethodType) {
            JsonRpcMethodType.Notification -> null
            JsonRpcMethodType.Request -> "type Result = ${renderType(op.outputType, true)}"
        }
        val operationProperties = listOfNotNull(
            "type Params = ${renderType(op.inputType, true)}",
            output,
            """const METHOD: &'static str = "${op.jsonRpcMethod}""""
        )

        return rustCode {
            -deriveRenderer.renderForOp()
            -"pub enum $name {}"
            newline()
            lines(renderHints(op.hints))
            block("impl ${renderJsonRpcMethodType(op.jsonRpcMethodType)} for $name") {
                lines(operationProperties, join = ";", end = ";")
            }
            newline()
        }
    }

    private fun renderService(def: Def.Service): CodeBlock {
        return rustCode {
            def.operations.forEach { operation ->
                include(renderOperation(operation))
            }
        }
    }

    private fun renderClosedEnum(def: Def.ClosedEnum<*>): CodeBlock {
        val isDefault = def.values.isNotEmpty()

        val genName: (String) -> String = { makeName(it.snakeToUpperCamelCase()) }
        fun renderEnumValue(ev: EnumValue<*>): String = when (def.enumType) {
            EnumType.IntEnum -> "${genName(ev.name)} = ${ev.value}"
            EnumType.StringEnum -> genName(ev.name)
        }

        val values = def.values.map { value ->
            rustCode { lines(renderHints(value.hints) + renderEnumValue(value), end = ",") }
        }

        return rustCode {
            lines(renderHints(def.hints))
            -deriveRenderer.renderForDef(def)
            lines(serializationRenderer.renderForDef(def))
            block("pub enum ${def.name}") {
                if (isDefault) {
                    -"#[default]"
                }
                for (value in values) {
                    include(value)
                }
            }
        }
    }

    private fun renderOpenEnum(def: Def.OpenEnum<*>): CodeBlock {
        val name = def.name

        fun renderValue(ev: EnumValue<*>): String = when (def.enumType) {
            EnumType.IntEnum -> "${ev.value}"
            EnumType.StringEnum -> """"${ev.value}""""
        }

        fun renderEnumValue(ev: EnumValue<*>): String =
            "pub const ${makeName(ev.name.uppercase())}: $name = $name::new(${renderValue(ev)})"

        val values = def.values.map { value ->
            rustCode { lines(renderHints(value.hints) + renderEnumValue(value), end = ";") }
        }
        val type = when (def.enumType) {
            EnumType.IntEnum -> "i32"
            EnumType.StringEnum -> "std::borrow::Cow<'static, str>"
        }
        val newFun = when (def.enumType) {
            EnumType.IntEnum -> "pub const fn new(tag: i32) -> Self { $name(tag) }"
            EnumType.StringEnum -> "pub const fn new(tag: &'static str) -> Self { $name(std::borrow::Cow::Borrowed(tag)) }"
        }

        return rustCode {
            lines(renderHints(def.hints))
            -deriveRenderer.renderForDef(def)
            lines(serializationRenderer.renderForDef(def))
            -"pub struct $name(pub $type);"
            block("impl $name") {
                for (value in values) {
                    include(value)
                }
                newline()
                -newFun
            }
        }
    }
}


