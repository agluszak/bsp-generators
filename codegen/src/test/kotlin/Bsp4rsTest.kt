import org.jetbrains.bsp.generators.bsp4rs.RustRenderer
import org.jetbrains.bsp.generators.bsp4rs.SerializationRenderer
import org.jetbrains.bsp.generators.bsp4rs.renderType
import org.jetbrains.bsp.generators.dsl.CodeBlock
import org.jetbrains.bsp.generators.dsl.rustCode
import org.jetbrains.bsp.generators.ir.Field
import org.jetbrains.bsp.generators.ir.Hint
import org.jetbrains.bsp.generators.ir.Type
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test
import software.amazon.smithy.model.shapes.ShapeId

class Bsp4rsTest {
    private val renderer = RustRenderer("", emptyList(), "")
    private val serializationRenderer = SerializationRenderer()

    @Test
    fun renderType() {
        data class TestCase(val input: Type, val expected: String)

        val cases = listOf(
            TestCase(Type.TUnit, "()"),
            TestCase(Type.TBool, "bool"),
            TestCase(Type.TString, "String"),
            TestCase(Type.TInt, "i32"),
            TestCase(Type.TLong, "i64"),
            TestCase(Type.TJson, "serde_json::Value"),
            TestCase(Type.TSet(Type.TString), "BTreeSet<String>"),
            TestCase(Type.TList(Type.TLong), "Vec<i64>"),
            TestCase(Type.TMap(Type.TInt, Type.TBool), "BTreeMap<i32, bool>"),
            TestCase(Type.TRef(ShapeId.fromParts("bsp", "SomeType")), "SomeType"),
        )

        for (case in cases) {
            assertEquals(case.expected, renderer.renderType(case.input))
        }
    }

    @Test
    fun renderFieldSerialization() {
        data class TestCase(val input: Field, val expectedSerialization: CodeBlock)

        val cases = listOf(
            TestCase(
                Field("test", Type.TBool, true, emptyList()),
                rustCode { },
            ),
            TestCase(
                Field("test", Type.TBool, false, emptyList()),
                rustCode {
                    -"""#[serde(default, skip_serializing_if = "Option::is_none")]"""
                },
            ),
            TestCase(
                Field("test", Type.TList(Type.TInt), false, emptyList()),
                rustCode {
                    -"""#[serde(default, skip_serializing_if = "Vec::is_empty")]"""
                }
            ),
            TestCase(
                Field(
                    "test",
                    Type.TMap(Type.TString, Type.TUnit),
                    false,
                    emptyList()
                ),
                rustCode {
                    -"""#[serde(default, skip_serializing_if = "BTreeMap::is_empty")]"""
                }
            ),
            TestCase(
                Field("test", Type.TSet(Type.TJson), false, emptyList()),
                rustCode {
                    -"""#[serde(default, skip_serializing_if = "BTreeSet::is_empty")]"""
                }
            ),
        )

        for (case in cases) {
            assertEquals(
                case.expectedSerialization.toString(),
                serializationRenderer.renderForField(case.input).toString()
            )
        }
    }

    @Test
    fun renderHints() {
        data class TestCase(val input: List<Hint>, val expectedDocs: CodeBlock)

        val cases = listOf(
            TestCase(emptyList(), rustCode { }),
            TestCase(
                listOf(
                    Hint.Deprecated("NO show"),
                    Hint.Documentation("One line comment 1"),
                    Hint.JsonRename("NO show"),
                    Hint.Documentation("One line comment 2"),
                ),
                rustCode {
                    -"/// One line comment 1"
                    -"/// One line comment 2"
                    -"#[deprecated(note = \"NO show\")]"
                }
            ),
            TestCase(
                listOf(Hint.Documentation("More\nlines\ncomment")),
                rustCode {
                    -"/// More"
                    -"/// lines"
                    -"/// comment"
                }
            ),
        )

        for (case in cases) {
            assertEquals(case.expectedDocs.toString(), renderer.renderHints(case.input).toString())
        }
    }
}