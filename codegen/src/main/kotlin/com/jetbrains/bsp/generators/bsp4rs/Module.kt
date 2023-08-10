package com.jetbrains.bsp.generators.bsp4rs

import com.jetbrains.bsp.generators.ir.Def

data class Module(val moduleName: String, val definitions: List<Def>)