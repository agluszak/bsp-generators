package com.jetbrains.bsp.generators.dsl

@DslMarker
annotation class CodeMarker

interface Element {
    fun render(builder: StringBuilder, indent: String)
}

class TextElement(val text: String) : Element {
    override fun render(builder: StringBuilder, indent: String) {
        builder.append("$indent$text\n")
    }
}

@CodeMarker
sealed class CodeBlock : Element {
    protected val children = arrayListOf<Element>()

    operator fun String.unaryMinus() {
        children.add(TextElement(this))
    }

    fun newline() {
        children.add(Newline())
    }

    fun line(line: String) {
        children.add(TextElement(line))
    }

    fun lines(lines: List<String>, join: String = "", end: String = "") {
        val linesWithoutLast = lines.dropLast(1)
        val last = lines.lastOrNull()
        for (line in linesWithoutLast) {
            children.add(TextElement(line + join))
        }
        if (last != null) {
            children.add(TextElement(last + end))
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

    override fun render(builder: StringBuilder, indent: String) {
        for (c in children) {
            c.render(builder, "$indent  ")
        }
    }

    override fun toString(): String {
        val builder = StringBuilder()
        render(builder, "")
        return builder.toString()
    }
}

class Newline : Element {
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
            c.render(builder, "$indent")
        }
    }
}

fun code(init: CodeBlock.() -> Unit): Code {
    val code = Code()
    code.init()
    return code
}
