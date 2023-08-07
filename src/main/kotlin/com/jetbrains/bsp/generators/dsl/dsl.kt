package com.jetbrains.bsp.generators.dsl

@DslMarker
annotation class CodeMarker

@CodeMarker
sealed class CodeBlock {
    protected val children = arrayListOf<CodeBlock>()

    operator fun String?.unaryMinus() {
        if (this != null) {
            children.add(Line(this))
        }
    }

    fun newline() {
        children.add(Newline())
    }

    fun line(line: String) {
        children.add(Line(line))
    }

    fun lines(lines: List<String>, join: String = "", end: String = "") {
        val linesWithoutLast = lines.dropLast(1)
        val last = lines.lastOrNull()
        for (line in linesWithoutLast) {
            children.add(Line(line + join))
        }
        if (last != null) {
            children.add(Line(last + end))
        }
    }

    fun block(thisText: String, init: CodeBlock.() -> Unit) {
        val block = Block(thisText)
        block.init()
        children.add(block)
    }

    fun paren(thisText: String, init: CodeBlock.() -> Unit) {
        val block = Paren(thisText)
        block.init()
        children.add(block)
    }

    fun code(other: CodeBlock) {
        children.add(other)
    }

    open fun render(builder: StringBuilder, indent: String) {
        for (c in children) {
            c.render(builder, "$indent  ")
        }
    }

    override fun toString(): String {
        val builder = StringBuilder()
        render(builder, "")
        return builder.toString()
    }

    override fun equals(other: Any?): Boolean {
        if (this === other) return true
        if (javaClass != other?.javaClass) return false

        other as CodeBlock

        return children == other.children
    }
}

class Line(val text: String) : CodeBlock() {
    override fun render(builder: StringBuilder, indent: String) {
        builder.append("$indent$text\n")
    }

    override fun equals(other: Any?): Boolean {
        if (this === other) return true
        if (other !is Line) return false
        if (text != other.text) return false

        return true
    }
}

class Newline : CodeBlock() {
    override fun render(builder: StringBuilder, indent: String) {
        builder.append("\n")
    }
}

sealed class Wrapper(val thisText: String, val begin: String, val end: String) : CodeBlock() {
    override fun render(builder: StringBuilder, indent: String) {
        builder.append("$indent$thisText$begin\n")
        super.render(builder, indent)
        builder.append("$indent$end\n")
    }
}

class Block(thisText: String) : Wrapper(thisText, " {", "}")
class Paren(thisText: String) : Wrapper(thisText, "(", ")")

class Code : CodeBlock() {
    override fun render(builder: StringBuilder, indent: String) {
        for (c in children) {
            c.render(builder, indent)
        }
    }
}

fun code(init: CodeBlock.() -> Unit): Code {
    val code = Code()
    code.init()
    return code
}
