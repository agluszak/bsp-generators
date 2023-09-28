package bsp4kt.util

import kotlinx.serialization.DeserializationStrategy
import kotlinx.serialization.SerializationException
import kotlinx.serialization.json.JsonElement
import kotlinx.serialization.json.JsonPrimitive
import kotlinx.serialization.json.intOrNull

fun <T>untaggedUnionSerializer(stringSerializer: DeserializationStrategy<T>? = null, intSerializer: DeserializationStrategy<T>? = null): (JsonElement) -> DeserializationStrategy<T> {
    fun selectDeserializer(element: JsonElement): DeserializationStrategy<T> {
        val error = SerializationException("Unsupported type")

        return when (element) {
            is JsonPrimitive -> {
                if (element.isString) {
                    stringSerializer ?: throw error
                } else if (element.intOrNull != null) {
                    intSerializer ?: throw error
                } else {
                    throw error
                }
            }
            else -> throw error
        }
    }

    return ::selectDeserializer
}