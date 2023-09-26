package org.jetbrains.bsp.generators.ir

class IrConfig(
    val strings: TypeAliasing = TypeAliasing.Pure,
    val maps: TypeAliasing = TypeAliasing.Pure,
    val dataWithKind: TypeAliasing = TypeAliasing.Pure,
    val openEnum: DefinitionLevel = DefinitionLevel.AsDef,
)

enum class DefinitionLevel {
    AsDef,
    AsType,
}

enum class TypeAliasing {
    Pure,
    Aliased,
}

