
import org.junit.jupiter.api.Test
import java.io.BufferedReader
import java.io.InputStreamReader
import java.io.PrintWriter
import java.io.File

class SerializationTest {
    @Test
    fun empty() {
        assert(true)
        val commands = listOf("{}", "{}", "{}", "exit")

        val processBuilder = ProcessBuilder("./tests/kotlin_json_roundtrip").directory(File("."))
        println(processBuilder.command())
        val process = processBuilder.start()

        val writer = PrintWriter(process.outputStream)
        val reader = BufferedReader(InputStreamReader(process.inputStream))

        commands.forEach { command ->
            writer.println(command)
            writer.flush()

            val response = reader.readLine()
            println(response)
        }

        process.waitFor()
        writer.close()
        reader.close()
    }
}