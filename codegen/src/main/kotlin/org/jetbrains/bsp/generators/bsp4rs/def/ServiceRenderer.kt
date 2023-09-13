package org.jetbrains.bsp.generators.bsp4rs.def

import org.jetbrains.bsp.generators.bsp4rs.RustRenderer
import org.jetbrains.bsp.generators.bsp4rs.renderIrShapeType
import org.jetbrains.bsp.generators.dsl.CodeBlock
import org.jetbrains.bsp.generators.dsl.rustCode
import org.jetbrains.bsp.generators.ir.Def
import org.jetbrains.bsp.generators.ir.JsonRpcMethodType
import org.jetbrains.bsp.generators.ir.Operation

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
        include(deriveRenderer.renderForOp())
        block("pub enum $name") {}
        newline()
        include(renderHints(op.hints))
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
    val input = "type Params = ${renderIrShapeType(op.inputType)}"
    val output = when (op.jsonRpcMethodType) {
        JsonRpcMethodType.Notification -> null
        JsonRpcMethodType.Request -> "type Result = ${renderIrShapeType(op.outputType)}"
    }
    val method = """const METHOD: &'static str = "${op.jsonRpcMethod}""""

    return rustCode {
        lines(listOfNotNull(input, output, method), join = ";", end = ";")
    }
}
