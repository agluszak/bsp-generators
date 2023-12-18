package org.jetbrains.bsp.generators.bsp4rs

import org.jetbrains.bsp.generators.CodegenFile
import org.jetbrains.bsp.generators.bsp4json.ContentsType
import org.jetbrains.bsp.generators.bsp4json.makeCompactJsonString
import org.jetbrains.bsp.generators.bsp4rs.def.renderStructure
import org.jetbrains.bsp.generators.bsp4rs.def.renderStructureTest
import org.jetbrains.bsp.generators.dsl.CodeBlock
import org.jetbrains.bsp.generators.dsl.rustCode
import org.jetbrains.bsp.generators.ir.Type
import kotlin.io.path.Path

fun RustRenderer.generateLibFile(modulesNames: List<String>): CodegenFile {
    val code = rustCode {
        -"#![allow(deprecated)]"
        newline()
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
        include(renderStructure(otherDataDef))
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

private fun RustRenderer.renderTestsBlock(): CodeBlock {
    val consts = listOf(Type.Bool, Type.Int, Type.Long, Type.String).map {
        Pair(
            Pair(
                renderTypeTestConstName(it),
                renderTypeTestConstType(it)
            ),
            renderTypeTestConstValue(it)
        )
    }

    return rustCode {
        -"#[cfg(test)]"
        block("pub mod tests") {
            include(renderTestsImports(false))
            newline()
            consts.forEach { (nameTypePair, value) ->
                -"""pub const ${nameTypePair.first}: ${nameTypePair.second} = $value;"""
            }
            newline()
            block("pub fn test_deserialization<T>(json: &str, expected: &T) where T: for<'de> Deserialize<'de> + PartialEq + std::fmt::Debug") {
                -"let value = serde_json::from_str::<T>(json).unwrap();"
                -"assert_eq!(&value, expected);"
            }
            newline()
            include(renderStructureTest(otherDataDef))
        }
    }
}

fun renderTypeTestConstType(type: Type): String = when (type) {
    is Type.Bool -> "bool"
    is Type.Int -> "i32"
    is Type.Long -> "i64"
    is Type.String -> "&str"
    else -> ""
}

fun RustRenderer.renderTypeTestConstValue(type: Type): String =
    makeCompactJsonString(jsonRenderer.renderTypeJson(type, ContentsType.TestOnlyPrimitive))
