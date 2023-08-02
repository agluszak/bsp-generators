import com.jetbrains.bsp.generators.dsl.code
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class DslTest {
    @Test
    fun simpleTest() {
        val input = code {
            -"val x = 5"
            newline()
            block("class Costam") {
                -"1"
                -"2"
                newline()
                paren("fun foo") {
                    -"3"
                    -"4"
                }
                newline()
                paren("fun bar") {
                    -"5"
                    -"6"
                }
            }
        }

        val expected = """
            val x = 5

            class Costam {
              1
              2

              fun foo(
                3
                4
              )

              fun bar(
                5
                6
              )
            }
            
        """.trimIndent()

        assertEquals(expected, input.toString())
    }

    @Test
    fun multiLines() {
        val input = code {
            lines(listOf("a", "b", "c"))
            lines(listOf("4", "5", "6"), join = " + ", end = " = 15")
        }

        val expected = """
            a
            b
            c
            4 + 
            5 + 
            6 = 15
            
        """.trimIndent()

        assertEquals(expected, input.toString())
    }
}