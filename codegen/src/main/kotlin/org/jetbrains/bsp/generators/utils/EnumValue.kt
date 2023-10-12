package org.jetbrains.bsp.generators.utils

fun printEnumValue(value: Any?): String = when (value) {
    is Int -> "$value"
    is String -> """"$value""""
    else -> ""
}