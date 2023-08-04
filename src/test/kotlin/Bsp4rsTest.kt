import com.jetbrains.bsp.generators.bsp4rs.RustRenderer
import com.jetbrains.bsp.generators.dsl.code
import com.jetbrains.bsp.generators.ir.Field
import com.jetbrains.bsp.generators.ir.Type
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test
import software.amazon.smithy.model.shapes.ShapeId

class Bsp4rsTest {
    private val renderer = RustRenderer("", emptyList(), "")

    @Test
    fun renderType() {
        data class TestCase(val input: Type, val expectedType: String, val expectedOptionalType: String)

        val cases = listOf(
            TestCase(Type.Unit, "()", "Option<()>"),
            TestCase(Type.Bool, "bool", "Option<bool>"),
            TestCase(Type.String, "String", "Option<String>"),
            TestCase(Type.Int, "i32", "Option<i32>"),
            TestCase(Type.Long, "i64", "Option<i64>"),
            TestCase(Type.Json, "serde_json::Value", "Option<serde_json::Value>"),
            TestCase(Type.Set(Type.String), "BTreeSet<String>", "BTreeSet<String>"),
            TestCase(Type.List(Type.Long), "Vec<i64>", "Vec<i64>"),
            TestCase(Type.Map(Type.Int, Type.List(Type.Bool)), "HashMap<i32, Vec<bool>>", "HashMap<i32, Vec<bool>>"),
            TestCase(Type.Ref(ShapeId.fromParts("test", "Foo")), "Foo", "Option<Foo>"),
        )

        for (case in cases) {
            assertEquals(case.expectedType, renderer.renderType(case.input))
            assertEquals(case.expectedOptionalType, renderer.renderOptionalType(case.input))
        }
    }

    @Test
    fun renderField() {
        data class TestCase(val input: Field, val expectedSerialization: String?, val expectedVal: String)

        val cases = listOf(
            TestCase(
                Field("test1a", Type.Bool, false, emptyList()),
                """#[serde(skip_serializing_if = "Option::is_none")]""",
                "pub test1a: Option<bool>,"
            ),
            TestCase(
                Field("test1b", Type.Bool, true, emptyList()),
                null,
                "pub test1b: bool,"
            ),
            TestCase(
                Field("test2a", Type.List(Type.Int), false, emptyList()),
                """#[serde(default, skip_serializing_if = "Vec::is_empty")]""",
                "pub test2a: Vec<i32>,"
            ),
            TestCase(
                Field("test2b", Type.List(Type.Int), true, emptyList()),
                null,
                "pub test2b: Vec<i32>,"
            ),
            TestCase(
                Field("test3a", Type.Map(Type.String, Type.Unit), false, emptyList()),
                """#[serde(default, skip_serializing_if = "HashMap::is_empty")]""",
                "pub test3a: HashMap<String, ()>,"
            ),
            TestCase(
                Field("test3b", Type.Map(Type.String, Type.Unit), true, emptyList()),
                null,
                "pub test3b: HashMap<String, ()>,"
            ),
            TestCase(
                Field("test4a", Type.Set(Type.Json), false, emptyList()),
                """#[serde(default, skip_serializing_if = "BTreeSet::is_empty")]""",
                "pub test4a: BTreeSet<serde_json::Value>,"
            ),
            TestCase(
                Field("test4b", Type.Set(Type.Json), true, emptyList()),
                null,
                "pub test4b: BTreeSet<serde_json::Value>,"
            ),
        )

        for (case in cases) {
            assertEquals(case.expectedSerialization, renderer.renderFieldSerialization(case.input))
            assertEquals(case.expectedVal, renderer.renderFieldRaw(case.input))
            assertEquals(
                code {
                    -case.expectedSerialization
                    -case.expectedVal
                },
                renderer.renderRustField(case.input)
            )
        }
    }
}