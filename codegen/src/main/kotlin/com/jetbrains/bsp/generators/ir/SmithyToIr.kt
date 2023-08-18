package com.jetbrains.bsp.generators.ir

import software.amazon.smithy.model.Model
import software.amazon.smithy.model.shapes.BooleanShape
import software.amazon.smithy.model.shapes.DocumentShape
import software.amazon.smithy.model.shapes.EnumShape
import software.amazon.smithy.model.shapes.IntEnumShape
import software.amazon.smithy.model.shapes.IntegerShape
import software.amazon.smithy.model.shapes.ListShape
import software.amazon.smithy.model.shapes.LongShape
import software.amazon.smithy.model.shapes.MapShape
import software.amazon.smithy.model.shapes.MemberShape
import software.amazon.smithy.model.shapes.OperationShape
import software.amazon.smithy.model.shapes.ServiceShape
import software.amazon.smithy.model.shapes.Shape
import software.amazon.smithy.model.shapes.ShapeId
import software.amazon.smithy.model.shapes.ShapeVisitor
import software.amazon.smithy.model.shapes.StringShape
import software.amazon.smithy.model.shapes.StructureShape
import software.amazon.smithy.model.traits.DeprecatedTrait
import software.amazon.smithy.model.traits.DocumentationTrait
import software.amazon.smithy.model.traits.MixinTrait
import software.amazon.smithy.model.traits.RequiredTrait
import traits.DataKindTrait
import traits.DataTrait
import traits.EnumKindTrait
import traits.JsonNotificationTrait
import traits.JsonRequestTrait
import traits.SetTrait
import java.util.Locale
import java.util.stream.Collectors
import kotlin.jvm.optionals.getOrNull

class SmithyToIr(val model: Model) {
    val allDataKindAnnotated: Map<ShapeId, List<PolymorphicDataKind>> = run {
        val allExtendableTypes = model.getShapesWithTrait(DataTrait::class.java).toList()
        val allExtendableTypeIds = allExtendableTypes.map { it.id }.toSet()

        val dataKindInhabitants = model
            .getShapesWithTrait(DataKindTrait::class.java)
            .toList()
            .map { shape ->
                val dataKindTrait = shape.expectTrait(DataKindTrait::class.java)
                shape to dataKindTrait
            }

        // Validate that all data kinds extend a known extendable type.
        dataKindInhabitants.forEach { (shape, dataKindTrait) ->
            val correct = dataKindTrait.polymorphicData.all { allExtendableTypeIds.contains(it) }
            if (!correct) {
                throw RuntimeException(
                    "DataKindTrait on ${shape.id.name} must extend a known extendable type."
                )
            }
        }

        val groupedInhabitants = dataKindInhabitants
            .flatMap { (shape, dataKindTrait) ->
                dataKindTrait.polymorphicData.map { Triple(shape, dataKindTrait.kind, it) }
            }
            .groupBy { (_, _, referencedShapeId) -> referencedShapeId }
            .map { (dataType, shapeAndTraits) ->
                dataType to shapeAndTraits.map { (shape, dataKind, _) ->
                    PolymorphicDataKind(dataKind, shape.id)
                }
            }.toMap()

        allExtendableTypeIds.associateWith { id ->
            val inhabitants = groupedInhabitants.getOrDefault(id, emptyList()).sortedBy { it.kind }
            inhabitants
        }
    }

    fun definitions(namespace: String): List<Def> =
        model.shapes().filter { it.id.namespace == namespace && !it.hasTrait("smithy.api#trait") }
            .flatMap { it.accept(toDefVisitor).stream() }.collect(Collectors.toList())

    private val toDefVisitor = object : ShapeVisitor.Default<List<Def>>() {
        override fun getDefault(shape: Shape): List<Def> = emptyList()

        fun buildOperation(op: OperationShape): Operation? {
            val maybeMethod = when {
                op.hasTrait(JsonRequestTrait::class.java) -> {
                    val methodName = op.expectTrait(JsonRequestTrait::class.java).value!!
                    val methodType = JsonRpcMethodType.Request
                    methodName to methodType
                }

                op.hasTrait(JsonNotificationTrait::class.java) -> {
                    val methodName = op.expectTrait(JsonNotificationTrait::class.java).value!!
                    val methodType = JsonRpcMethodType.Notification
                    methodName to methodType
                }

                else -> null
            }

            return maybeMethod?.let { (methodName, methodType) ->
                val inputType = getType(op.input.getOrNull()) ?: Type.Unit
                val outputType = getType(op.output.getOrNull()) ?: Type.Unit
                val hints = getHints(op)
                Operation(op.id, inputType, outputType, methodType, methodName, hints)
            }
        }

        override fun serviceShape(shape: ServiceShape): List<Def> {
            val operations = shape.operations.toList()
                .map { model.expectShape(it, OperationShape::class.java) }
                .mapNotNull { buildOperation(it) }

            return listOf(Def.Service(shape.id, operations, getHints(shape)))
        }

        fun toField(member: MemberShape): Field? {
            val required = member.hasTrait(RequiredTrait::class.java)
            val name = member.memberName
            return getType(member.target)?.let {
                Field(name, it, required, getHints(member))
            }
        }

        fun getType(shapeId: ShapeId?): Type? = shapeId?.let { model.expectShape(it).accept(toTypeVisitor) }

        override fun structureShape(shape: StructureShape): List<Def> {
            // Skip shapes that are used as mixins.
            if (shape.hasTrait(MixinTrait::class.java)) {
                return emptyList()
            }

            val fields = shape.members().mapNotNull { toField(it) }

            return listOf(Def.Structure(shape.id, fields, getHints(shape)))
        }

        override fun intEnumShape(shape: IntEnumShape): List<Def> {
            val enumValues = shape.enumValues.map { (name, value) ->
                val valueHints = getHints(shape.allMembers.getValue(name))
                EnumValue(name, value!!, valueHints)
            }

            val hints = getHints(shape)
            return when (shape.expectTrait(EnumKindTrait::class.java).enumKind) {
                EnumKindTrait.EnumKind.OPEN ->
                    listOf(Def.OpenEnum(shape.id, EnumType.IntEnum, enumValues.sortedBy { it.value }, hints))

                EnumKindTrait.EnumKind.CLOSED ->
                    listOf(Def.ClosedEnum(shape.id, EnumType.IntEnum, enumValues, hints))
            }
        }

        override fun enumShape(shape: EnumShape): List<Def> {
            val enumValues = shape.enumValues.map { (name, value) ->
                val valueHints = getHints(shape.allMembers.getValue(name))
                EnumValue(name, value!!, valueHints)
            }

            val hints = getHints(shape)
            return when (shape.expectTrait(EnumKindTrait::class.java).enumKind) {
                EnumKindTrait.EnumKind.OPEN ->
                    listOf(Def.OpenEnum(shape.id, EnumType.StringEnum, enumValues.sortedBy { it.name }, hints))

                EnumKindTrait.EnumKind.CLOSED ->
                    listOf(Def.ClosedEnum(shape.id, EnumType.StringEnum, enumValues, hints))
            }
        }

        override fun documentShape(shape: DocumentShape): List<Def> {
            val hints = getHints(shape)

            // A document shape with the data trait in fact defines two structures: one for the data (a type alias for any)
            // and one for the data kind (an enum).
            return if (shape.hasTrait(DataTrait::class.java)) {
                val id = shape.id
                val dataKinds = allDataKindAnnotated[id]!!.mapNotNull { polyDataKind ->
                    getType(polyDataKind.shapeId)?.let { PolymorphicData(polyDataKind.kind, it) }
                }

                listOf(Def.Data(id, dataKinds, hints))
            } else {
                typeShape(shape)
            }
        }

        fun typeShape(shape: Shape): List<Def> {
            val hints = getHints(shape)
            return when (shape) {
                is BooleanShape -> listOf(Def.Alias(shape.getId(), Type.Bool, hints))
                is IntegerShape -> listOf(Def.Alias(shape.getId(), Type.Int, hints))
                is LongShape -> listOf(Def.Alias(shape.getId(), Type.Long, hints))
                is StringShape -> listOf(Def.Alias(shape.getId(), Type.String, hints))
                is DocumentShape -> listOf(Def.Alias(shape.getId(), Type.Json, hints))
                is ListShape -> listOfNotNull(
                    toTypeVisitor.listShape(shape)?.let { Def.Alias(shape.getId(), it, hints) })

                is MapShape -> listOfNotNull(toTypeVisitor.mapShape(shape)?.let { Def.Alias(shape.getId(), it, hints) })
                else -> emptyList()
            }
        }

        override fun booleanShape(shape: BooleanShape): List<Def> = typeShape(shape)

        override fun integerShape(shape: IntegerShape): List<Def> = typeShape(shape)

        override fun longShape(shape: LongShape): List<Def> = typeShape(shape)

        override fun stringShape(shape: StringShape): List<Def> = typeShape(shape)

        override fun listShape(shape: ListShape): List<Def> = typeShape(shape)

        override fun mapShape(shape: MapShape): List<Def> = typeShape(shape)

    }

    val toTypeVisitor = object : ShapeVisitor.Default<Type?>() {
        override fun getDefault(shape: Shape): Type? = null

        private fun primitiveShape(shape: Shape): Type? {
            return when (shape) {
                is BooleanShape -> Type.Bool
                is IntegerShape -> Type.Int
                is LongShape -> Type.Long
                is StringShape -> Type.String
                is DocumentShape -> Type.Json
                else -> null
            }
        }

        private fun primitiveOrAliasShape(shape: Shape): Type? {
            val id = shape.id

            return primitiveShape(shape)?.let { if (id.namespace == "smithy.api") it else Type.Alias(id, it) }
        }

        override fun booleanShape(shape: BooleanShape): Type? = primitiveOrAliasShape(shape)

        override fun integerShape(shape: IntegerShape): Type? = primitiveOrAliasShape(shape)

        override fun longShape(shape: LongShape): Type? = primitiveOrAliasShape(shape)

        override fun stringShape(shape: StringShape): Type? = primitiveOrAliasShape(shape)

        override fun documentShape(shape: DocumentShape): Type? {
            return if (shape.hasTrait(DataTrait::class.java)) {
                Type.Ref(shape.id)
            } else {
                primitiveOrAliasShape(shape)
            }
        }

        override fun listShape(shape: ListShape): Type? {
            return shape.member.accept(this)?.let { memberType ->
                if (shape.hasTrait(SetTrait::class.java)) Type.Set(memberType)
                else Type.List(memberType)
            }
        }

        override fun mapShape(shape: MapShape): Type? {
            return shape.key.accept(this)?.let { key ->
                shape.value.accept(this)?.let { value ->
                    Type.Map(key, value)
                }
            }
        }

        override fun structureShape(shape: StructureShape): Type = Type.Ref(shape.id)

        override fun enumShape(shape: EnumShape): Type = Type.Ref(shape.id)

        override fun intEnumShape(shape: IntEnumShape): Type = Type.Ref(shape.id)

        override fun memberShape(shape: MemberShape): Type? = model.expectShape(shape.target).accept(this)
    }

    fun getHints(shape: Shape): List<Hint> {
        val documentation = shape
            .getTrait(DocumentationTrait::class.java)
            .map { Hint.Documentation(it.value) }

        val deprecated = shape
            .getTrait(DeprecatedTrait::class.java)
            .map { Hint.Deprecated(it.message.orElse("")) }

        return listOf(documentation, deprecated).mapNotNull { it.getOrNull() }
    }
}