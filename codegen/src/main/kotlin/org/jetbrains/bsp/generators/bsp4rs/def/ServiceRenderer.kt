package org.jetbrains.bsp.generators.bsp4rs.def

import org.jetbrains.bsp.generators.bsp4rs.RustRenderer
import org.jetbrains.bsp.generators.bsp4rs.renderType
import org.jetbrains.bsp.generators.dsl.CodeBlock
import org.jetbrains.bsp.generators.dsl.rustCode
import org.jetbrains.bsp.generators.ir.Def
import org.jetbrains.bsp.generators.ir.Hint
import org.jetbrains.bsp.generators.ir.JsonRpcMethodType
import org.jetbrains.bsp.generators.ir.Operation
import org.jetbrains.bsp.generators.utils.camelToSnakeCase

fun RustRenderer.renderService(def: Def.Service): CodeBlock =
    rustCode {
        def.operations.forEach { operation ->
            include(renderOperation(operation))
            newline()
        }
    }

private fun RustRenderer.renderOperation(op: Operation): CodeBlock {
    val name = makeName(op.name)
    val traitName = renderJsonRpcMethodType(op.jsonRpcMethodType)

    return rustCode {
        include(renderHints(op.hints))
        include(deriveRenderer.renderForOperation())
        block("pub enum $name") {}
        newline()
        if (op.hints.count { it is Hint.Deprecated } > 0) -"#[allow(deprecated)]"
        block("impl $traitName for $name") {
            include(renderOperationTraitProperties(op))
        }
    }
}

private fun renderJsonRpcMethodType(type: JsonRpcMethodType): String =
    when (type) {
        JsonRpcMethodType.Notification -> "Notification"
        JsonRpcMethodType.Request -> "Request"
    }

private fun RustRenderer.renderOperationTraitProperties(op: Operation): CodeBlock {
    val input = "type Params = ${renderType(op.inputType)}"
    val output = when (op.jsonRpcMethodType) {
        JsonRpcMethodType.Notification -> null
        JsonRpcMethodType.Request -> "type Result = ${renderType(op.outputType)}"
    }
    val method = """const METHOD: &'static str = "${op.jsonRpcMethod}""""

    return rustCode {
        lines(listOfNotNull(input, output, method), join = ";", end = ";")
    }
}

fun RustRenderer.renderServiceTest(def: Def.Service): CodeBlock =
    rustCode {
        def.operations.forEach { operation ->
            include(renderOperationTest(operation))
            newline()
        }
    }

private fun RustRenderer.renderOperationTest(op: Operation): CodeBlock {
    val name = makeName(op.name)

    return rustCode {
        if (op.hints.count { it is Hint.Deprecated } > 0) -"#[allow(deprecated)]"
        -"#[test]"
        block("fn ${name.camelToSnakeCase()}_method()") {
            -"""assert_eq!($name::METHOD, "${op.jsonRpcMethod}");"""
        }
    }
}
