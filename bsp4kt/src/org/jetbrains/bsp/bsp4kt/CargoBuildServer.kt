package org.jetbrains.bsp.bsp4kt

import org.jetbrains.jsonrpc4kt.services.JsonNotification
import org.jetbrains.jsonrpc4kt.services.JsonRequest
import kotlinx.serialization.SerialName

interface CargoBuildServer {
  @JsonRequest("workspace/cargoFeaturesState")
  fun suspend cargoFeaturesState(): CargoFeaturesStateResult

  @JsonRequest("workspace/enableCargoFeatures")
  fun suspend enableCargoFeatures(params: EnableCargoFeaturesParams): Unit

  @JsonRequest("workspace/disableCargoFeatures")
  fun suspend disableCargoFeatures(params: DisableCargoFeaturesParams): Unit

}
