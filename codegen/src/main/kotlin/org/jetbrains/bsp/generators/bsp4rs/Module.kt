package org.jetbrains.bsp.generators.bsp4rs

import org.jetbrains.bsp.generators.ir.Def

data class Module(val moduleName: String, val definitions: List<Def>)
