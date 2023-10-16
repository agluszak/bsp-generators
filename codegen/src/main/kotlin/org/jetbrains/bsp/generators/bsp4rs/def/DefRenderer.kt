package org.jetbrains.bsp.generators.bsp4rs.def

import org.jetbrains.bsp.generators.bsp4rs.RustRenderer
import org.jetbrains.bsp.generators.dsl.CodeBlock
import org.jetbrains.bsp.generators.ir.Def

fun RustRenderer.renderDef(def: Def): CodeBlock = when (def) {
    is Def.Structure -> renderStructure(def)
    is Def.OpenEnum<*> -> renderOpenEnum(def)
    is Def.ClosedEnum<*> -> renderClosedEnum(def)
    is Def.Service -> renderService(def)
    is Def.Alias -> renderAlias(def)
    is Def.DataKinds -> renderDataKinds(def)
    is Def.UntaggedUnion -> renderUntaggedUnion(def)
}

fun RustRenderer.renderDefTest(def: Def): CodeBlock = when (def) {
    is Def.ClosedEnum<*> -> renderClosedEnumTest(def)
    is Def.OpenEnum<*> -> renderOpenEnumTest(def)
    is Def.Service -> renderServiceTest(def)
    is Def.Alias -> renderAliasTest(def)
    is Def.UntaggedUnion -> renderUntaggedUnionTest(def)
    is Def.Structure -> renderStructureTest(def)
    is Def.DataKinds -> renderDataKindsTest(def)
}

fun RustRenderer.renderDefDefault(def: Def): String = when (def) {
    is Def.Service -> ""
    is Def.UntaggedUnion -> renderUntaggedUnionDefault(def)
    is Def.DataKinds -> renderDataKindsDefault(def)
    else -> makeName(def.name) + "::default()"
}

fun RustRenderer.renderDefDefaultJson(def: Def): String = when (def) {
    is Def.Alias -> renderAliasDefaultJson(def)
    is Def.Structure -> renderStructureDefaultJson(def)
    is Def.ClosedEnum<*> -> renderClosedEnumDefaultJson(def)
    is Def.OpenEnum<*> -> renderOpenEnumDefaultJson(def)
    is Def.UntaggedUnion -> renderUntaggedUnionDefaultJson(def)
    else -> ""
}
