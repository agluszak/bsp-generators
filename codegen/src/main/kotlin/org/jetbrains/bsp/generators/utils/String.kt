package com.jetbrains.bsp.generators.utils

fun String.camelToSnakeCase(): String {
    val pattern = "(?<=[^A-Z])[A-Z]".toRegex()
    return this.replace(pattern, "_$0").lowercase()
}

fun String.snakeToUpperCamelCase(): String {
    val pattern = "_(.)".toRegex()
    return this.lowercase().replace(pattern) { it.groupValues[1].uppercase() }
        .replaceFirstChar { it.uppercaseChar() }
}

fun String.kebabToUpperCamelCase(): String {
    val pattern = "-(.)".toRegex()
    return this.lowercase().replace(pattern) { it.groupValues[1].uppercase() }
        .replaceFirstChar { it.uppercaseChar() }
}

fun String.toUpperCamelCase(): String {
    return this.lowercase().split("_").joinToString("") { word ->
        word.replaceFirstChar {
            it.uppercase()
        }
    }
}