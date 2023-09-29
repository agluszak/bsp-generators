package org.jetbrains.bsp.generators.ir

// This configuration allows to specify preferences for how the smithy model is represented internally.
// This enhances generating libraries that fully utilize the particular language's features.
class IrConfig(
    val strings: TypeAliasing,
    val maps: TypeAliasing,
    val dataWithKind: AbstractionLevel,
    val openEnums: AbstractionLevel,
    val untaggedUnions: AbstractionLevel,
)

// It specifies how to represent a particular type, if it should be aliased or not.
//
// Example: 2 ways of using strings in kotlin
// - Pure:
//      `val field: String`
// - Aliased:
//      `typealias AliasedStringName = String`
//      `val field: AliasedStringName`
//
enum class TypeAliasing {
    Pure,
    Aliased,
}

// It specifies if we would like to treat a particular shape as a type or as a top level definition.
//
// Example: representing UntaggedUnion
// - AsType: (scala)
//      `val field: Either[String, Int]`
// - AsDef: (rust)
//      ```
//      pub enum EitherTypeName {
//          String(String),
//          I32(i32),
//      }
//       ```
//      `val field: EitherTypeName`
//
enum class AbstractionLevel {
    AsDef,
    AsType,
}
