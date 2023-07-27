package com.jetbrains.bsp.generators.ir

import software.amazon.smithy.model.Model
import software.amazon.smithy.model.shapes.*

class SmithyToIr(val model: Model) {

}

object ToDefVisitor : ShapeVisitor.Default<List<Def>>() {
    override fun getDefault(shape: Shape): List<Def> = emptyList()


}

object ToTypeVisitor : ShapeVisitor.Default<Type?>() {
    override fun getDefault(shape: Shape): Type? = null

    override fun booleanShape(shape: BooleanShape): Type = Type.Bool

    override fun integerShape(shape: IntegerShape): Type = Type.Int

    override fun longShape(shape: LongShape): Type = Type.Long

    override fun stringShape(shape: StringShape): Type = Type.String

    override fun documentShape(shape: DocumentShape): Type = Type.Json

    override fun listShape(shape: ListShape): Type? {
        return shape.member.accept(this)?.let { Type.List(it) }
    }






}