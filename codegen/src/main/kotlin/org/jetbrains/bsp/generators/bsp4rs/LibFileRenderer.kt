package org.jetbrains.bsp.generators.bsp4rs

import org.jetbrains.bsp.generators.CodegenFile
import org.jetbrains.bsp.generators.bsp4rs.def.renderStructure
import org.jetbrains.bsp.generators.dsl.CodeBlock
import org.jetbrains.bsp.generators.dsl.rustCode
import org.jetbrains.bsp.generators.ir.Def
import org.jetbrains.bsp.generators.ir.Field
import org.jetbrains.bsp.generators.ir.Type
import software.amazon.smithy.model.shapes.ShapeId
import kotlin.io.path.Path

fun RustRenderer.generateLibFile(modulesNames: List<String>): CodegenFile {
    val code = rustCode {
        include(renderImports(false))
        newline()
        lines(modulesNames.map { "pub mod $it" }, ";", ";")
        newline()
        lines(modulesNames.map { "use $it::*" }, ";", ";")
        newline()
        include(renderConsts())
        newline()
        include(renderRpcTraits())
        newline()
        include(renderOtherDataStruct())
        newline()
        include(renderTestsBlock())
    }

    return generateFile(code, Path(""), "lib.rs")
}

private fun RustRenderer.renderConsts(): CodeBlock {
    val versionStr = """pub const PROTOCOL_VERSION: &str = "$version";"""

    return rustCode {
        -versionStr
    }
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

private fun RustRenderer.renderOtherDataStruct(): CodeBlock {
    val def = Def.Structure(
        ShapeId.fromParts("bsp", "OtherData"),
        listOf(
            Field("dataKind", Type.String, true, listOf()),
            Field("data", Type.Json, true, listOf())
        ),
        listOf()
    )

    return renderStructure(def)
}

private fun renderTestsBlock(): CodeBlock {
    return rustCode {
        -"#[cfg(test)]"
        block("pub mod tests") {
            -"use serde::Deserialize;"
            newline()
            block("pub fn test_deserialization<T>(json: &str, expected: &T) where T: for<'de> Deserialize<'de> + PartialEq + std::fmt::Debug") {
                -"let value = serde_json::from_str::<T>(json).unwrap();"
                -"assert_eq!(&value, expected);"
            }
        }
    }
}
