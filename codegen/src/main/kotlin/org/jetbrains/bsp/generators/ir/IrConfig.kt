package org.jetbrains.bsp.generators.ir

class IrConfig(
    val strings: TypeAliasing,
    val maps: TypeAliasing,
    val dataWithKind: AbstractionLevel,
    val openEnums: AbstractionLevel,
    val untaggedUnions: AbstractionLevel,
)

enum class AbstractionLevel {
    AsDef,
    AsType,
}

enum class TypeAliasing {
    Pure,
    Aliased,
}

