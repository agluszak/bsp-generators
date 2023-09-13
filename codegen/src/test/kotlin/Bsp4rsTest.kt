import org.jetbrains.bsp.generators.bsp4rs.RustRenderer
import org.jetbrains.bsp.generators.ir.Hint
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class Bsp4rsTest {
    private val renderer = RustRenderer("", emptyList(), "")

//    @Test
//    fun renderType() {
//        data class TestCase(val input: Type, val expectedRequired: String, val expectedOptional: String)
//
//        val cases = listOf(
//            TestCase(Type.Unit, "()", "Option<()>"),
//            TestCase(Type.Bool, "bool", "Option<bool>"),
//            TestCase(Type.String, "String", "Option<String>"),
//            TestCase(Type.Int, "i32", "Option<i32>"),
//            TestCase(Type.Long, "i64", "Option<i64>"),
//            TestCase(Type.Json, "serde_json::Value", "Option<serde_json::Value>"),
//            TestCase(Type.Set(Type.String), "BTreeSet<String>", "BTreeSet<String>"),
//            TestCase(Type.List(Type.Long), "Vec<i64>", "Vec<i64>"),
//            TestCase(Type.Map(Type.Int, Type.List(Type.Bool)), "HashMap<i32, Vec<bool>>", "HashMap<i32, Vec<bool>>"),
//            TestCase(Type.Ref(ShapeId.fromParts("test", "Foo")), "Foo", "Option<Foo>"),
//        )
//
//        for (case in cases) {
//            assertEquals(case.expectedRequired, renderer.renderType(case.input, true))
//            assertEquals(case.expectedOptional, renderer.renderType(case.input, false))
//        }
//    }

//    @Test
//    fun renderFieldRaw() {
//        data class TestCase(val input: Field, val expectedVal: String)
//
//        val cases = listOf(
//            TestCase(
//                Field("test1", Type.Bool, false, emptyList()),
//                "pub test1: Option<bool>"
//            ),
//            TestCase(
//                Field("test2", Type.Bool, true, emptyList()),
//                "pub test2: bool"
//            ),
//            TestCase(
//                Field("NameTest1", Type.Bool, true, emptyList()),
//                "pub name_test1: bool"
//            ),
//            TestCase(
//                Field("nameTest2", Type.Bool, true, emptyList()),
//                "pub name_test2: bool"
//            ),
//            TestCase(
//                Field("name_test_3", Type.Bool, true, emptyList()),
//                "pub name_test_3: bool"
//            ),
//            TestCase(
//                Field("name-test-4", Type.Bool, true, emptyList()),
//                "pub name-test-4: bool"
//            ),
//        )
//
//        for (case in cases) {
//            assertEquals(case.expectedVal, renderer.renderStructFieldRaw(case.input))
//        }
//    }

//    @Test
//    fun renderFieldSerialization() {
//        data class TestCase(val input: Field, val expectedSerialization: String?)
//
//        val cases = listOf(
//            TestCase(
//                Field("test", Type.Bool, true, emptyList()),
//                null,
//            ),
//            TestCase(
//                Field("test", Type.Bool, false, emptyList()),
//                """#[serde(skip_serializing_if = "Option::is_none")]""",
//            ),
//            TestCase(
//                Field("test", Type.List(Type.Int), false, emptyList()),
//                """#[serde(default, skip_serializing_if = "Vec::is_empty")]""",
//            ),
//            TestCase(
//                Field("test", Type.Map(Type.String, Type.Unit), false, emptyList()),
//                """#[serde(default, skip_serializing_if = "HashMap::is_empty")]""",
//            ),
//            TestCase(
//                Field("test", Type.Set(Type.Json), false, emptyList()),
//                """#[serde(default, skip_serializing_if = "BTreeSet::is_empty")]""",
//            ),
//        )
//
//        for (case in cases) {
//            assertEquals(case.expectedSerialization, renderer.renderFieldSerialization(case.input))
//        }
//    }

    @Test
    fun renderHints() {
        data class TestCase(val input: List<Hint>, val expectedDocs: List<String>)

        val cases = listOf(
            TestCase(emptyList(), emptyList()),
            TestCase(
                listOf(
                    Hint.Deprecated("NO show"),
                    Hint.Documentation("One line comment 1"),
                    Hint.JsonRename("NO show"),
                    Hint.Documentation("One line comment 2"),
                ),
                listOf("/** One line comment 1 */", "/** One line comment 2 */")
            ),
            TestCase(
                listOf(Hint.Documentation("More\nlines\ncomment")),
                listOf("/** More", "lines", "comment */")
            ),
        )

        for (case in cases) {
            assertEquals(case.expectedDocs, renderer.renderHints(case.input))
        }
    }

//    @Test
//    fun renderOperation() {
//        data class TestCase(val input: Operation, val expected: String)
//
//        val cases = listOf(
//            TestCase(
//                Operation(
//                    ShapeId.fromParts("test", "TestReq"),
//                    Type.Bool,
//                    Type.Ref(ShapeId.fromParts("test", "RequestResult")),
//                    JsonRpcMethodType.Request,
//                    "test-req",
//                    emptyList()
//                ),
//                """
//                    #[derive(Debug)]
//                    pub enum TestReq {}
//
//                    impl Request for TestReq {
//                        type Params = bool;
//                        type Result = RequestResult;
//                        const METHOD: &'static str = "test-req";
//                    }
//
//
//                """.trimIndent()
//            ),
//            TestCase(
//                Operation(
//                    ShapeId.fromParts("test", "TestNot"),
//                    Type.Unit,
//                    Type.Int,
//                    JsonRpcMethodType.Notification,
//                    "test-not",
//                    emptyList()
//                ),
//                """
//                    #[derive(Debug)]
//                    pub enum TestNot {}
//
//                    impl Notification for TestNot {
//                        type Params = ();
//                        const METHOD: &'static str = "test-not";
//                    }
//
//
//                """.trimIndent()
//            ),
//        )
//
//        for (case in cases) {
//            assertEquals(case.expected, renderer.renderOperation(case.input).toString())
//        }
//    }
}