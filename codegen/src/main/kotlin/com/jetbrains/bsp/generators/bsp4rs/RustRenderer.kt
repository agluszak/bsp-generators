package com.jetbrains.bsp.generators.bsp4rs

import com.jetbrains.bsp.generators.CodegenFile
import com.jetbrains.bsp.generators.dsl.CodeBlock
import com.jetbrains.bsp.generators.dsl.rustCode
import com.jetbrains.bsp.generators.ir.*
import java.nio.file.Path
import kotlin.io.path.Path
import software.amazon.smithy.model.shapes.ShapeId

class RustRenderer(basepkg: String, private val modules: List<Module>, val version: String) {
    private val baseRelPath = Path(basepkg.replace(".", "/"))
    private val renames: Map<String, String> = mapOf(Pair("type", "r#type"), Pair("r#version", "version"))
    private val bannedAliases: List<String> = listOf("Integer", "Long")

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

    private fun renderData(def: Def.Data): CodeBlock {
        val dataKinds = def.kinds.map { kind ->
            val name = makeName(kind.kind.kebabToUpperCamelCase())
            val dataType = renderType(kind.type, true)
            Pair(name, dataType)
        }

        val namedName = "Named${def.name}"

        return rustCode {
            -"#[derive(${renderBasicDerives(false)}, ${renderSerializeDerives(true)})]"
            -"""#[serde(rename_all = "kebab-case", tag = "dataKind", content = "data")]"""
            block("pub enum $namedName") {
                lines(dataKinds.map { "${it.first}(${it.second})" }, ",", ",")
            }
            newline()
            lines(renderHints(def.hints))
            -"#[allow(clippy::large_enum_variant)]"
            -"#[derive(${renderBasicDerives(false)}, ${renderSerializeDerives(true)})]"
            -"""# [serde(untagged)]"""
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

    private fun isAliasRenderable(name: String, underlying: Type): Boolean {
        if (name in bannedAliases) return false
        if (underlying is Type.List) return false
        if (underlying is Type.Set) return false

        return true
    }

    //    TODO: change "pub type" to something else
    private fun renderAlias(def: Def.Alias): CodeBlock? {
        if (!isAliasRenderable(def.name, def.aliasedType)) return null

        return rustCode {
            lines(renderHints(def.hints))
            -"pub type ${def.name} = ${renderType(def.aliasedType, true)};"
            newline()
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
                Field("dataKind", Type.String, true, listOf(),),
                Field("data", Type.Json, true, listOf(),)
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
            is Def.Data -> renderData(def)
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

    //    TODO
//    private fun renderDeprecated(hints: List<Hint.Deprecated>): List<String> {
//        return emptyList()
//    }

    //    TODO
//    private fun renderRename(hints: List<Hint.JsonRename>): List<String> {
//        return emptyList()
//    }

    fun renderHints(hints: List<Hint>): List<String> {
        return renderDocumentation(hints.filterIsInstance<Hint.Documentation>())
//                + renderDeprecated(hints.filterIsInstance<Hint.Deprecated>()) + renderRename(hints.filterIsInstance<Hint.JsonRename>())
    }

    fun renderFieldSerialization(field: Field): String? {
        var serdeOpt = "default"

        if (field.name == "data") {
            serdeOpt += ", flatten"
        }

        if (!field.required) {
            serdeOpt += when (field.type) {
                is Type.List -> """, skip_serializing_if = "Vec::is_empty""""
                is Type.Map -> """, skip_serializing_if = "HashMap::is_empty""""
                is Type.Set -> """, skip_serializing_if = "BTreeSet::is_empty""""
                else -> """, skip_serializing_if = "Option::is_none""""
            }
        }

        return """#[serde($serdeOpt)]"""
    }

    private fun renderTypeName(type: Type): String = when (type) {
        Type.Bool -> "bool"
        Type.Int -> "i32"
        Type.Json -> "serde_json::Value"
        is Type.List -> "Vec<${renderTypeName(type.member)}>"
        Type.Long -> "i64"
        is Type.Map -> "BTreeMap<${renderTypeName(type.key)}, ${renderTypeName(type.value)}>"
        is Type.Ref -> makeName(type.shapeId.name)
        is Type.Set -> "BTreeSet<${renderTypeName(type.member)}>"
        Type.String -> "String"
        Type.Unit -> "()"
        is Type.Alias -> if (isAliasRenderable(type.shapeId.name, type.underlying)) makeName(type.shapeId.name) else renderTypeName(type.underlying)
    }

    fun renderType(type: Type, isRequired: Boolean): String {
        if (isRequired) return renderTypeName(type)

        return when (type) {
            is Type.List, is Type.Map, is Type.Set -> renderTypeName(type)
            else -> "Option<${renderTypeName(type)}>"
        }
    }

    fun renderStructFieldRaw(field: Field): String {
        return "pub ${makeName(field.name).camelToSnakeCase()}: ${renderType(field.type, field.required)}"
    }

    private fun renderStructField(field: Field): CodeBlock {
        return rustCode {
            lines(renderHints(field.hints))
            -renderFieldSerialization(field)
            -"${renderStructFieldRaw(field)},"
        }
    }

    private fun renderImports(canImportCrate: Boolean): CodeBlock {
        return rustCode {
            -"use serde::{Deserialize, Serialize};"
            -"use serde::de::DeserializeOwned;"
            -"use serde_repr::{Deserialize_repr, Serialize_repr};"
            -"use std::collections::{BTreeSet, BTreeMap};"
            -(if (canImportCrate) "use crate::*;" else null)
            newline()
        }
    }

    private fun renderBasicDerives(isDefault: Boolean): String {
        return """Debug, PartialEq, Eq, Clone""" + if (isDefault) ", Default" else ""
    }

    private fun renderSerializeDerives(isDefault: Boolean): String {
        return if (isDefault) """Serialize, Deserialize""" else "Serialize_repr, Deserialize_repr"
    }

    private fun renderStructure(def: Def.Structure): CodeBlock {
        return rustCode {
            lines(renderHints(def.hints))
            -"#[derive(${renderBasicDerives(true)}, ${renderSerializeDerives(true)})]"
            -"""#[serde(rename_all = "camelCase")]"""
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
            -"#[derive(Debug)]"
            -"pub enum $name {}"
            newline()
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

        val renderEnumSerialization: CodeBlock = when (def.enumType) {
            EnumType.IntEnum -> rustCode {
                -"#[derive(${renderBasicDerives(isDefault)}, ${renderSerializeDerives(false)})]"
                -"#[repr(u8)]"
            }

            EnumType.StringEnum -> rustCode {
                -"#[derive(${renderBasicDerives(isDefault)}, ${renderSerializeDerives(true)})]"
                -"""#[serde(rename_all = "kebab-case")]"""
            }
        }

        return rustCode {
            lines(renderHints(def.hints))
            include(renderEnumSerialization)
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
            -"#[derive(${renderBasicDerives(true)}, ${renderSerializeDerives(true)})]"
            -"#[serde(transparent)]"
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

fun String.camelToSnakeCase(): String {
    val pattern = "(?<=[^A-Z])[A-Z]".toRegex()
    return this.replace(pattern, "_$0").lowercase()
}

fun String.snakeToUpperCamelCase(): String {
    val pattern = "_(.)".toRegex()
    return this.lowercase().replace(pattern) { it.groupValues[1].uppercase() }
        .replaceFirstChar { it.uppercaseChar() }
}

fun String.kebabToUpperCamelCase(): String {
    val pattern = "-(.)".toRegex()
    return this.lowercase().replace(pattern) { it.groupValues[1].uppercase() }
        .replaceFirstChar { it.uppercaseChar() }
}
