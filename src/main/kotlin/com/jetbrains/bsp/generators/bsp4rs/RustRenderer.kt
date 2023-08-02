package com.jetbrains.bsp.generators.bsp4rs

import com.jetbrains.bsp.generators.CodegenFile
import com.jetbrains.bsp.generators.ir.Def
import kotlin.io.path.Path

class RustRenderer(val basepkg: String, val definitions: List<Def>, val version: String) {
    val baseRelPath = Path(basepkg.replace(".", "/"))

    fun render(): List<CodegenFile> = TODO()
}
