package com.jetbrains.bsp.generators

import software.amazon.smithy.model.Model

object ModelLoader {

    // By virtue of this module depending on the `spec` module, the models can be found on the classpath.
    val model: Model = Model.assembler().discoverModels().assemble().unwrap()
}
