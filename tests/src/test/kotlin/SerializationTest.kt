import org.jetbrains.bsp.generators.Loader
import org.jetbrains.bsp.generators.bsp4json.JsonRenderer2
import org.junit.jupiter.api.Test
import org.junit.jupiter.api.BeforeAll
import java.io.BufferedReader
import java.io.InputStreamReader
import java.io.PrintWriter
import java.io.File
import com.fasterxml.jackson.databind.ObjectMapper
import kotlinx.serialization.Serializable
import kotlinx.serialization.encodeToString
import org.jetbrains.bsp.generators.ir.Def
import org.jetbrains.bsp.generators.ir.IrConfig
import org.jetbrains.bsp.generators.ir.SmithyToIr
import org.jetbrains.bsp.generators.ir.TypeAliasing
import org.jetbrains.bsp.generators.ir.AbstractionLevel
import kotlinx.serialization.json.Json
import kotlinx.serialization.json.JsonElement
import kotlinx.serialization.json.encodeToJsonElement
import org.jetbrains.bsp.generators.bsp4json.ContentsType
import org.jetbrains.bsp.generators.bsp4json.NotRequiredState
import org.junit.jupiter.api.Assertions.assertEquals

@Serializable
data class DataWrapper(val name: String, val jsonData: JsonElement)

class SerializationTest {
    @Test
    fun empty() {
        assert(true)

        val objectMapper = ObjectMapper()
        val json = Json { ignoreUnknownKeys = true }
        val processBuilder = ProcessBuilder("./tests/kotlin_json_roundtrip").directory(File("."))
        println(processBuilder.command())
        val process = processBuilder.start()

        val writer = PrintWriter(process.outputStream)
        val reader = BufferedReader(InputStreamReader(process.inputStream))

        testData.forEach { data ->
            val input = json.encodeToJsonElement(data)
            println("INPUT: $input")
            writer.println(input)
            writer.flush()

            val response = reader.readLine()
            println("RES  : $response")
            assertEquals(objectMapper.readTree(json.encodeToString(data.jsonData)), objectMapper.readTree(response));
        }

        writer.println("exit")
        writer.flush()

        process.waitFor()
        writer.close()
        reader.close()
    }

    companion object {

        var testData: List<DataWrapper> = listOf()

        @JvmStatic
        @BeforeAll
        fun prepareShapes() {
            val model = Loader.model
            val namespaces = Loader.namespaces
            val irConfig = IrConfig(
                strings = TypeAliasing.Pure,
                maps = TypeAliasing.Pure,
                dataWithKind = AbstractionLevel.AsType,
                openEnums = AbstractionLevel.AsType,
                untaggedUnions = AbstractionLevel.AsDef,
            )
            val ir = SmithyToIr(model, irConfig)
            val definitions = namespaces.flatMap { ir.definitions(it) }

            val jsonRenderer = JsonRenderer2(definitions)
            val shapes =
                definitions.associateBy { it.shapeId }.filterValues { it is Def.Structure }.mapValues { (_, def) ->
                    listOf(
                        jsonRenderer.renderDefJson(def, ContentsType.TestValue, NotRequiredState.Exclude),
                        jsonRenderer.renderDefJson(def, ContentsType.Default, NotRequiredState.Include),
                        jsonRenderer.renderDefJson(def, ContentsType.TestValue, NotRequiredState.Include),
                    )
                }

            testData = shapes.flatMap { (id, jsons) ->
                jsons.map { jsonData -> DataWrapper(id.name.toString(), jsonData) }
            }
        }
    }
}