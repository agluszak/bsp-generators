package com.jetbrains.bsp.generators.dsl

@DslMarker
annotation class CodeMarker

data class RenderSettings(
    val indent: String = "  ",
    val lineSeparator: String = "\n"
)

data class RenderContext(
    val settings: RenderSettings,
    var indentLevel: Int = 0
)

interface Renderable {
    fun render(builder: StringBuilder, context: RenderContext)
}

@CodeMarker
class CodeBlock(val context: RenderContext) : Renderable {
    protected val children = arrayListOf<Renderable>()

    operator fun String?.unaryMinus() {
        if (this != null) {
            line(this)
        }
    }

    fun newline() {
        children.add(Newline)
    }

    fun line(text: String) {
        children.add(Line(text))
    }

    fun removeNewline() {
        children.add(NoNewline)
    }

    fun indent() {
        children.add(Indent)
    }

    fun deindent() {
        children.add(Deindent)
    }

    fun elements(lines: List<Renderable>, join: Renderable? = null, end: Renderable? = null) {
        val linesWithoutLast = lines.dropLast(1)
        val last = lines.lastOrNull()
        for (line in linesWithoutLast) {
            children.add(line)
            join?.let { children.add(it) }
        }
        if (last != null) {
            children.add(last)
            end?.let { children.add(it) }
        }
    }

    val noIndentContext = RenderContext(context.settings, 0)

    fun lines(lines: List<String>, join: String? = null, end: String? = null) {
        elements(lines.zip(lines.indices).map { (it, i) ->
            CodeBlock(context).apply {
                line(it)
                if (join != null && (i < lines.size - 1 || end != null)) {
                    removeNewline()
                }
            }
        }, join?.let {
            CodeBlock(noIndentContext).apply {
                line(it)
            }
        }, end?.let {
            CodeBlock(noIndentContext).apply {
                line(it)
            }
        })
    }

    fun block(thisText: String, init: CodeBlock.() -> Unit) {
        val block = CodeBlock(context)

        block.line("$thisText {")
        block.indent()
        block.init()
        block.deindent()
        block.line("}")

        children.add(block)
    }

    fun paren(thisText: String, init: CodeBlock.() -> Unit) {
        val block = CodeBlock(context)

        block.line("$thisText(")
        block.indent()
        block.init()
        block.deindent()
        block.line(")")

        children.add(block)
    }

    fun code(init: CodeBlock.() -> Unit) {
        val block = CodeBlock(context)
        block.init()
        children.add(block)
    }

    fun include(other: Renderable) {
        children.add(other)
    }

    override fun render(builder: StringBuilder, context: RenderContext) {
        for (c in children) {
            c.render(builder, this.context)
        }
    }

    override fun toString(): String {
        val builder = StringBuilder()
        render(builder, context)
        return builder.toString()
    }
}

class Line(val text: String) : Renderable {
    override fun render(builder: StringBuilder, context: RenderContext) {
        for (i in 0 until context.indentLevel) {
            builder.append(context.settings.indent)
        }
        builder.append(text)
        builder.append(context.settings.lineSeparator)
    }
}

object Newline : Renderable {
    override fun render(builder: StringBuilder, context: RenderContext) {
        builder.append(context.settings.lineSeparator)
    }
}

object NoNewline : Renderable {
    override fun render(builder: StringBuilder, context: RenderContext) {
        if (builder.endsWith(context.settings.lineSeparator)) {
            builder.delete(builder.length - context.settings.lineSeparator.length, builder.length)
        }
    }
}

object Indent : Renderable {
    override fun render(builder: StringBuilder, context: RenderContext) {
        context.indentLevel++
    }
}

object Deindent : Renderable {
    override fun render(builder: StringBuilder, context: RenderContext) {
        require(context.indentLevel > 0)
        context.indentLevel--
    }
}

fun code(settings: RenderSettings = RenderSettings(), init: CodeBlock.() -> Unit): CodeBlock {
    val code = CodeBlock(RenderContext(settings))
    code.init()
    return code
}
