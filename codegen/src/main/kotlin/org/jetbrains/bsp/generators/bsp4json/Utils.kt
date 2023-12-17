package org.jetbrains.bsp.generators.bsp4json

import com.google.gson.GsonBuilder
import com.google.gson.JsonParser
import kotlinx.serialization.ExperimentalSerializationApi
import kotlinx.serialization.encodeToString
import kotlinx.serialization.json.Json
import kotlinx.serialization.json.JsonElement

fun makeCompactJsonString(jsonElement: JsonElement): String {
    return makeKotlinxPrettyJsonString(jsonElement, "")
        .replace(",\n", ", ")
        .replace("\n", "")
}

@OptIn(ExperimentalSerializationApi::class)
private fun makeKotlinxPrettyJsonString(jsonElement: JsonElement, ident: String = "  "): String {
    val json = Json { encodeDefaults = false; explicitNulls = true; prettyPrint = true; prettyPrintIndent = ident }
    return json.encodeToString(jsonElement)
}

fun makeGsonPrettyJsonString(jsonElement: JsonElement): String {
    val input = jsonElement.toString()
    val jsonElement2 = JsonParser.parseString(input)
    val gson = GsonBuilder().setPrettyPrinting().serializeNulls().create()
    return gson.toJson(jsonElement2)
}