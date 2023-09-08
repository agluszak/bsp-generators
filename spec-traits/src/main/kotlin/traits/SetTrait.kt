package traits

import software.amazon.smithy.model.node.ObjectNode
import software.amazon.smithy.model.shapes.ShapeId
import software.amazon.smithy.model.traits.AnnotationTrait

class SetTrait(node: ObjectNode) : AnnotationTrait(ID, node) {

    class Provider : AnnotationTrait.Provider<SetTrait>(ID, ::SetTrait)

    companion object {
        val ID: ShapeId = ShapeId.from("traits#set")
    }
}
